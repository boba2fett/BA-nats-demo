use std::sync::Arc;

use crate::{
    models::{RootDto, RootLinks, CreateJobDto, Token},
    util::{consts::{NAME, VERSION}, state::ServiceCollection},
};
use axum::{http::StatusCode, routing::post, extract::{State, Query, Path}, response::{IntoResponse, AppendHeaders}};
use axum::routing::get;
use axum::{Json, Router};
use chrono::Utc;
use common::{util::random, models::JobModel};
use reqwest::header;

pub fn create_route(services: Arc<ServiceCollection>) -> Router {
    Router::new()
        .route("/", get(root_links))
        .route("/health", get(health))
        .route("/job", post(create_job))
        .route("/job/:job_id", get(get_job))
        .with_state(services)
}

#[tracing::instrument]
pub async fn root_links() -> Json<RootDto> {
    Json(RootDto {
        version: VERSION,
        name: NAME,
        _links: RootLinks {
            publish: "/job"
        },
    })
}

#[tracing::instrument]
pub async fn health() -> StatusCode {
    StatusCode::OK
}


#[axum_macros::debug_handler]
pub async fn create_job(State(services): State<Arc<ServiceCollection>>, Json(content): Json<CreateJobDto>) -> impl IntoResponse {
    let id = random::generate_30_alphanumeric();
    let token = random::generate_30_alphanumeric();
    let location = common::util::routes::job_route(&id, &token);
    let job = JobModel {
        id: id.clone(),
        token,
        created: Utc::now(),
        status: common::models::JobStatus::Pending,
        message: None,
        payload: content.payload,
        callback_uri: content.callback_uri,
    };
    if let Err(err) = services.object_store_service.put(&job).await {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, err))
    }
    let headers = AppendHeaders([(header::LOCATION, location)]);
    match services.publish_service.publish(&id).await {
        Ok(_) => Ok((StatusCode::CREATED, headers, Json(job))),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err))
    }
}

#[axum_macros::debug_handler]
pub async fn get_job(State(services): State<Arc<ServiceCollection>>, Path(job_id): Path<String>, Query(token): Query<Token>) -> impl IntoResponse {
    let job = services.object_store_service.get(&job_id).await;
    if let Err(err) = job {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, err))
    }
    let job = job.unwrap();
    if job.is_none() {
        return Err((StatusCode::NOT_FOUND, ""))
    }
    let job = job.unwrap();
    if job.token != token.token {
        return Err((StatusCode::NOT_FOUND, ""))
    }
    let location = common::util::routes::job_route(&job.id, &job.token);
    let headers = AppendHeaders([(header::LOCATION, location)]);
    Ok((StatusCode::OK, headers, Json(job)))
}
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use chrono::serde::ts_seconds;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IdModel {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RefIdModel<'a> {
    pub id: &'a str,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JobModel {
    pub id: String,
    pub token: String,
    pub payload: String,
    #[serde(with = "ts_seconds")]
    pub created: DateTime<Utc>,
    pub status: JobStatus,
    pub message: Option<String>,
    pub callback_uri: Option<String>,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, Clone)]
#[repr(u8)]
pub enum JobStatus {
    Pending = 0,
    InProgress = 1,
    Finished = 2,
    Error = 3,
}
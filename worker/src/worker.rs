use std::{sync::Arc, time::Duration};

use common::{nats::{object_store::IObjectStoreService, subscribe::{IWorkerService, WorkError}}, models::{JobModel, JobStatus}};
use tokio::time::sleep;
use tracing::info;

pub struct WorkerService {
    pub object_store_service: Arc<dyn IObjectStoreService>,
    pub client: reqwest::Client,
}

#[async_trait::async_trait]
impl IWorkerService for WorkerService {
    #[tracing::instrument(skip(self))]
    async fn work(&self, job_id: &str) -> Result<(), WorkError> {
        let mut job = self.get_job(job_id).await.map_err(|_| WorkError::NoRetry)?;
        let mut job = self.set_in_progress(&mut job).await.map_err(|_| WorkError::Retry)?;

        self.work(&job).await?;

        _ = self.finish(&mut job).await.map_err(|_| WorkError::Retry)?;
        Ok(())
    }
}

impl WorkerService {
    async fn get_job(&self, job_id: &str) -> Result<JobModel, &'static str> {
        let job = self.object_store_service.get(job_id).await?.ok_or("Not found")?;
        Ok(job)
    }
    async fn set_in_progress<'a>(&self, job: &'a mut JobModel) -> Result<&'a mut JobModel, &'static str> {
        job.status = JobStatus::InProgress;
        self.object_store_service.put(job).await?;
        Ok(job)
    }
    async fn finish<'a>(&self, job: &'a mut JobModel) -> Result<&'a mut JobModel, &'static str> {
        job.status = JobStatus::Finished;
        self.object_store_service.put(job).await?;
        if let Some(callback_uri) = &job.callback_uri {
            _ = self.client.post(callback_uri).json::<JobModel>(&job).send().await;
        }
        Ok(job)
    }
    async fn work<'a>(&self, job: &'a JobModel) -> Result<(), WorkError> {
        info!("workin 1sec on job {}", &job.id);
        sleep(Duration::from_secs(1)).await;
        if job.payload == "retry" {
            return Err(WorkError::Retry)
        }
        if job.payload == "noretry" {
            return Err(WorkError::NoRetry)
        }
        Ok(())
    }
}
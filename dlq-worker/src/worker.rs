use std::sync::Arc;

use common::{nats::{object_store::IObjectStoreService, dlq_subscribe::IDLQWorkerService}, models::{JobModel, JobStatus}};
pub struct WorkerService {
    pub object_store_service: Arc<dyn IObjectStoreService>,
    pub client: reqwest::Client,
}

#[async_trait::async_trait]
impl IDLQWorkerService for WorkerService {
    #[tracing::instrument(skip(self))]
    async fn work(&self, job_id: &str) -> () {
        let job = self.get_job(job_id).await;
        if let Ok(mut job) = job {
            _ = self.error(&mut job, "max retries used or not processable".to_string()).await;
        }
    }
}

impl WorkerService {
    async fn get_job(&self, job_id: &str) -> Result<JobModel, &'static str> {
        let job = self.object_store_service.get(job_id).await?.ok_or("Not found")?;
        Ok(job)
    }
    async fn error<'a>(&self, job: &'a mut JobModel, err: String) -> Result<&'a mut JobModel, &'static str> {
        job.status = JobStatus::Error;
        job.message = Some(err);
        self.object_store_service.put(job).await?;
        if let Some(callback_uri) = &job.callback_uri {
            _ = self.client.post(callback_uri).json::<JobModel>(&job).send().await;
        }
        Ok(job)
    }
}
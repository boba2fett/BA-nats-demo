use std::{sync::Arc, time::Duration};

use common::{nats::{object_store::IObjectStoreService, subscribe::{IWorker, WorkError}}, models::{JobModel, JobStatus}};
use tokio::time::sleep;
use tracing::info;

pub struct WorkerService {
    pub object_store_service: Arc<dyn IObjectStoreService>,
    pub client: reqwest::Client,
}

#[async_trait::async_trait]
impl IWorker for WorkerService {
    #[tracing::instrument(skip(self))]
    async fn work(&self, job_id: &str) -> Result<(), WorkError> {
        let mut job = self.get_job(job_id).await.map_err(|_| WorkError::NoRetry)?;
        let mut job = self.set_in_progress(&mut job).await.map_err(|_| WorkError::Retry)?;

        self.work(&job).await.map_err(|_| WorkError::Retry)?;

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
        self.object_store_service.set(job).await?;
        Ok(job)
    }
    async fn finish<'a>(&self, job: &'a mut JobModel) -> Result<&'a mut JobModel, &'static str> {
        job.status = JobStatus::Finished;
        self.object_store_service.set(job).await?;
        if let Some(callback_uri) = &job.callback_uri {
            _ = self.client.post(callback_uri).json::<JobModel>(&job).send().await;
        }
        Ok(job)
    }
    async fn work<'a>(&self, job: &'a JobModel) -> Result<(), &'static str> {
        info!("workin 10secs on job {}", &job.id);
        sleep(Duration::from_secs(10)).await;
        Ok(())
    }
    // async fn error<'a>(&self, job: &'a mut JobModel, err: String) -> Result<&'a mut JobModel, &'static str> {
    //     job.status = JobStatus::Error;
    //     job.message = Some(err);
    //     self.object_store_service.set(job).await?;
    //     if let Some(callback_uri) = &job.callback_uri {
    //         self.client.post(callback_uri).json::<JobModel>(&job).send().await;
    //     }
    //     Ok(job)
    // }
}
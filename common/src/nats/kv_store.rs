use std::{sync::Arc, time::Duration};
use async_nats::jetstream::kv::{Store, Config};

use crate::models::JobModel;

use super::base::BaseJetStream;

#[async_trait::async_trait]
pub trait IKeyValueStoreService: Sync + Send {
    async fn put<'a>(&self, job: &'a JobModel) -> Result<(), &'static str>;
    async fn get<'a>(&self, id: &'a str) -> Result<Option<JobModel>, &'static str>;
}

pub struct KeyValueStoreService  {
    key_value: Store,
}

impl KeyValueStoreService {
    pub async fn build(base: Arc<BaseJetStream>, bucket: String, max_age: Duration) -> Result<Self, &'static str> {
        let key_value = base.jetstream.create_key_value(Config {
            bucket,
            max_age,
            ..Default::default()
        }).await.map_err(|_| "could not create key value store bucket")?;
        Ok(KeyValueStoreService {
            key_value
        })
    }
}

#[async_trait::async_trait]
impl IKeyValueStoreService for KeyValueStoreService {
    async fn put<'a>(&self, job: &'a JobModel) -> Result<(), &'static str>{
        let json = serde_json::to_string(&job).map_err(|_| "job is not valid json")?;
        self.key_value.put(job.id.as_str(), json.into()).await.map_err(|_| "could not put job")?;
        Ok(())
    }
    async fn get<'a>(&self, id: &'a str) -> Result<Option<JobModel>, &'static str>{
        let stream = self.key_value.get(id).await.map_err(|_| "could not get job")?;
        if stream.is_none() {
            return Ok(None);
        }
        let job = serde_json::from_slice(&stream.unwrap()).map_err(|_| "job is not valid json")?;
        Ok(Some(job))
    }
}
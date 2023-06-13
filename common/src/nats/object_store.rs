use std::{sync::Arc, time::Duration};
use async_nats::jetstream::object_store::{ObjectStore, Config};
use tokio::io::AsyncReadExt;

use crate::{models::JobModel, util::stream::VecReader};

use super::base::BaseJetstream;

#[async_trait::async_trait]
pub trait IObjectStoreService: Sync + Send {
    async fn set<'a>(&self, job: &'a JobModel) -> Result<(), &'static str>;
    async fn get<'a>(&self, id: &'a str) -> Result<Option<JobModel>, &'static str>;
}

pub struct ObjectStoreService  {
    store: ObjectStore,
}

impl ObjectStoreService {
    pub async fn build(base: Arc<BaseJetstream>, bucket: String, max_age: Duration) -> Result<Self, &'static str> {
        let store = base.jetstream.create_object_store(Config {
            bucket,
            max_age,
            ..Default::default()
        }).await.map_err(|_| "could not get object store")?;
        Ok(ObjectStoreService {
            store
        })
    }
}

#[async_trait::async_trait]
impl IObjectStoreService for ObjectStoreService {
    async fn set<'a>(&self, job: &'a JobModel) -> Result<(), &'static str>{
        let json = serde_json::to_vec(&job).map_err(|_| "not valid json")?;
        let mut stream = VecReader {
            vec: json
        };
        self.store.put(job.id.as_str(), &mut stream).await.map_err(|_| "not published")?;
        Ok(())
    }
    async fn get<'a>(&self, id: &'a str) -> Result<Option<JobModel>, &'static str>{
        let mut stream = self.store.get(id).await.map_err(|_| "not published")?;
        if stream.info.deleted {
            return Ok(None);
        };
        let mut json = Vec::<u8>::new();
        stream.read_to_end(&mut json).await.map_err(|_| "could not read")?;
        let job = serde_json::from_slice(&json).map_err(|_| "not valid json")?;
        Ok(Some(job))
    }
}
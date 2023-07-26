use std::{sync::Arc, time::Duration};

use common::nats::{dlq_subscribe::{IDLQSubscribeService, DLQSubscribeService}, kv_store::{KeyValueStoreService}, base::BaseJetStream};

use crate::worker::WorkerService;

pub struct ServiceCollection {
    pub subscribe_service: Arc<dyn IDLQSubscribeService>,
}

impl ServiceCollection {
    pub async fn build(nats_uri: &str, stream: String, bucket: String, consumer: String, max_age: Duration, max_age_mirror: Duration) -> Result<Arc<Self>, &'static str> {
        let base_jetstream = Arc::new(BaseJetStream::build(nats_uri).await?);
        let kv_store_service = Arc::new(KeyValueStoreService::build(base_jetstream.clone(), bucket, max_age).await?);
        let worker = WorkerService {
            kv_store_service,
            client: reqwest::Client::builder().danger_accept_invalid_certs(true).build().unwrap(),
        };
        Ok(Arc::new(ServiceCollection {
            subscribe_service: Arc::new(DLQSubscribeService::build(base_jetstream.clone(), stream, worker, consumer, max_age_mirror).await?),
        }))
    }
}

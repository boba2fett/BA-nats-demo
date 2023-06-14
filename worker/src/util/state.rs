use std::{sync::Arc, time::Duration};

use common::nats::{subscribe::{ISubscribeService, SubscribeService}, object_store::{ObjectStoreService}, base::BaseJetstream};

use crate::worker::WorkerService;

pub struct ServiceCollection {
    pub subscribe_service: Arc<dyn ISubscribeService>,
}

impl ServiceCollection {
    pub async fn build(nats_uri: &str, stream: String, bucket: String, consumer: String, max_age: Duration, max_deliver: i64, consumer_ack_wait: Duration) -> Result<Arc<Self>, &'static str> {
        let base_jetstream = Arc::new(BaseJetstream::build(nats_uri).await?);
        let object_store_service = Arc::new(ObjectStoreService::build(base_jetstream.clone(), bucket, max_age).await?);
        let worker = WorkerService {
            object_store_service,
            client: reqwest::Client::builder().danger_accept_invalid_certs(true).build().unwrap(),
        };
        Ok(Arc::new(ServiceCollection {
            subscribe_service: Arc::new(SubscribeService::build(base_jetstream.clone(), stream, worker, consumer, max_deliver, consumer_ack_wait).await?),
        }))
    }
}

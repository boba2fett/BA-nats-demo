use std::{sync::Arc, time::Duration};

use common::nats::{publish::{PublishService, IPublishService}, base::BaseJetStream, kv_store::{IKeyValueStoreService, KeyValueStoreService}};

pub struct ServiceCollection {
    pub publish_service: Arc<dyn IPublishService>,
    pub kv_store_service: Arc<dyn IKeyValueStoreService>,
}

impl ServiceCollection {
    pub async fn build(nats_uri: &str, stream: String, bucket: String, max_age: Duration) -> Result<Arc<Self>, &'static str> {
        let base_jetstream = Arc::new(BaseJetStream::build(nats_uri).await?);
        Ok(Arc::new(ServiceCollection{
            publish_service: Arc::new(PublishService::new(base_jetstream.clone(), stream)),
            kv_store_service: Arc::new(KeyValueStoreService::build(base_jetstream.clone(), bucket, max_age).await?)
        }))
    }
}

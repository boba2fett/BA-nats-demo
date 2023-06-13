use std::{sync::Arc, time::Duration};

use common::nats::{publish::{PublishService, IPublishService}, base::BaseJetstream, object_store::{IObjectStoreService, ObjectStoreService}};

pub struct ServiceCollection {
    pub publish_service: Arc<dyn IPublishService>,
    pub object_store_service: Arc<dyn IObjectStoreService>,
}

impl ServiceCollection {
    pub async fn build(nats_uri: &str, stream: String, bucket: String, max_age: Duration) -> Result<Arc<Self>, &'static str> {
        let base_jetstream = Arc::new(BaseJetstream::build(nats_uri).await?);
        Ok(Arc::new(ServiceCollection{
            publish_service: Arc::new(PublishService::build(base_jetstream.clone(), stream).await?),
            object_store_service: Arc::new(ObjectStoreService::build(base_jetstream.clone(), bucket, max_age).await?)
        }))
    }
}

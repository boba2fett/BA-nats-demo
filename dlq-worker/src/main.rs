use std::{env, time::Duration};
//https://docs.rs/async-nats/latest/async_nats/jetstream/stream/struct.Stream.html#method.direct_get
use dlq_worker::util::state::ServiceCollection;

#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::fmt().json().finish();
    tracing::subscriber::set_global_default(subscriber).expect("Could not init tracing.");

    let nats_uri = get_nats();
    let stream = get_stream();
    let consumer = get_consumer();
    let bucket = get_bucket();
    let max_age = get_max_age();

    let services = ServiceCollection::build(&nats_uri, stream, bucket, consumer, max_age).await.unwrap();

    services.subscribe_service.subscribe().await.unwrap();
}

fn get_nats() -> String {
    env::var("NATS_URI").unwrap_or_else(|_| "nats://localhost:4222".to_string())
}

fn get_stream() -> String {
    env::var("NATS_JETSTREAM_QUEUE").unwrap_or_else(|_| "newJob".to_string())
}

fn get_consumer() -> String {
    env::var("NATS_JETSTREAM_CONSUMER").unwrap_or_else(|_| "worker".to_string())
}

fn get_bucket() -> String {
    env::var("NATS_OBJECT_STORE_BUCKET").unwrap_or_else(|_| "job".to_string())
}

fn get_max_age() -> Duration {
    let max_age = env::var("MAX_AGE_SECONDS").map(|expire| expire.parse::<u64>());

    let max_age = match max_age {
        Ok(Ok(max_age)) => max_age,
        _ => 60 * 60 * 25,
    };
    Duration::from_secs(max_age)
}
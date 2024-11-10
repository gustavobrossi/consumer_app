use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::config::ClientConfig;
use rdkafka::message::BorrowedMessage;
use rdkafka::util::get_rdkafka_version;
use futures::StreamExt;
use rdkafka::Message;
use dotenv::dotenv;
use std::env;

async fn consume_message(message: &BorrowedMessage<'_>) {
    let payload = match message.payload_view::<str>() {
        Some(Ok(payload)) => payload,
        Some(Err(_)) => "<invalid utf-8>",
        None => "<null>",
    };
    println!(
        "key: '{:?}', payload: '{}', topic: {}, partition: {}, offset: {}",
        message.key(),
        payload,
        message.topic(),
        message.partition(),
        message.offset()
    );
}

#[tokio::main]
async fn main() {
    let (version_n, version_s) = get_rdkafka_version();
    println!("rd_kafka_version: 0x{:08x}, {}", version_n, version_s);

    dotenv().ok();

    let kafka_server_url = env::var("KAFKA_SERVER_URL").unwrap_or_else(|_| {
        println!("Warning: KAFKA_SERVER_URL is not set. Using localhost as default.");
        "localhost:9092".to_string()
    });

    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "example_consumer_group")
        .set("bootstrap.servers", kafka_server_url)
        .set("enable.partition.eof", "false")
        .set("auto.offset.reset", "earliest")
        .create()
        .expect("Consumer creation failed");

    consumer.subscribe(&["test_topic"])
        .expect("Can't subscribe to specified topics");

    let mut message_stream = consumer.stream();

    while let Some(message) = message_stream.next().await {
        match message {
            Ok(m) => consume_message(&m).await,
            Err(e) => eprintln!("Kafka error: {}", e),
        }
    }
}

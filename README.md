
# Consumer App

## Overview
The `consumer_app` is a simple Kafka consumer written in Rust. It listens to messages from a Kafka topic.

## Requirements
- Rust
- Docker & Docker Compose (optional to run Kafka Services)

## Setup Instructions

1. **Clone the Repository**

   ```sh
   git clone git@github.com:gustavobrossi/consumer_app.git
   cd consumer_app
   ```

2. **Build the Application**

   ```sh
   cargo build --release
   ```

3. **Run the Kafka Services**

   Make sure you have a Kafka Services running so the application can connect and consume from it. You will only need one server that both producer and consumer will connect into.
   For testing puropse, use the provided `docker-compose.yml` file bellow to set up Kafka.
   
   ```yaml
   version: '3'
   services:
     zookeeper:
       image: zookeeper
       ports:
         - "2181:2181"
     kafka:
       image: bitnami/kafka
       ports:
         - "9092:9092"
       environment:
         KAFKA_ADVERTISED_LISTENERS: PLAINTEXT://localhost:9092
         KAFKA_ZOOKEEPER_CONNECT: zookeeper:2181
   ```

   ```sh
   docker-compose up -d
   ```

4. **Run the Consumer Application**

   ```sh
   cargo run --release
   ```

## Code Explanation

- `main.rs`

  This file sets up a Kafka consumer and listens for messages from a topic.

  ```rust
  use rdkafka::consumer::{Consumer, StreamConsumer};
  use rdkafka::config::ClientConfig;
  use rdkafka::message::BorrowedMessage;
  use rdkafka::util::get_rdkafka_version;
  use futures::StreamExt;
  use rdkafka::Message;

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

      let consumer: StreamConsumer = ClientConfig::new()
          .set("group.id", "example_consumer_group")
          .set("bootstrap.servers", "localhost:9092")
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
  ```

- `cargo.toml`

  ```toml
  [package]
  name = "consumer_app"
  version = "0.1.0"
  edition = "2021"

  [dependencies]
  rdkafka = "0.26"
  tokio = { version = "1", features = ["full"] }
  futures = "0.3"
  ```

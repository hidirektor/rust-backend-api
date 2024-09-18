use lapin::{Connection, ConnectionProperties};

pub async fn connect(rabbitmq_url: &str) -> Connection {
    Connection::connect(rabbitmq_url, ConnectionProperties::default())
        .await
        .expect("Failed to connect to RabbitMQ")
}
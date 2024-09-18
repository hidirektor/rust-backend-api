use redis::Client;

pub fn connect(redis_url: &str) -> Client {
    Client::open(redis_url).expect("Failed to connect to Redis")
}
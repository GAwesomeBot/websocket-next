use redis::{RedisResult, Client};

#[derive(Debug, Clone)]
pub struct RedisClient {
    client: Client,
}

impl RedisClient {
    pub fn new(url: String) -> RedisResult<Self> {
        Ok(Self {
            client: Client::open(url)?
        })
    }
}

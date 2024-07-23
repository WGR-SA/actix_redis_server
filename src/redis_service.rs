use actix::prelude::*;
use redis::AsyncCommands;
use std::sync::Arc;

pub struct RedisActor {
    client: Arc<redis::Client>,
}

impl RedisActor {
    pub fn new(redis_url: &str) -> Self {
        let client = redis::Client::open(redis_url).expect("Invalid Redis URL");
        RedisActor {
            client: Arc::new(client),
        }
    }
}

impl Actor for RedisActor {
    type Context = Context<Self>;
}

pub struct GetRedisValue {
    pub key: String,
}

impl Message for GetRedisValue {
    type Result = Result<Vec<u8>, redis::RedisError>;
}

impl Handler<GetRedisValue> for RedisActor {
    type Result = ResponseFuture<Result<Vec<u8>, redis::RedisError>>;

    fn handle(&mut self, msg: GetRedisValue, _: &mut Self::Context) -> Self::Result {
        let client = Arc::clone(&self.client);
        let key = msg.key;
        Box::pin(async move {
            let mut con = client.get_async_connection().await?;
            let data: Vec<u8> = con.get(key).await?;
            Ok(data)
        })
    }
}

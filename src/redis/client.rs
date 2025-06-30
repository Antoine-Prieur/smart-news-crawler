use redis::{Client, Commands, RedisResult};
use serde_json::Value;

use log::info;

#[derive(Debug, Clone)]
pub enum QueueName {
    Articles,
}

impl QueueName {
    pub fn as_str(&self) -> &'static str {
        match self {
            QueueName::Articles => "articles",
        }
    }
}

pub struct RedisQueueClient {
    redis_client: Client,
}

impl RedisQueueClient {
    pub fn new(redis_url: &str) -> RedisResult<Self> {
        let client = Client::open(redis_url)?;
        Ok(RedisQueueClient {
            redis_client: client,
        })
    }

    pub fn setup_queues(&self) -> RedisResult<()> {
        let mut con = self.redis_client.get_connection()?;

        let queues = [QueueName::Articles];

        for queue in &queues {
            let _: () = con.hset(
                "queue_metadata",
                queue.as_str(),
                "created_by_smart_news_crawler",
            )?;
        }

        info!("All queues initialized successfully");
        Ok(())
    }

    pub fn enqueue(&self, queue: QueueName, message: Value) -> RedisResult<()> {
        let mut con = self.redis_client.get_connection()?;
        let _: () = con.lpush(queue.as_str(), message.to_string())?;
        Ok(())
    }
}

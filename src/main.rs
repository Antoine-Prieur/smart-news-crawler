mod config;
mod crawler;
mod database;
mod redis;
mod serializers;

use crate::config::Config;
use crate::crawler::crawl;

use log::error;

use self::database::ArticleRepository;
use self::redis::client::RedisQueueClient;

#[tokio::main]
async fn main() {
    env_logger::init();

    let config = match Config::new() {
        Ok(config) => config,
        Err(e) => {
            error!("Failed to load configuration: {}", e);
            std::process::exit(1);
        }
    };

    let redis_client = match RedisQueueClient::new(&config.redis_url) {
        Ok(client) => client,
        Err(e) => {
            error!("Failed to connect to Redis: {}", e);
            std::process::exit(1);
        }
    };
    if let Err(e) = redis_client.setup_queues() {
        error!("Failed to setup Redis queues: {}", e);
        std::process::exit(1);
    }

    let repository = match ArticleRepository::new(
        &config.mongodb_connection_string,
        &config.mongodb_database_name,
        &config.mongodb_collection_name,
    )
    .await
    {
        Ok(repo) => repo,
        Err(e) => {
            error!("Failed to connect to MongoDB: {}", e);
            std::process::exit(1);
        }
    };

    if let Err(e) = crawl(&config, repository, redis_client).await {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

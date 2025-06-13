mod config;
mod crawler;

use crate::config::Config;
use crate::crawler::crawl;

use log::error;

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

    if let Err(e) = crawl(&config).await {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

mod config;
mod news_model;

use crate::config::API_NEWS_BASE_URL;
use crate::config::API_NEWS_ENDPOINT;
use crate::config::API_NEWS_MAX_CALLS_PER_DAY;
use crate::config::get_api_news_secret;

use crate::news_model::NewsResponse;
use log::error;
use log::info;
use std::fs::File;
use std::io::Write;
use tokio::time::{Duration, sleep};
use url::Url;

use self::config::API_NEWS_COUNTRY_CODE;
use self::news_model::Article;

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let base_url: Url = Url::parse(API_NEWS_BASE_URL)?;
    let top_headlines_url: Url = base_url.join(API_NEWS_ENDPOINT)?;
    let top_headlines_url_string = top_headlines_url.to_string();

    let api_news_api_key = get_api_news_secret()?;

    let client = reqwest::Client::builder()
        .user_agent(format!(
            "{}/{}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        ))
        .build()?;

    let mut all_articles: Vec<Article> = Vec::new();

    let mut call_count = 0;
    let mut page = 1;
    let page_size = 100;

    let max_retries = 3;
    let mut retry_count = 0;
    let base_retry_delay = Duration::from_millis(1000);

    while call_count < API_NEWS_MAX_CALLS_PER_DAY {
        info!(
            "Call {}/{} - Crawling...",
            call_count + 1,
            API_NEWS_MAX_CALLS_PER_DAY,
        );

        let res = client
            .get(&top_headlines_url_string)
            .query(&[("country", API_NEWS_COUNTRY_CODE)])
            .query(&[("pageSize", page_size)])
            .query(&[("page", page)])
            .header("X-Api-Key", &api_news_api_key)
            .send()
            .await?;

        if !res.status().is_success() {
            error!("HTTP Error: {}", res.status());
            let error_text = res.text().await?;
            error!("Error response: {}", error_text);

            if retry_count == max_retries {
                break;
            }

            let delay = base_retry_delay * 2_u32.pow(retry_count);
            retry_count += 1;

            sleep(delay).await;
            continue;
        }

        retry_count = 0;

        let news_response: NewsResponse = res.json().await?;
        info!(
            "Status: {}, Total Results: {}",
            news_response.status, news_response.total_results
        );

        let articles_count = news_response.articles.len();
        all_articles.extend(news_response.articles);

        if articles_count < page_size || all_articles.len() >= news_response.total_results as usize
        {
            info!(
                "Retrieved all available articles. Total: {}",
                all_articles.len()
            );
            break;
        }

        page += 1;
        call_count += 1;
        sleep(Duration::from_millis(100)).await;
    }

    if call_count == API_NEWS_MAX_CALLS_PER_DAY {
        info!("Reached {} calls limit", API_NEWS_MAX_CALLS_PER_DAY);
    }

    if !all_articles.is_empty() {
        let json_data = serde_json::to_string_pretty(&all_articles)?;
        let filename = format!(
            "articles_{}.json",
            chrono::Utc::now().format("%Y%m%d_%H%M%S")
        );

        let mut file = File::create(&filename)?;
        file.write_all(json_data.as_bytes())?;

        info!("Saved {} articles to {}", all_articles.len(), filename);
    } else {
        info!("No articles to save");
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    env_logger::init();

    if let Err(e) = run().await {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

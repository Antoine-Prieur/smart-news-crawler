use crate::config::Config;
use crate::crawler::models::Article;
use crate::crawler::models::NewsResponse;
use crate::database::ArticleDocument;
use crate::database::ArticleRepository;
use crate::redis::client::QueueName;
use crate::redis::client::RedisQueueClient;

use log::error;
use log::info;
use tokio::time::{Duration, sleep};
use url::Url;

struct NewsCrawlerClient {
    client: reqwest::Client,
    call_count: i32,
    config: Config,
}

impl NewsCrawlerClient {
    fn new(config: Config) -> Result<Self, Box<dyn std::error::Error>> {
        let client = reqwest::Client::builder()
            .user_agent(format!(
                "{}/{}",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION")
            ))
            .build()?;

        Ok(Self {
            client,
            call_count: 0,
            config,
        })
    }

    async fn fetch_news_page(
        &mut self,
        url: &str,
        page: usize,
        page_size: usize,
    ) -> Result<NewsResponse, Box<dyn std::error::Error>> {
        let mut retry_count: u32 = 0;
        let base_retry_delay = Duration::from_millis(1000);

        while self.call_count < self.config.api_news_max_calls_per_day {
            info!(
                "Call {}/{} - Crawling...",
                self.call_count + 1,
                self.config.api_news_max_calls_per_day,
            );

            let res = self
                .client
                .get(url)
                .query(&[("country", self.config.api_news_country_code)])
                .query(&[("pageSize", page_size)])
                .query(&[("page", page)])
                .header("X-Api-Key", self.config.get_api_news_secret())
                .send()
                .await?;

            self.call_count += 1;

            if res.status().is_success() {
                let news_response: NewsResponse = res.json().await?;
                return Ok(news_response);
            }

            error!("HTTP Error: {}", res.status());
            let error_text = res.text().await?;
            error!("Error response: {}", error_text);

            if retry_count >= self.config.api_news_max_retry_count {
                break;
            }

            let delay = base_retry_delay * 2_u32.pow(retry_count);
            retry_count += 1;

            sleep(delay).await;
        }
        Err("Max retries exceeded or API limit reached".into())
    }

    async fn save_and_enqueue_articles(
        &self,
        articles: &[Article],
        repository: &ArticleRepository,
        redis_client: &RedisQueueClient,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if articles.is_empty() {
            info!("No articles to process");
            return Ok(());
        }
        let article_documents = ArticleDocument::from_articles(articles.to_vec());

        let inserted_articles_document = repository.insert_articles(&article_documents).await?;
        info!("Saved {} articles to database", articles.len());

        for article_doc in &inserted_articles_document {
            let article_json = serde_json::to_value(article_doc)?;
            redis_client.enqueue(QueueName::Articles, article_json)?;
        }
        info!("Enqueued {} articles to Redis", articles.len());

        Ok(())
    }

    pub async fn crawl_all_articles(
        &mut self,
        base_url: &str,
        endpoint: &str,
        article_repository: &ArticleRepository,
        redis_client: &RedisQueueClient,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let base_url: Url = Url::parse(base_url)?;
        let top_headlines_url: Url = base_url.join(endpoint)?;
        let top_headlines_url_string = top_headlines_url.to_string();

        let mut all_articles: Vec<Article> = Vec::new();
        let mut page: usize = 1;
        let page_size: usize = 100;

        while self.call_count < self.config.api_news_max_calls_per_day {
            match self
                .fetch_news_page(&top_headlines_url_string, page, page_size)
                .await
            {
                Ok(news_response) => {
                    info!(
                        "Status: {}, Total Results: {}",
                        news_response.status, news_response.total_results
                    );

                    let articles_count = news_response.articles.len();
                    all_articles.extend(news_response.articles);

                    if articles_count < page_size
                        || all_articles.len() >= news_response.total_results as usize
                    {
                        info!(
                            "Retrieved all available articles. Total: {}",
                            all_articles.len()
                        );
                        break;
                    }

                    page += 1;
                    sleep(Duration::from_millis(100)).await;
                }
                Err(e) => {
                    error!("Failed to fetch news page: {}", e);
                    break;
                }
            }
        }

        self.save_and_enqueue_articles(&all_articles, article_repository, redis_client)
            .await?;

        Ok(())
    }
}

pub async fn crawl(
    config: &Config,
    article_repository: ArticleRepository,
    redis_client: RedisQueueClient,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = NewsCrawlerClient::new(config.clone())?;
    client
        .crawl_all_articles(
            config.api_news_base_url,
            config.api_news_endpoint,
            &article_repository,
            &redis_client,
        )
        .await
}

use std::env;

#[derive(Clone)]
pub struct Config {
    pub api_news_base_url: &'static str,
    pub api_news_endpoint: &'static str,
    pub api_news_max_calls_per_day: i32,
    pub api_news_country_code: &'static str,
    pub api_news_max_retry_count: u32,
    api_news_secret: String,

    pub mongodb_connection_string: String,
    pub mongodb_database_name: String,
    pub mongodb_collection_name: String,
}

impl Config {
    pub fn new() -> Result<Self, env::VarError> {
        Ok(Self {
            api_news_base_url: "https://newsapi.org/v2/",
            api_news_endpoint: "top-headlines",
            api_news_max_calls_per_day: 100,
            api_news_country_code: "us", // seems that newsapi only returns results for US
            api_news_max_retry_count: 3,
            api_news_secret: env::var("API_NEWS_SECRET")?,

            mongodb_connection_string: env::var("MONGO_URL")
                .unwrap_or_else(|_| "mongodb://admin:password123@localhost:27017".to_string()),
            mongodb_database_name: env::var("MONGODB_DATABASE_NAME")
                .unwrap_or_else(|_| "news".to_string()),
            mongodb_collection_name: env::var("MONGODB_COLLECTION_NAME")
                .unwrap_or_else(|_| "articles".to_string()),
        })
    }

    pub fn get_api_news_secret(&self) -> &str {
        &self.api_news_secret
    }
}

use std::env;

#[derive(Clone)]
pub struct Config {
    pub api_news_base_url: &'static str,
    pub api_news_endpoint: &'static str,
    pub api_news_max_calls_per_day: i32,
    pub api_news_country_code: &'static str,
    pub api_news_max_retry_count: u32,
    api_news_secret: String, // private field
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
        })
    }

    pub fn get_api_news_secret(&self) -> &str {
        &self.api_news_secret
    }
}

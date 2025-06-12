use std::env;

pub const API_NEWS_BASE_URL: &str = "https://newsapi.org/v2/";
pub const API_NEWS_ENDPOINT: &str = "top-headlines";
pub const API_NEWS_MAX_CALLS_PER_DAY: i32 = 100;
pub const API_NEWS_COUNTRY_CODE: &str = "us"; // seems that newsapi only returns results for US

pub fn get_api_news_secret() -> Result<String, env::VarError> {
    env::var("API_NEWS_SECRET")
}

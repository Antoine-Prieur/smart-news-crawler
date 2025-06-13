use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct NewsResponse {
    pub status: String,
    #[serde(rename = "totalResults")]
    pub total_results: u32,
    pub articles: Vec<Article>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Source {
    id: Option<String>,
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Article {
    source: Source,
    author: Option<String>,
    title: Option<String>,
    description: Option<String>,
    url: Option<String>,
    #[serde(rename = "urlToImage")]
    url_to_image: Option<String>,
    #[serde(rename = "publishedAt")]
    published_at: Option<String>,
    content: Option<String>,
}

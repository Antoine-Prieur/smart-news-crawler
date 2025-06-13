use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct NewsResponse {
    pub status: String,
    #[serde(rename = "totalResults")]
    pub total_results: u32,
    pub articles: Vec<Article>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Source {
    pub id: Option<String>,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Article {
    pub source: Source,
    pub author: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    #[serde(rename = "urlToImage")]
    pub url_to_image: Option<String>,
    #[serde(rename = "publishedAt")]
    pub published_at: Option<String>,
    pub content: Option<String>,
}

use crate::serializers::{deserialize_datetime, serialize_mongo_date};
use chrono::{DateTime, Utc};
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
    #[serde(
        rename = "publishedAt",
        deserialize_with = "deserialize_datetime",
        serialize_with = "serialize_mongo_date"
    )]
    pub published_at: DateTime<Utc>,
    pub content: Option<String>,
}

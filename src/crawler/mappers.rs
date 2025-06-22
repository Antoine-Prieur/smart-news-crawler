use chrono::Utc;

use crate::crawler::models::Article;
use crate::database::ArticleDocument;
use crate::database::models::SourceDocument;

impl From<Article> for ArticleDocument {
    fn from(article: Article) -> Self {
        let now = Utc::now();

        Self {
            source: SourceDocument {
                id: article.source.id,
                name: article.source.name,
            },
            author: article.author,
            title: article.title,
            description: article.description,
            url: article.url,
            url_to_image: article.url_to_image,
            published_at: article.published_at,
            content: article.content,
            created_at: now,
            updated_at: now,
        }
    }
}

impl ArticleDocument {
    pub fn from_articles(articles: Vec<Article>) -> Vec<ArticleDocument> {
        articles.into_iter().map(ArticleDocument::from).collect()
    }
}

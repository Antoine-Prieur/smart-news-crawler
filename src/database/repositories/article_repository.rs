use crate::database::models::ArticleDocument;
use log::{error, info};
use mongodb::options::IndexOptions;
use mongodb::{Client, Collection, Database, IndexModel};

pub struct ArticleRepository {
    collection: Collection<ArticleDocument>,
}

impl ArticleRepository {
    pub async fn new(
        connection_string: &str,
        database_name: &str,
        collection_name: &str,
    ) -> Result<Self, mongodb::error::Error> {
        let client = Client::with_uri_str(connection_string).await?;

        let database: Database = client.database(database_name);

        let collection: Collection<ArticleDocument> = database.collection(collection_name);

        info!(
            "Connected to MongoDB database: {}, collection: {}",
            database_name, collection_name
        );

        let repository = Self { collection };

        repository.create_url_index().await?;

        Ok(repository)
    }

    async fn create_url_index(&self) -> Result<(), mongodb::error::Error> {
        let index_doc = mongodb::bson::doc! {
            "url": 1
        };

        let index_options = IndexOptions::builder().unique(true).sparse(true).build();

        let index_model = IndexModel::builder()
            .keys(index_doc)
            .options(index_options)
            .build();

        match self.collection.create_index(index_model).await {
            Ok(result) => {
                info!(
                    "Successfully created unique index on 'url' field: {:?}",
                    result
                );
                Ok(())
            }
            Err(e) => {
                if e.to_string().contains("already exists") {
                    info!("Unique index on 'url' field already exists, skipping creation");
                    Ok(())
                } else {
                    error!("Failed to create unique index on 'url' field: {}", e);
                    Err(e)
                }
            }
        }
    }

    pub async fn insert_article(
        &self,
        article: &ArticleDocument,
    ) -> Result<Option<ArticleDocument>, mongodb::error::Error> {
        let mut article_with_id = article.clone();

        match self.collection.insert_one(&article_with_id).await {
            Ok(result) => {
                if let mongodb::bson::Bson::ObjectId(oid) = result.inserted_id {
                    article_with_id.id = Some(oid);
                    info!("Successfully inserted article with ID: {}", oid);
                    Ok(Some(article_with_id))
                } else {
                    error!("Unexpected ID type from MongoDB insert");
                    Ok(None)
                }
            }
            Err(e) => {
                if e.to_string().contains("duplicate key") || e.to_string().contains("E11000") {
                    info!(
                        "Article skipped due to duplicate URL (this is expected): {}",
                        article.url.as_deref().unwrap_or("unknown URL")
                    );
                    Ok(None)
                } else {
                    error!("Failed to insert article: {}", e);
                    Err(e)
                }
            }
        }
    }

    pub async fn insert_articles(
        &self,
        articles: &[ArticleDocument],
    ) -> Result<Vec<ArticleDocument>, mongodb::error::Error> {
        if articles.is_empty() {
            info!("No articles to insert");
            return Ok(Vec::new());
        }

        let mut successfully_inserted = Vec::new();
        let mut duplicate_count = 0;
        let mut error_count = 0;

        info!("Processing {} articles for insertion...", articles.len());

        for (index, article) in articles.iter().enumerate() {
            match self.insert_article(article).await {
                Ok(Some(inserted_article)) => {
                    successfully_inserted.push(inserted_article);
                }
                Ok(_none) => {
                    duplicate_count += 1;
                }
                Err(e) => {
                    error!(
                        "Failed to insert article {}/{}: {}",
                        index + 1,
                        articles.len(),
                        e
                    );
                    error_count += 1;
                }
            }
        }

        info!(
            "Insertion complete. Successfully inserted: {}, Duplicates skipped: {}, Errors: {}",
            successfully_inserted.len(),
            duplicate_count,
            error_count
        );

        Ok(successfully_inserted)
    }
}

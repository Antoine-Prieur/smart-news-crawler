use crate::database::DatabaseRepository;
use crate::database::models::ArticleDocument;
use async_trait::async_trait;
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

    pub async fn insert_articles(
        &self,
        articles: &[ArticleDocument],
    ) -> Result<(), mongodb::error::Error> {
        if articles.is_empty() {
            info!("No articles to insert");
            return Ok(());
        }

        self.insert_many(articles).await
    }
}

#[async_trait]
impl DatabaseRepository<ArticleDocument> for ArticleRepository {
    type Error = mongodb::error::Error;

    async fn insert_many(&self, documents: &[ArticleDocument]) -> Result<(), Self::Error> {
        if documents.is_empty() {
            info!("No documents to insert");
            return Ok(());
        }

        let documents_vec = documents.to_vec();

        match self.collection.insert_many(documents_vec).await {
            Ok(result) => {
                info!(
                    "Successfully inserted {} article documents",
                    result.inserted_ids.len()
                );
                Ok(())
            }
            Err(e) => {
                if e.to_string().contains("duplicate key") || e.to_string().contains("E11000") {
                    info!(
                        "Some articles were skipped due to duplicate URLs (this is expected behavior)"
                    );

                    Ok(())
                } else {
                    error!("Failed to insert article documents: {}", e);
                    Err(e)
                }
            }
        }
    }
}

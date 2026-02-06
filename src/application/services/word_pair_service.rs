use heck::ToTitleCase;
use thiserror::Error;
use tracing::error;

use crate::{
    api::models::word_pair::CreateWordPairDTO,
    domain::{
        models::word_pair::WordPair,
        traits::repositories::{repository::Repository, word_pair_repository::IWordPairRepository},
        types::ID,
    },
    infrastructure::storage::database::models::word_pair::WordPairEntity,
};

#[derive(Clone)]
pub struct WordPairService<Repo> {
    repo: Repo,
}

#[derive(Debug, Error)]
pub enum WordPairServiceError {
    #[error("This word pair already exists: {0}")]
    WordPairAlreadyExists(String),

    #[error("Word pair not found: `{0}`")]
    NotFound(String),

    #[error("Database error: `{0}`")]
    Database(#[from] sqlx::Error),

    #[error("Unknown error")]
    Unknown,
}

impl<Repo> WordPairService<Repo>
where
    Repo: Repository<Item = WordPair, Entity = WordPairEntity, Error = sqlx::Error>,
{
    pub fn new(repo: Repo) -> Self {
        Self { repo: repo }
    }

    pub async fn create(
        &self,
        user_id: &ID,
        params: &CreateWordPairDTO,
    ) -> Result<WordPair, WordPairServiceError> {
        let word_pair = WordPair::new_simple(
            *user_id,
            params.target_text.to_title_case(),
            params.source_text.clone(),
            params.target_language.clone(),
            params.source_language.clone(),
        );

        let res = self.repo.insert(&word_pair).await.map_err(|error| {
            error!("WordPair DB error: {}", error);
            error
        })?;

        Ok(res)
    }

    pub async fn get_by_id(&self, id: &ID) -> Result<WordPair, WordPairServiceError> {
        let res = self
            .repo
            .select_by_id(&id)
            .await
            .map_err(|error| match &error {
                sqlx::Error::RowNotFound => WordPairServiceError::NotFound(error.to_string()),
                _ => {
                    error!("User DB error: {}", error);
                    WordPairServiceError::Database(error.into())
                }
            })?;

        Ok(res)
    }

    pub async fn delete_by_id(&self, id: &ID) -> Result<(), WordPairServiceError> {
        self.repo
            .delete_by_id(&id)
            .await
            .map_err(|error| match &error {
                sqlx::Error::RowNotFound => WordPairServiceError::NotFound(error.to_string()),
                _ => {
                    error!("User DB error: {}", error);
                    WordPairServiceError::Database(error.into())
                }
            })?;

        Ok(())
    }
}

impl<Repo> WordPairService<Repo>
where
    Repo: IWordPairRepository<Error = sqlx::Error>,
{
    pub async fn get_by_user_id(
        &self,
        user_id: &ID,
    ) -> Result<Vec<WordPair>, WordPairServiceError> {
        let res = self
            .repo
            .select_by_user_id(&user_id)
            .await
            .map_err(|error| match &error {
                sqlx::Error::RowNotFound => WordPairServiceError::NotFound(error.to_string()),
                _ => {
                    error!("User DB error: {}", error);
                    WordPairServiceError::Database(error.into())
                }
            })?;

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use chrono::Utc;
    use uuid::Uuid;

    use crate::infrastructure::storage::database::models::word_pair::WordPairEntity;

    use super::*;

    struct TestWordPairRepository {
        _db: i32,
    }

    #[async_trait]
    impl Repository for TestWordPairRepository {
        type Pool = i32;
        type Item = WordPair;
        type Entity = WordPairEntity;
        type Error = sqlx::Error;

        fn new(db: i32) -> Self {
            Self { _db: db }
        }

        async fn insert(&self, item: &Self::Item) -> Result<Self::Item, Self::Error> {
            Ok(item.clone())
        }

        async fn select_by_id(&self, id: &ID) -> Result<Self::Item, Self::Error> {
            let test_word_pair = WordPair {
                id: *id,
                user_id: Uuid::new_v4(),
                target_text: "Hallo".to_string(),
                source_text: "Hello".to_string(),
                target_language: "de".to_string(),
                source_language: "en".to_string(),
                created_at: Utc::now(),
            };

            Ok(test_word_pair)
        }

        async fn delete_by_id(&self, id: &ID) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    #[async_trait]
    impl IWordPairRepository for TestWordPairRepository {
        async fn select_by_user_id(&self, user_id: &ID) -> Result<Vec<Self::Item>, Self::Error> {
            Ok(vec![WordPair {
                id: Uuid::new_v4(),
                user_id: *user_id,
                target_text: "Hallo".to_string(),
                source_text: "Hello".to_string(),
                target_language: "de".to_string(),
                source_language: "en".to_string(),
                created_at: Utc::now(),
            }])
        }
    }

    #[tokio::test]
    async fn test_create() {
        let repo = TestWordPairRepository { _db: 12345 };

        let word_pair_service = WordPairService::new(repo);

        let test_user_id = Uuid::new_v4();
        let test_params = CreateWordPairDTO {
            target_text: "Hallo".to_string(),
            source_text: "Hello".to_string(),
            target_language: "de".to_string(),
            source_language: "en".to_string(),
        };

        let res = word_pair_service
            .create(&test_user_id, &test_params)
            .await
            .unwrap();

        assert_eq!(res.user_id, test_user_id);
    }

    #[tokio::test]
    async fn test_format() {
        let repo = TestWordPairRepository { _db: 12345 };

        let word_pair_service = WordPairService::new(repo);

        let test_user_id = Uuid::new_v4();
        let test_params = CreateWordPairDTO {
            target_text: "Hallo".to_string(),
            source_text: "Hello".to_string(),
            target_language: "de".to_string(),
            source_language: "en".to_string(),
        };

        let res = word_pair_service
            .create(&test_user_id, &test_params)
            .await
            .unwrap();

        assert_eq!(res.source_text, "Hello");
        assert_eq!(res.target_text, "Hallo".to_string());
    }

    #[tokio::test]
    async fn test_get_by_user_id() {
        let repo = TestWordPairRepository { _db: 12345 };

        let word_pair_service = WordPairService::new(repo);

        let test_user_id = Uuid::new_v4();

        let res = word_pair_service
            .get_by_user_id(&test_user_id)
            .await
            .unwrap();

        let val = vec![WordPair {
            id: Uuid::new_v4(),
            user_id: test_user_id,
            target_text: "Hallo".to_string(),
            source_text: "Hello".to_string(),
            target_language: "de".to_string(),
            source_language: "en".to_string(),
            created_at: Utc::now(),
        }];

        assert_eq!(res[0].user_id, val[0].user_id);
    }
}

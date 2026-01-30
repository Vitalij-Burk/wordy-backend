use heck::ToTitleCase;
use thiserror::Error;
use tracing::error;

use crate::{
    api::models::word_pair::CreateWordPairDTO,
    domain::{
        models::word_pair::WordPair,
        traits::repositories::{repository::Repository, word_pair_repository::IWordPairRepository},
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
    Repo: Repository<Item = WordPairEntity, Error = sqlx::Error>,
{
    pub fn new(repo: Repo) -> Self {
        Self { repo: repo }
    }

    pub async fn create(
        &self,
        user_id: &i32,
        params: &CreateWordPairDTO,
    ) -> Result<WordPair, WordPairServiceError> {
        let word_pair = WordPair::new(
            &user_id,
            &params.target_text.to_title_case(),
            &params.source_text,
            &params.target_language,
            &params.source_language,
        );

        let res = self
            .repo
            .insert(&WordPairEntity::from(word_pair))
            .await
            .map_err(|error| {
                error!("WordPair DB error: {}", error);
                error
            })?;

        let word_pair = WordPair::from(res);

        Ok(word_pair)
    }

    pub async fn get_by_id(&self, id: &i32) -> Result<WordPair, WordPairServiceError> {
        let res = self.repo.select_by_id(&id).await.map_err(|error| {
            error!("WordPair DB error: {}", error);
            error
        })?;

        let word_pair = WordPair::from(res);

        Ok(word_pair)
    }

    pub async fn delete_by_id(&self, id: &i32) -> Result<(), WordPairServiceError> {
        self.repo.delete_by_id(&id).await.map_err(|error| {
            error!("WordPair DB error: {}", error);
            error
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
        user_id: &i32,
    ) -> Result<Vec<WordPair>, WordPairServiceError> {
        let res = self
            .repo
            .select_by_user_id(&user_id)
            .await
            .map_err(|error| {
                error!("WordPair DB error: {}", error);
                error
            })?;
        let mut word_pairs: Vec<WordPair> = vec![];

        for res_item in res.into_iter() {
            word_pairs.push(WordPair::from(res_item))
        }

        Ok(word_pairs)
    }
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;

    use crate::infrastructure::storage::database::models::word_pair::WordPairEntity;

    use super::*;

    struct TestWordPairRepository {
        _db: i32,
    }

    #[async_trait]
    impl Repository for TestWordPairRepository {
        type Pool = i32;
        type Item = WordPairEntity;
        type Error = sqlx::Error;

        fn new(db: i32) -> Self {
            Self { _db: db }
        }

        async fn insert(&self, item: &Self::Item) -> Result<Self::Item, Self::Error> {
            Ok(item.clone())
        }

        async fn select_by_id(&self, id: &i32) -> Result<Self::Item, Self::Error> {
            let test_word_pair = WordPairEntity {
                id: *id,
                user_id: 13432324,
                target_text: "Hallo".to_string(),
                source_text: "Hello".to_string(),
                target_language: "de".to_string(),
                source_language: "en".to_string(),
            };

            Ok(test_word_pair)
        }

        async fn delete_by_id(&self, id: &i32) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    #[async_trait]
    impl IWordPairRepository for TestWordPairRepository {
        async fn select_by_user_id(&self, user_id: &i32) -> Result<Vec<Self::Item>, Self::Error> {
            Ok(vec![WordPairEntity {
                id: 1234,
                user_id: *user_id,
                target_text: "Hallo".to_string(),
                source_text: "Hello".to_string(),
                target_language: "de".to_string(),
                source_language: "en".to_string(),
            }])
        }
    }

    #[tokio::test]
    async fn test_create() {
        let repo = TestWordPairRepository { _db: 12345 };

        let word_pair_service = WordPairService::new(repo);

        let test_user_id = 1234567;
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

        let test_user_id = 1234567;
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

        let test_user_id = 1234567;

        let res = word_pair_service
            .get_by_user_id(&test_user_id)
            .await
            .unwrap();

        let val = vec![WordPair {
            id: 1234,
            user_id: test_user_id,
            target_text: "Hallo".to_string(),
            source_text: "Hello".to_string(),
            target_language: "de".to_string(),
            source_language: "en".to_string(),
        }];

        assert_eq!(res[0].user_id, val[0].user_id);
        assert_eq!(res[0].id, val[0].id);
    }
}

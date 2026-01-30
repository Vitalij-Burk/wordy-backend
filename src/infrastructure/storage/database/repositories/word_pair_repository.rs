use crate::{
    domain::traits::repositories::{
        repository::Repository, word_pair_repository::IWordPairRepository,
    },
    infrastructure::storage::database::models::word_pair::WordPairEntity,
};
use async_trait::async_trait;
use sqlx::{Error, postgres::PgPool};

#[derive(Clone)]
pub struct WordPairPostgresRepository {
    db: PgPool,
}

#[async_trait]
impl Repository for WordPairPostgresRepository {
    type Pool = PgPool;
    type Item = WordPairEntity;
    type Error = Error;

    fn new(db: Self::Pool) -> Self {
        Self { db: db }
    }

    async fn insert(&self, word_pair: &Self::Item) -> Result<Self::Item, Self::Error> {
        let db_word_pair = sqlx::query_as!(
            Self::Item,
            "INSERT INTO word_pairs (id, user_id, target_text, source_text, target_language, source_language) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *", 
            &word_pair.id,
            &word_pair.user_id,
            &word_pair.target_text,
            &word_pair.source_text,
            &word_pair.target_language,
            &word_pair.source_language
        )
            .fetch_one(&self.db)
            .await?;

        Ok(db_word_pair)
    }

    async fn select_by_id(&self, id: &i32) -> Result<Self::Item, Self::Error> {
        let db_word_pair = sqlx::query_as!(
            Self::Item,
            "SELECT id, user_id, target_text, source_text, target_language, source_language FROM word_pairs WHERE id = $1",
            &id,
        )
        .fetch_one(&self.db)
        .await?;

        Ok(db_word_pair)
    }

    async fn delete_by_id(&self, id: &i32) -> Result<(), Self::Error> {
        sqlx::query_as!(Self::Item, "DELETE FROM word_pairs WHERE id = $1", &id,)
            .execute(&self.db)
            .await?;

        Ok(())
    }
}

#[async_trait]
impl IWordPairRepository for WordPairPostgresRepository {
    async fn select_by_user_id(&self, user_id: &i32) -> Result<Vec<Self::Item>, Self::Error> {
        let db_word_pairs = sqlx::query_as!(
            Self::Item,
            "SELECT id, user_id, target_text, source_text, target_language, source_language FROM word_pairs WHERE user_id = $1",
            &user_id
        )
            .fetch_all(&self.db)
            .await?;

        Ok(db_word_pairs)
    }
}

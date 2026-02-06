use crate::{
    domain::{
        models::word_pair::WordPair,
        traits::repositories::{repository::Repository, word_pair_repository::IWordPairRepository},
        types::ID,
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
    type Item = WordPair;
    type Entity = WordPairEntity;
    type Error = Error;

    fn new(db: Self::Pool) -> Self {
        Self { db: db }
    }

    async fn insert(&self, word_pair: &Self::Item) -> Result<Self::Item, Self::Error> {
        let entity = Self::Entity::from(word_pair);

        let db_entity = sqlx::query_as::<_, Self::Entity>(
            "INSERT INTO word_pairs (id, user_id, target_text, source_text, target_language, source_language, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *", 
            )
            .bind(&entity.id)
            .bind(&entity.user_id)
            .bind(&entity.target_text)
            .bind(&entity.source_text)
            .bind(&entity.target_language)
            .bind(&entity.source_language)
            .bind(&entity.created_at)
            .fetch_one(&self.db)
            .await?;

        let word_pair = Self::Item::from(&db_entity);

        Ok(word_pair)
    }

    async fn select_by_id(&self, id: &ID) -> Result<Self::Item, Self::Error> {
        let db_entity = sqlx::query_as::<_, Self::Entity>(
            "SELECT id, user_id, target_text, source_text, target_language, source_language, created_at FROM word_pairs WHERE id = $1"
            )
            .bind(id)
            .fetch_one(&self.db)
            .await?;

        let word_pair = Self::Item::from(&db_entity);

        Ok(word_pair)
    }

    async fn delete_by_id(&self, id: &ID) -> Result<(), Self::Error> {
        sqlx::query("DELETE FROM word_pairs WHERE id = $1")
            .bind(id)
            .execute(&self.db)
            .await?;

        Ok(())
    }
}

#[async_trait]
impl IWordPairRepository for WordPairPostgresRepository {
    async fn select_by_user_id(&self, user_id: &ID) -> Result<Vec<Self::Item>, Self::Error> {
        let db_entities = sqlx::query_as::<_, Self::Entity>(
            "SELECT id, user_id, target_text, source_text, target_language, source_language, created_at FROM word_pairs WHERE user_id = $1"
            )
            .bind(user_id)
            .fetch_all(&self.db)
            .await?;

        let mut word_pairs: Vec<Self::Item> = Vec::new();

        for entity in db_entities.iter() {
            word_pairs.push(Self::Item::from(entity));
        }

        Ok(word_pairs)
    }
}

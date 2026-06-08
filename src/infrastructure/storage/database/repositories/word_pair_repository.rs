use crate::{
    domain::{
        models::{
            sort::{FilterBy, GetWordPairsQueryList, SortBy, SortDirection, TimeRange},
            word_pair::WordPair,
        },
        traits::repositories::{repository::Repository, word_pair_repository::IWordPairRepository},
        types::ID,
    },
    infrastructure::storage::database::models::word_pair::WordPairEntity,
};
use async_trait::async_trait;
use sqlx::{Error, QueryBuilder, postgres::PgPool};

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
        tracing::info!("Word pair postgres repository initialized");
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

    async fn select_by_user_id_with_sort_and_filters(
        &self,
        user_id: &ID,
        query_params: GetWordPairsQueryList,
    ) -> Result<Vec<Self::Item>, Self::Error> {
        let mut query_builder = QueryBuilder::new(
            "SELECT id, user_id, target_text, source_text, target_language, source_language, created_at FROM word_pairs WHERE user_id = ",
        );
        query_builder.push_bind(user_id);

        if let Some(filters) = query_params.filter_by {
            for filter in filters {
                match filter {
                    FilterBy::Time(time_range) => match time_range {
                        TimeRange::To(time) => {
                            query_builder.push(" AND created_at < ").push_bind(time);
                        }
                        TimeRange::From(time) => {
                            query_builder.push(" AND created_at > ").push_bind(time);
                        }
                        TimeRange::Between(time_1, time_2) => {
                            query_builder.push(" AND created_at > ").push_bind(time_1);
                            query_builder.push(" AND created_at < ").push_bind(time_2);
                        }
                    },
                    FilterBy::Language {
                        target_language,
                        source_language,
                    } => {
                        if let Some(target) = target_language {
                            query_builder
                                .push(" AND target_language = ")
                                .push_bind(target);
                        }
                        if let Some(source) = source_language {
                            query_builder
                                .push(" AND source_language = ")
                                .push_bind(source);
                        }
                    }
                }
            }
        }

        match query_params.sort_by {
            Some(SortBy::Time(direction)) => {
                query_builder.push(" ORDER BY created_at ");
                match direction {
                    SortDirection::Asc => {
                        query_builder.push("ASC");
                    }
                    SortDirection::Desc => {
                        query_builder.push("DESC");
                    }
                }
            }
            None => {}
        }

        let db_entities: Vec<WordPairEntity> =
            query_builder.build_query_as().fetch_all(&self.db).await?;

        let mut word_pairs: Vec<Self::Item> = Vec::new();

        for entity in db_entities.iter() {
            word_pairs.push(Self::Item::from(entity));
        }

        Ok(word_pairs)
    }
}

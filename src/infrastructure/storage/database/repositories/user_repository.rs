use crate::{
    domain::{
        models::user::User,
        traits::repositories::{repository::Repository, user_repository::IUserRepository},
        types::ID,
    },
    infrastructure::storage::database::models::user::UserEntity,
};
use async_trait::async_trait;
use sqlx::{Error, postgres::PgPool};

#[derive(Clone)]
pub struct UserPostgresRepository {
    db: PgPool,
}

#[async_trait]
impl Repository for UserPostgresRepository {
    type Pool = PgPool;
    type Item = User;
    type Entity = UserEntity;
    type Error = Error;

    fn new(db: Self::Pool) -> Self {
        Self { db: db }
    }

    async fn insert(&self, user: &Self::Item) -> Result<Self::Item, Self::Error> {
        let entity = Self::Entity::from(user);

        let db_entity = sqlx::query_as::<_, Self::Entity>(
            "INSERT INTO users (id, hashed_password, key, name, created_at, updated_at) 
                 VALUES ($1, $2, $3, $4, $5, $6) 
                 RETURNING *",
        )
        .bind(&entity.id)
        .bind(&entity.hashed_password)
        .bind(&entity.key)
        .bind(&entity.name)
        .bind(&entity.created_at)
        .bind(&entity.updated_at)
        .fetch_one(&self.db)
        .await?;

        let user = Self::Item::from(&db_entity);

        Ok(user)
    }

    async fn select_by_id(&self, id: &ID) -> Result<Self::Item, Self::Error> {
        let db_entity = sqlx::query_as::<_, Self::Entity>(
            "SELECT id, key, name, hashed_password, created_at, updated_at FROM users WHERE id = $1"
            )
            .bind(id)
            .fetch_one(&self.db)
            .await?;

        let user = User::from(&db_entity);

        Ok(user)
    }

    async fn delete_by_id(&self, id: &ID) -> Result<(), Self::Error> {
        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(&self.db)
            .await?;

        Ok(())
    }
}

#[async_trait]
impl IUserRepository for UserPostgresRepository {
    async fn select_by_key(&self, key: &str) -> Result<Self::Item, Self::Error> {
        let db_entity = sqlx::query_as::<_, Self::Entity>(
            "SELECT id, key, name, hashed_password, created_at, updated_at FROM users WHERE key=$1",
        )
        .bind(key)
        .fetch_one(&self.db)
        .await?;

        let user = Self::Item::from(&db_entity);

        Ok(user)
    }

    async fn update_by_id(&self, updated_user: &Self::Item) -> Result<Self::Item, Self::Error> {
        let entity = &Self::Entity::from(updated_user);

        let updated_db_entity = sqlx::query_as::<_, Self::Entity>(
            "UPDATE users SET key = COALESCE($1, key), name = COALESCE($2, name), hashed_password = COALESCE($3, hashed_password), updated_at = $4 WHERE id = $5 RETURNING *"
            )
            .bind(&entity.key)
            .bind(&entity.name)
            .bind(&entity.hashed_password)
            .bind(&entity.updated_at)
            .bind(&entity.created_at)
            .bind(&entity.id)
            .fetch_one(&self.db)
            .await?;

        let updated_user = Self::Item::from(&updated_db_entity);

        Ok(updated_user)
    }
}

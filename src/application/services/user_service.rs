use std::borrow::Cow;

use thiserror::Error;
use tracing::error;

use crate::{
    api::models::user::{CreateUserDTO, UpdateUserDTO},
    domain::{
        models::user::User,
        traits::{
            crypto::crypto::ICrypto,
            repositories::{repository::Repository, user_repository::IUserRepository},
        },
        types::ID,
    },
    infrastructure::{storage::database::models::user::UserEntity, utils::password::Argon2Crypto},
};

#[derive(Clone)]
pub struct UserService<Repo> {
    repo: Repo,
}

#[derive(Error, Debug)]
pub enum UserServiceError {
    #[error("User with this key already exists: {0}")]
    KeyAlreadyExists(String),

    #[error("Invalid user key: {0}")]
    InvalidKey(String),

    #[error("Password is too easy: {0}")]
    InvalidPassword(String),

    #[error("User not found: `{0}`")]
    NotFound(String),

    #[error("Database error: `{0}`")]
    Database(#[from] sqlx::Error),

    #[error("Crypto driver error: `{0}`")]
    Crypto(String),

    #[error("Unknown error")]
    Unknown,
}

impl<Repo> UserService<Repo>
where
    Repo: Repository<Item = User, Entity = UserEntity, Error = sqlx::Error>,
{
    pub fn new(repo: Repo) -> Self {
        Self { repo: repo }
    }

    pub async fn create(&self, params: &CreateUserDTO) -> Result<User, UserServiceError> {
        let crypto_driver = Argon2Crypto;

        let hashed_password =
            crypto_driver
                .hash(&params.password)
                .map_err(|error| match &error {
                    _ => {
                        error!("User crypto error: {:?}", error);
                        UserServiceError::Crypto(error.to_string())
                    }
                })?;

        let user = User::new_simple(params.key.clone(), params.name.clone(), hashed_password);

        let res = self
            .repo
            .insert(&user)
            .await
            .map_err(|error| match &error {
                sqlx::Error::Database(error) if error.code() == Some(Cow::Borrowed("23505")) => {
                    UserServiceError::KeyAlreadyExists(params.key.clone())
                }
                _ => {
                    error!("User DB error: {:?}", error);
                    UserServiceError::Database(error.into())
                }
            })?;

        Ok(res)
    }

    pub async fn get_by_id(&self, id: &ID) -> Result<User, UserServiceError> {
        let res = self
            .repo
            .select_by_id(id)
            .await
            .map_err(|error| match &error {
                sqlx::Error::RowNotFound => UserServiceError::NotFound(error.to_string()),
                _ => {
                    error!("User DB error: {}", error);
                    UserServiceError::Database(error.into())
                }
            })?;

        Ok(res)
    }

    pub async fn delete_by_id(&self, id: &ID) -> Result<(), UserServiceError> {
        self.repo
            .delete_by_id(id)
            .await
            .map_err(|error| match &error {
                sqlx::Error::RowNotFound => UserServiceError::NotFound(error.to_string()),
                _ => {
                    error!("User DB error: {}", error);
                    UserServiceError::Database(error.into())
                }
            })?;

        Ok(())
    }
}

impl<Repo> UserService<Repo>
where
    Repo: IUserRepository<Error = sqlx::Error>,
{
    pub async fn get_by_key(&self, key: &str) -> Result<User, UserServiceError> {
        let res = self
            .repo
            .select_by_key(key)
            .await
            .map_err(|error| match &error {
                sqlx::Error::RowNotFound => UserServiceError::NotFound(error.to_string()),
                _ => {
                    error!("User DB error: {}", error);
                    UserServiceError::Database(error.into())
                }
            })?;

        Ok(res)
    }

    pub async fn update_by_id(
        &self,
        id: &ID,
        params: &UpdateUserDTO,
    ) -> Result<User, UserServiceError> {
        let mut user = self
            .repo
            .select_by_id(id)
            .await
            .map_err(|error| match &error {
                sqlx::Error::RowNotFound => UserServiceError::NotFound(error.to_string()),
                _ => {
                    error!("User DB error: {}", error);
                    UserServiceError::Database(error.into())
                }
            })?;

        user.update(params.key.clone(), params.name.clone());

        let res = self.repo.update_by_id(&user).await.map_err(|error| {
            error!("User DB error: {}", error);
            error
        })?;

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use chrono::Utc;
    use uuid::Uuid;

    use super::*;

    struct TestUserRepository {
        _db: i32,
    }

    #[async_trait]
    impl Repository for TestUserRepository {
        type Pool = i32;
        type Item = User;
        type Entity = UserEntity;
        type Error = sqlx::Error;

        fn new(db: Self::Pool) -> Self {
            Self { _db: db }
        }

        async fn insert(&self, item: &Self::Item) -> Result<Self::Item, Self::Error> {
            Ok(item.clone())
        }

        async fn select_by_id(&self, id: &ID) -> Result<Self::Item, Self::Error> {
            let test_user = User {
                id: *id,
                key: "faksfjas".to_string(),
                name: "Mdafasdfd".to_string(),
                hashed_password: "JJalksf".to_string(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };

            Ok(test_user)
        }

        async fn delete_by_id(&self, id: &ID) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    #[async_trait]
    impl IUserRepository for TestUserRepository {
        async fn select_by_key(&self, key: &str) -> Result<Self::Item, Self::Error> {
            let test_user = User {
                id: Uuid::new_v4(),
                key: key.to_string(),
                name: "Mdafasdfd".to_string(),
                hashed_password: "JJalksf".to_string(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };

            Ok(test_user)
        }

        async fn update_by_id(
            &self,
            updated_params: &Self::Item,
        ) -> Result<Self::Item, Self::Error> {
            Ok(updated_params.clone())
        }
    }

    #[tokio::test]
    async fn test_create() {
        let repo = TestUserRepository { _db: 12345 };

        let user_service = UserService::new(repo);

        let test_user = CreateUserDTO {
            key: "fsdfsf".to_string(),
            name: "Me".to_string(),
            password: "hefkajdf".to_string(),
        };

        let res = user_service.create(&test_user).await.unwrap();

        assert_eq!(res.name, test_user.name);
        assert_eq!(res.key, test_user.key);
        assert_ne!(res.hashed_password, test_user.password);
    }
}

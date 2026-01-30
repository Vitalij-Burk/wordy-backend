use std::borrow::Cow;

use thiserror::Error;
use tracing::error;

use crate::{
    api::models::user::{CreateUserDTO, UpdateUserDTO},
    domain::{
        models::user::User,
        traits::repositories::{repository::Repository, user_repository::IUserRepository},
    },
    infrastructure::storage::database::models::user::{UpdateDBUser, UserEntity},
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

    #[error("Unknown error")]
    Unknown,
}

impl<Repo> UserService<Repo>
where
    Repo: Repository<Item = UserEntity, Error = sqlx::Error>,
{
    pub fn new(repo: Repo) -> Self {
        Self { repo: repo }
    }

    pub async fn create(&self, params: &CreateUserDTO) -> Result<User, UserServiceError> {
        let user = User::new(&params.key, &params.name);

        let res =
            self.repo
                .insert(&UserEntity::from(user))
                .await
                .map_err(|error| match &error {
                    sqlx::Error::Database(error)
                        if error.code() == Some(Cow::Borrowed("23505")) =>
                    {
                        UserServiceError::KeyAlreadyExists(params.key.clone())
                    }
                    _ => {
                        error!("User DB error: {:?}", error);
                        UserServiceError::Database(error.into())
                    }
                })?;
        let user = User::from(res);

        Ok(user)
    }

    pub async fn get_by_id(&self, id: &i32) -> Result<User, UserServiceError> {
        let res = self.repo.select_by_id(id).await.map_err(|error| {
            error!("User DB error: {}", error);
            error
        })?;
        let user = User::from(res);

        Ok(user)
    }

    pub async fn delete_by_id(&self, id: &i32) -> Result<(), UserServiceError> {
        self.repo.delete_by_id(id).await.map_err(|error| {
            error!("User DB error: {}", error);
            error
        })?;

        Ok(())
    }
}

impl<Repo> UserService<Repo>
where
    Repo: IUserRepository<Error = sqlx::Error>,
{
    pub async fn get_by_key(&self, key: &str) -> Result<User, UserServiceError> {
        let res = self.repo.select_by_key(key).await.map_err(|error| {
            error!("User DB error: {}", error);
            error
        })?;
        let user = User::from(res);

        Ok(user)
    }

    pub async fn update_by_id(
        &self,
        id: &i32,
        params: &UpdateUserDTO,
    ) -> Result<User, UserServiceError> {
        let params = UpdateDBUser {
            key: params.key.to_owned(),
            name: params.name.to_owned(),
        };

        let res = self.repo.update_by_id(id, params).await.map_err(|error| {
            error!("User DB error: {}", error);
            error
        })?;

        let user = User::from(res);

        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;

    use crate::infrastructure::storage::database::models::user::UpdateDBUser;

    use super::*;

    struct TestUserRepository {
        _db: i32,
    }

    #[async_trait]
    impl Repository for TestUserRepository {
        type Pool = i32;
        type Item = UserEntity;
        type Error = sqlx::Error;

        fn new(db: Self::Pool) -> Self {
            Self { _db: db }
        }

        async fn insert(&self, item: &UserEntity) -> Result<Self::Item, Self::Error> {
            Ok(item.clone())
        }

        async fn select_by_id(&self, id: &i32) -> Result<Self::Item, Self::Error> {
            let test_user = UserEntity {
                id: *id,
                key: "faksfjas".to_string(),
                name: "Mdafasdfd".to_string(),
            };

            Ok(test_user)
        }

        async fn delete_by_id(&self, id: &i32) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    #[async_trait]
    impl IUserRepository for TestUserRepository {
        async fn select_by_key(&self, key: &str) -> Result<Self::Item, Self::Error> {
            let test_user = UserEntity {
                id: 122398923,
                key: key.to_string(),
                name: "Mdafasdfd".to_string(),
            };

            Ok(test_user)
        }

        async fn update_by_id(
            &self,
            id: &i32,
            params: UpdateDBUser,
        ) -> Result<Self::Item, Self::Error> {
            let test_user = UserEntity {
                id: *id,
                key: "Hey".to_string(),
                name: "Mdafasdfd".to_string(),
            };

            Ok(test_user)
        }
    }

    #[tokio::test]
    async fn test_create() {
        let repo = TestUserRepository { _db: 12345 };

        let user_service = UserService::new(repo);

        let test_user = CreateUserDTO {
            key: "fsdfsf".to_string(),
            name: "Me".to_string(),
        };

        let res = user_service.create(&test_user).await.unwrap();

        assert_eq!(res.name, test_user.name);
        assert_eq!(res.key, test_user.key);
    }
}

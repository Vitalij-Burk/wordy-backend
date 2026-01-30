use async_trait::async_trait;

use crate::{
    domain::traits::repositories::repository::Repository,
    infrastructure::storage::database::models::user::{UpdateDBUser, UserEntity},
};

#[async_trait]
pub trait IUserRepository: Repository<Item = UserEntity> {
    async fn select_by_key(&self, key: &str) -> Result<Self::Item, Self::Error>;

    async fn update_by_id(&self, id: &i32, params: UpdateDBUser)
    -> Result<Self::Item, Self::Error>;
}

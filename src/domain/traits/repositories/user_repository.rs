use async_trait::async_trait;

use crate::{
    domain::{models::user::User, traits::repositories::repository::Repository},
    infrastructure::storage::database::models::user::UserEntity,
};

#[async_trait]
pub trait IUserRepository: Repository<Item = User, Entity = UserEntity> {
    async fn select_by_key(&self, key: &str) -> Result<Self::Item, Self::Error>;

    async fn update_by_id(&self, updated_user: &Self::Item) -> Result<Self::Item, Self::Error>;
}

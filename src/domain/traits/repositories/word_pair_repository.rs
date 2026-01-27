use async_trait::async_trait;

use crate::{
    domain::traits::repositories::repository::Repository,
    infrastructure::storage::database::models::word_pair::WordPairEntity,
};

#[async_trait]
pub trait IWordPairRepository: Repository<Item = WordPairEntity> {
    async fn select_by_user_id(&self, user_id: &i32) -> Result<Vec<Self::Item>, Self::Error>;
}

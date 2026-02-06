use async_trait::async_trait;

use crate::{
    domain::{
        models::word_pair::WordPair, traits::repositories::repository::Repository, types::ID,
    },
    infrastructure::storage::database::models::word_pair::WordPairEntity,
};

#[async_trait]
pub trait IWordPairRepository: Repository<Item = WordPair, Entity = WordPairEntity> {
    async fn select_by_user_id(&self, user_id: &ID) -> Result<Vec<Self::Item>, Self::Error>;
}

use async_trait::async_trait;

use crate::{
    domain::{
        models::{sort::GetWordPairsQueryList, word_pair::WordPair},
        traits::repositories::repository::Repository,
        types::ID,
    },
    infrastructure::storage::database::models::word_pair::WordPairEntity,
};

#[async_trait]
pub trait IWordPairRepository: Repository<Item = WordPair, Entity = WordPairEntity> {
    async fn select_by_user_id(&self, user_id: &ID) -> Result<Vec<Self::Item>, Self::Error>;

    async fn select_by_user_id_with_sort_and_filters(
        &self,
        user_id: &ID,
        query_params: GetWordPairsQueryList,
    ) -> Result<Vec<Self::Item>, Self::Error>;
}

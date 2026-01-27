use async_trait::async_trait;

#[async_trait]
pub trait Repository: Send + Sync {
    type Pool;
    type Item;
    type Error: std::error::Error;

    fn new(db: Self::Pool) -> Self;

    async fn insert(&self, item: &Self::Item) -> Result<Self::Item, Self::Error>;
}

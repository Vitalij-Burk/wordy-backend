use async_trait::async_trait;

#[async_trait]
pub trait Repository: Send + Sync {
    type Pool: Send + Sync;
    type Item: Send + Sync;
    type Error: std::error::Error + Send + Sync;

    fn new(db: Self::Pool) -> Self;

    async fn insert(&self, item: &Self::Item) -> Result<Self::Item, Self::Error>;

    async fn select_by_id(&self, id: &i32) -> Result<Self::Item, Self::Error>;

    async fn delete_by_id(&self, id: &i32) -> Result<(), Self::Error>;
}

use async_trait::async_trait;

#[async_trait]
pub trait ITranslator {
    type Item;
    type Error: std::error::Error;

    async fn translate_text(
        &self,
        source_text: &str,
        source_language: &str,
        target_language: &str,
    ) -> Result<Self::Item, Self::Error>;
}

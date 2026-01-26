use crate::domain::traits::translator::translator::ITranslator;
use async_trait::async_trait;
use translators::{GoogleTranslator, Translator};

#[derive(Clone)]
pub struct TranslatorsTranslator;

#[async_trait]
impl ITranslator for TranslatorsTranslator {
    type Item = String;
    type Error = translators::Error;

    async fn translate_text(
        &self,
        source_text: &str,
        source_language: &str,
        target_language: &str,
    ) -> Result<String, translators::Error> {
        let google_translator = GoogleTranslator::default();

        let target_text = google_translator
            .translate_async(source_text, source_language, target_language)
            .await?;

        Ok(target_text)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        domain::traits::translator::translator::ITranslator,
        infrastructure::external_api::translate::translate::TranslatorsTranslator,
    };

    #[tokio::test]
    async fn test_translate_text() {
        let source_text = "Hello";
        let target_language = "de";
        let source_language = "en";
        let expected_value = "hallo".to_string();

        let translator = TranslatorsTranslator;

        let target_text = translator
            .translate_text(source_text, source_language, target_language)
            .await
            .unwrap();

        assert_eq!(expected_value, target_text.to_lowercase());
    }
}

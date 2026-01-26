use thiserror::Error;
use tracing::error;

use crate::domain::traits::translator::translator::ITranslator;

#[derive(Clone)]
pub struct TranslateService<Translator> {
    pub translator: Translator,
}

#[derive(Error, Debug)]
pub enum TranslateServiceError {
    #[error("Language not found: `{0}`")]
    NotFoundLanguage(String),

    #[error("Translation failed: `{0}`")]
    TranslatorError(#[from] translators::Error),

    #[error("Unknown error")]
    Unknown,
}

impl<Translator> TranslateService<Translator>
where
    Translator: ITranslator<Item = String, Error = translators::Error>,
{
    pub fn new(translator: Translator) -> Self {
        Self {
            translator: translator,
        }
    }
    pub async fn translate_text(
        &self,
        source_text: &str,
        target_language: &str,
        source_language: &str,
    ) -> Result<String, TranslateServiceError> {
        let target_text = self
            .translator
            .translate_text(source_text, source_language, target_language)
            .await
            .map_err(|error| {
                error!("Translation error: {}", error);
                error
            })?;

        Ok(target_text)
    }
}

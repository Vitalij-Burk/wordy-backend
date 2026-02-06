use thiserror::Error;
use tracing::error;

use crate::{
    api::models::translate::TranslateDTO,
    domain::{models::translate::Translation, traits::translate::translator::ITranslator},
};

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
        params: &TranslateDTO,
    ) -> Result<Translation, TranslateServiceError> {
        let res = self
            .translator
            .translate_text(
                &params.source_text,
                &params.source_language,
                &params.target_language,
            )
            .await
            .map_err(|error| {
                error!("Translation error: {}", error);
                error
            })?;

        let translation = Translation::new(
            &res,
            &params.source_text,
            &params.target_language,
            &params.source_language,
        );

        Ok(translation)
    }
}

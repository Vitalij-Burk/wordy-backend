use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
use thiserror::Error;

use crate::{
    api::auth::models::Claims,
    domain::error::error_handling::log_err,
    infrastructure::auth::token::jwks::claims::{JwksClaims, JwksClaimsError, usize_to_datetime},
};

#[derive(Debug, Clone)]
pub struct JwksTokenValidator {
    validation: Validation,
}

#[derive(Debug, Error)]
pub enum JwksTokenValidatorError {
    #[error("Rsa error: {0}")]
    Rsa(#[from] rsa::Error),

    #[error("JWT error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("Jwks claims error: {0}")]
    JwksClaims(#[from] JwksClaimsError),
}

impl JwksTokenValidator {
    pub fn new() -> Self {
        let validation = Validation::new(Algorithm::RS256);

        Self { validation }
    }

    pub fn verify(
        &self,
        token: &str,
        decoding_key: &DecodingKey,
    ) -> Result<bool, JwksTokenValidatorError> {
        let _ = decode::<JwksClaims>(&token, &decoding_key, &self.validation)
            .map_err(log_err)?
            .claims;

        Ok(true)
    }

    pub fn decode(
        &self,
        token: &str,
        decoding_key: &DecodingKey,
    ) -> Result<Claims, JwksTokenValidatorError> {
        let storage_claims = decode::<JwksClaims>(&token, &decoding_key, &self.validation)
            .map_err(log_err)?
            .claims;

        let claims = Claims {
            sub: storage_claims.sub,
            jti: storage_claims.jti,
            iat: usize_to_datetime(storage_claims.iat)?,
            exp: usize_to_datetime(storage_claims.exp)?,
        };

        Ok(claims)
    }
}

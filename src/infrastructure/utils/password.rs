use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

use crate::domain::traits::crypto::crypto::ICrypto;

pub struct Argon2Crypto;

impl ICrypto for Argon2Crypto {
    type Error = argon2::password_hash::Error;

    fn hash(&self, password: &str) -> Result<String, Self::Error> {
        let salt = SaltString::generate(&mut OsRng);

        let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)?
            .to_string();

        Ok(password_hash)
    }

    fn verify(&self, password: &str, hashed_password: &str) -> Result<(), Self::Error> {
        let parsed_hash = PasswordHash::new(&hashed_password)?;

        let argon2 = Argon2::default();

        argon2.verify_password(password.as_bytes(), &parsed_hash)?;

        Ok(())
    }
}

pub trait ICrypto: Send + Sync {
    type Error: Send + Sync;

    fn hash(&self, source: &str) -> Result<String, Self::Error>;

    fn verify(&self, source: &str, hashed: &str) -> Result<(), Self::Error>;
}

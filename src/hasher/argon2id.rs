use argon2::{self, Config, ThreadMode, Variant, Version};
use rand::prelude::*;
use rand_hc::Hc128Rng;
use std::str;
use zeroize::Zeroize;

///
/// An object that simplifies the use argon2id
///
pub struct Argon2id<'a> {
    rng: Hc128Rng,
    config: Config<'a>,
}

impl Argon2id<'_> {
    ///
    /// Creates a new instance with some default strong settings.
    ///
    pub fn new() -> Argon2id<'static> {
        let rng = Hc128Rng::from_entropy();
        let config = Config {
            variant: Variant::Argon2id,
            version: Version::Version13,
            mem_cost: 131072,
            time_cost: 25,
            lanes: 8,
            thread_mode: ThreadMode::Parallel,
            secret: &[],
            ad: &[],
            hash_length: 8912,
        };

        Argon2id { rng, config }
    }

    ///
    /// Create a custom instance with your desired settings
    ///
    pub fn new_custom(config: Config<'static>) -> Argon2id<'static> {
        let rng = Hc128Rng::from_entropy();

        Argon2id { rng, config }
    }

    ///
    /// Hash bytes
    ///
    pub fn hash_bytes(&mut self, pass: &[u8]) -> Result<Vec<u8>, argon2::Error> {
        let mut salt: [u8; 1024] = [0; 1024];
        self.rng.fill_bytes(&mut salt);

        let hash = match argon2::hash_encoded(pass, &salt, &self.config) {
            Ok(hash) => hash,
            Err(error) => return Err(error),
        };

        salt.zeroize();
        Ok(hash.into_bytes())
    }

    ///
    /// Hash a string
    ///
    pub fn hash_str(&mut self, pass: &str) -> Result<Vec<u8>, argon2::Error> {
        let mut salt: [u8; 1024] = [0; 1024];
        self.rng.fill_bytes(&mut salt);

        let hash = match argon2::hash_encoded(pass.as_bytes(), &salt, &self.config) {
            Ok(hash) => hash,
            Err(error) => return Err(error),
        };

        salt.zeroize();
        Ok(hash.into_bytes())
    }

    ///
    /// Check if 2 hashes are equal.
    /// you can't just use == since there is salt used
    /// to make the hash different each time the hashing process
    /// is run.
    ///
    pub fn verify_str(&mut self, hash: &Vec<u8>, pass: &str) -> Result<bool, argon2::Error> {
        let hash_str = match str::from_utf8(&hash) {
            Ok(hash_str) => hash_str,
            Err(_) => return Err(argon2::Error::DecodingFail),
        };

        argon2::verify_encoded(hash_str, pass.as_bytes())
    }

    ///
    /// Check if 2 hashes are equal.
    /// you can't just use == since there is salt used
    /// to make the hash different each time the hashing process
    /// is run.
    ///
    pub fn verify_bytes(&mut self, hash: &Vec<u8>, pass: &[u8]) -> Result<bool, argon2::Error> {
        let hash_str = match str::from_utf8(&hash) {
            Ok(hash_str) => hash_str,
            Err(_) => return Err(argon2::Error::DecodingFail),
        };

        argon2::verify_encoded(hash_str, pass)
    }
}

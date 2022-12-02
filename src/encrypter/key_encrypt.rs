use argon2::{self, Config, ThreadMode, Variant, Version};
use chacha20poly1305::aead::{Aead, NewAead};
use chacha20poly1305::{Key, XChaCha20Poly1305, XNonce};
use rand::prelude::*;
use rand_hc::Hc128Rng;
use zeroize::Zeroize;

///
/// And object that uses a cryptographically generated rng (Hc128Rng)
/// and argon2id in order to generate a 32 bytes long key (256bits)
/// for encrypting using an algorithm like chacha20poly1305
/// that takes 32 bytes keys
///
/// Its use in this app is for generating a key from the password
/// in order to encrypt the private key of the vault.
///
pub struct KeyEncrypt<'a> {
    rng: Hc128Rng,
    config_argon: Config<'a>,
}

impl KeyEncrypt<'_> {
    ///
    /// Creates a new KeyEncrypt object with necessary configuration needed
    /// to achive the goal
    ///
    pub fn new() -> KeyEncrypt<'static> {
        let rng = Hc128Rng::from_entropy();
        let config_argon = Config {
            variant: Variant::Argon2id,
            version: Version::Version13,
            mem_cost: 131072,
            time_cost: 100,
            lanes: 8,
            thread_mode: ThreadMode::Parallel,
            secret: &[],
            ad: &[],
            hash_length: 32,
        };

        KeyEncrypt { rng, config_argon }
    }

    ///
    /// Takes the given passwords, generates a 32 bytes key
    /// and encrypts the given key with XChaCha20Poly1305.
    ///
    pub fn encrypt_with_password_bytes(
        &mut self,
        password: &[u8],
        key: &[u8; 32],
    ) -> Result<Vec<u8>, argon2::Error> {
        // generate a salt to hash the password with
        let mut salt: [u8; 1024] = [0; 1024];
        self.rng.fill_bytes(&mut salt);

        // hash the password with the raw method so only the hash comes out, in order to use it for
        // encryption
        let hash = match argon2::hash_raw(password, &salt, &self.config_argon) {
            Ok(hash) => hash,
            Err(error) => return Err(error),
        };

        // generate a key and an aead
        let encryption_key = Key::from_slice(&hash); // 32-bytes
        let aead = XChaCha20Poly1305::new(encryption_key);

        // generate a strong nonce
        let mut nonce: [u8; 24] = [0; 24];
        self.rng.fill_bytes(&mut nonce);
        let nonce = XNonce::from_slice(&nonce);

        // encrypt key with crated aead and nonce
        let mut cipher = match aead.encrypt(&nonce, key.as_ref()) {
            Ok(cipher) => cipher,
            Err(_) => return Err(argon2::Error::DecodingFail),
        };

        // append the nonce and the salt to the vec to be used for decryption
        cipher.append(&mut nonce.to_vec());
        cipher.append(&mut salt.to_vec());

        // clean memory
        salt.zeroize();

        // return the encrypted bytes
        Ok(cipher)
    }

    ///
    /// Decrypt the given key using the given password,
    /// key has to have been encrypted with XChaCha20Poly1305.
    ///
    pub fn decrypt_with_password_bytes(
        &mut self,
        password: &[u8],
        encrypted_key: &[u8],
    ) -> Result<Vec<u8>, argon2::Error> {
        // split the arrays
        let bytes_split1 = encrypted_key.split_at(encrypted_key.len() - 1024);
        let bytes_split2 = bytes_split1.0.split_at(bytes_split1.0.len() - 24);

        // get the nonce and the salt
        let salt = bytes_split1.1;
        let nonce = XNonce::from_slice(bytes_split2.1);

        // hash the password with the raw method so only the hash comes out, in order to use it for
        // decryption
        let mut hash = match argon2::hash_raw(password, &salt, &self.config_argon) {
            Ok(hash) => hash,
            Err(error) => return Err(error),
        };

        // get the actual cihper
        let cipher = bytes_split2.0;

        // create aead
        let key = Key::from_slice(&hash);
        let aead = XChaCha20Poly1305::new(key);

        // decrypt cipher
        let plaintext = match aead.decrypt(&nonce, cipher) {
            Ok(text) => text,
            Err(_) => return Err(argon2::Error::DecodingFail),
        };

        // clean memory
        hash.zeroize();

        // return the decrypted bytes
        Ok(plaintext)
    }
}

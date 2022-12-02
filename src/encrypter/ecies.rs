use crypto_box::{
    aead::generic_array::typenum, aead::generic_array::GenericArray, aead::Aead, ChaChaBox,
    PublicKey, SecretKey,
};
use rand::prelude::*;
use rand_hc::Hc128Rng;
use zeroize::Zeroize;

pub struct ECIES {
    rng: Hc128Rng,
}

impl ECIES {
    ///
    /// Generates a new instance of ECIES
    /// (eliptic curve integrated encryption scheme)
    /// Combines the X25519 Diffie-Hellman function and
    /// the xchacha20poly1305 authenticated encryption cipher
    /// into an ECIES.
    /// Which is what I personally consider
    /// the penicle of end to end encryption.
    ///
    /// Now why use end to end encryption in this project?
    /// Well, future proof, and why not as there is no
    /// downside to it, same security and like no performance impact.
    ///
    /// This generetes a new instanse using a cryptographically
    /// generated rng (Hc128Rng)
    ///
    pub fn new() -> ECIES {
        let rng = Hc128Rng::from_entropy();

        ECIES { rng }
    }

    ///
    /// Generates a new key pair for the ECIES.
    ///
    pub fn gen_key_pair(&mut self) -> ([u8; 32], [u8; 32]) {
        let secret_key = SecretKey::generate(&mut self.rng);
        let public_key = secret_key.public_key();

        (
            public_key.as_bytes().to_owned(),
            secret_key.as_bytes().to_owned(),
        )
    }

    ///
    /// Encrypts a vec<u8> with the given secret and public key.
    ///
    pub fn encrypt_bytes(
        &mut self,
        bytes: &Vec<u8>,
        secret_key: &[u8; 32],
        public_key: &[u8; 32],
    ) -> Result<Vec<u8>, crypto_box::aead::Error> {
        let crypt_box = ChaChaBox::new(
            &PublicKey::from(public_key.to_owned()),
            &SecretKey::from(secret_key.to_owned()),
        );

        let mut nonce = crypto_box::generate_nonce(&mut self.rng);
        let mut cipher = match crypt_box.encrypt(&nonce, &bytes[..]) {
            Ok(cipher) => cipher,
            Err(error) => return Err(error),
        };

        cipher.append(&mut nonce.to_vec());
        nonce.zeroize();

        Ok(cipher)
    }

    ///
    /// Decrypts the given vec<u8> with the given secret and public key.
    ///
    pub fn decrypt_bytes(
        &mut self,
        bytes: &Vec<u8>,
        secret_key: &[u8; 32],
        public_key: &[u8; 32],
    ) -> Result<Vec<u8>, crypto_box::aead::Error> {
        let crypt_box = ChaChaBox::new(
            &PublicKey::from(public_key.to_owned()),
            &SecretKey::from(secret_key.to_owned()),
        );

        let bytes_split = bytes.split_at(bytes.len() - 24);

        let mut nonce: GenericArray<u8, typenum::U24> = *GenericArray::from_slice(&bytes_split.1);
        let mut cipher = bytes_split.0.to_vec();

        let decrypted = match crypt_box.decrypt(&nonce, &cipher[..]) {
            Ok(decrypted) => decrypted,
            Err(error) => return Err(error),
        };

        nonce.zeroize();
        cipher.zeroize();

        Ok(decrypted)
    }

    ///
    /// Encrypts an [u8] array using the given secret and public keys.
    ///
    pub fn encrypt_bytes_array(
        &mut self,
        bytes: &[u8],
        secret_key: &[u8; 32],
        public_key: &[u8; 32],
    ) -> Result<Vec<u8>, crypto_box::aead::Error> {
        let crypt_box = ChaChaBox::new(
            &PublicKey::from(public_key.to_owned()),
            &SecretKey::from(secret_key.to_owned()),
        );

        let mut nonce = crypto_box::generate_nonce(&mut self.rng);
        let mut cipher = match crypt_box.encrypt(&nonce, &bytes[..]) {
            Ok(cipher) => cipher,
            Err(error) => return Err(error),
        };

        cipher.append(&mut nonce.to_vec());
        nonce.zeroize();

        Ok(cipher)
    }

    ///
    /// Decrypts the given [u8] array using the given secret and public keys
    ///
    pub fn decrypt_bytes_array(
        &mut self,
        bytes: &[u8],
        secret_key: &[u8; 32],
        public_key: &[u8; 32],
    ) -> Result<Vec<u8>, crypto_box::aead::Error> {
        let crypt_box = ChaChaBox::new(
            &PublicKey::from(public_key.to_owned()),
            &SecretKey::from(secret_key.to_owned()),
        );

        let bytes_split = bytes.split_at(bytes.len() - 24);

        let mut nonce: GenericArray<u8, typenum::U24> = *GenericArray::from_slice(&bytes_split.1);
        let mut cipher = bytes_split.0.to_vec();

        let decrypted = match crypt_box.decrypt(&nonce, &cipher[..]) {
            Ok(decrypted) => decrypted,
            Err(error) => return Err(error),
        };

        nonce.zeroize();
        cipher.zeroize();

        Ok(decrypted)
    }
}

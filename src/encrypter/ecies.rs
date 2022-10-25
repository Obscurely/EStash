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
    pub fn new() -> ECIES {
        let rng = Hc128Rng::from_entropy();

        ECIES { rng }
    }

    pub fn gen_key_pair(&mut self) -> ([u8; 32], [u8; 32]) {
        let secret_key = SecretKey::generate(&mut self.rng);
        let public_key = secret_key.public_key();

        (
            public_key.as_bytes().to_owned(),
            secret_key.as_bytes().to_owned(),
        )
    }

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

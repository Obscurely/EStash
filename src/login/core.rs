use crate::{utils::{db, self}, encrypter::{ecies::ECIES, key_encrypt::KeyEncrypt}, hasher::argon2id::Argon2id};
use crate::hasher::blake3;
use crate::hasher::argon2id;
use std::str;
use serde::{Serialize, Deserialize};
use crate::utils::Vault;

#[derive(Serialize, Deserialize)]
pub struct VaultDbValue {
    id: u64,
    password: Vec<u8>,
}

// Extension method for Vec obj in order to be able to easily convert between Vec<u8> and [u8;
// usize]
pub trait ToOwnedArray {
    fn to_owned_array<T, const N: usize>(v: Vec<T>) -> Result<[T; N], Vec<T>> {
        v.try_into()
    }
}

impl ToOwnedArray for Vec<u8> {
    fn to_owned_array<T, const N: usize>(v: Vec<T>) -> Result<[T; N], Vec<T>> {
        v.try_into()
    }
}

pub fn login_vault(vault_name: &str, password: &str, estashdb: &mut db::EstashDb, argon: &mut Argon2id, ecies: &mut ECIES, key_encrypt: &mut KeyEncrypt, is_windows: bool ) -> Vault {
    let vault_name_hashed = blake3::hash_str(vault_name);
    // check if the vault is present in the database
    match estashdb.vault_db.contains_key(vault_name_hashed) {
        Ok(val) => {
            if !val {
                println!("Vault doesn't exist!");
                std::process::exit(100);
                // TODO: handle error
            }
        }
        Err(error) => {
            println!("{error}");
            std::process::exit(100);
            // TODO: handle error
        },
    }

    // get back the data at the given vault name 
    let data = match estashdb.vault_db.get(vault_name_hashed) {
        Ok(data_unchecked) => match data_unchecked {
            Some(data_raw) => match str::from_utf8(&data_raw) {
                Ok(string) => string.to_owned(),
                Err(error) => {
                    println!("{error}");
                    std::process::exit(100);
                    // TODO: handle error
                },
            },
            None => {
                panic!("No data at specified key!");
                // TODO: handle error
            },
        },
        Err(error) => {
            panic!("{error}");
            // TODO: handle error
        },
    };

    // parse data into object
    let vault_value: VaultDbValue = match serde_json::from_str(&data) {
        Ok(value) => value,
        Err(error) => {
            panic!("{error}");
            // TODO: handle error
        },
    };

    // extract data into separate values
    let mut vault_id = vault_value.id;
    let mut vault_password = vault_value.password;

    // check if passwords match
    let is_password_right = match argon.verify_str(&vault_password, password) {
        Ok(status) => status,
        Err(error) => {
            panic!("{error}");
            // TODO: handle error
        }
    };

    if !is_password_right {
        panic!("Password is not right!");
        // TODO: handle error
    }

    // extract and decrypt the private encryption key for the vault
    let vault_priv_key_encrypted = match estashdb.vault_priv_key_db.get(vault_name_hashed) {
        Ok(data_unchecked) => match data_unchecked {
            Some(data_raw) => data_raw.to_vec(),
            None => {
                panic!("No data at specified key!");
                // TODO: handle error
            },
        },
        Err(error) => {
            panic!("{error}");
            // TODO: handle error
        },
    }; 
    let vault_priv_key = match key_encrypt.decrypt_with_password_bytes(password.as_bytes(), &vault_priv_key_encrypted) {
        Ok(key) => key,
        Err(error) => {
            panic!("{error}");
            // TODO: handle error
        }
    };

    // extract the public encryption key for the vault
    let vault_pub_key = match estashdb.vault_pub_key_db.get(vault_name_hashed) {
        Ok(data_unchecked) => match data_unchecked {
            Some(pub_key_raw) => pub_key_raw.to_vec(),
            None => {
                panic!("No data at the specified key!");
                // TODO: handle error
            }
        },
        Err(error) => {
            panic!("{error}");
            // TODO: handle error
        }
    };

    // TODO: handle errors
    let vault_priv_key_bytes: [u8; 32] = Vec::to_owned_array(vault_priv_key).unwrap();
    let vault_pub_key_bytes: [u8; 32] = Vec::to_owned_array(vault_pub_key).unwrap();

    Vault {
        vault_name: vault_name.to_string(),
        id: vault_id,
        priv_key: vault_priv_key_bytes,
        pub_key: vault_pub_key_bytes,
    }
}

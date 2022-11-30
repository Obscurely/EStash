use crate::hasher::blake3;
use crate::utils::Vault;
use crate::{encrypter::key_encrypt::KeyEncrypt, utils::db};
use serde::{Deserialize, Serialize};
use std::str;

#[derive(Serialize, Deserialize)]
pub struct VaultDbValue {
    id: u64,
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

#[derive(Debug)]
pub enum LoginError {
    WrongCredentials(u16),
    CorruptedVaultsDb(u16),
    FailedToAccessVaultsDb(u16),
    CorruptedVault(u16),
    FailedToAccessPrivKeyDb(u16),
    CorruptedPubKeyDb(u16),
    FailedToAccessPubKeyDb(u16),
}

pub fn login_vault(
    vault_name: &str,
    password: &str,
    estashdb: &mut db::EstashDb,
    key_encrypt: &mut KeyEncrypt,
) -> Result<Vault, LoginError> {
    let vault_name_hashed = blake3::hash_str(vault_name);
    // check if the vault is present in the database
    match estashdb.vault_db.contains_key(vault_name_hashed) {
        Ok(val) => {
            if !val {
                return Err(LoginError::WrongCredentials(0));
            }
        }
        Err(error) => {
            eprintln!("ERROR: There was an error accessing vaults db!\n{error}");
            return Err(LoginError::FailedToAccessVaultsDb(0));
        }
    }

    // get back the data at the given vault name
    let data = match estashdb.vault_db.get(vault_name_hashed) {
        Ok(data_unchecked) => match data_unchecked {
            Some(data_raw) => match str::from_utf8(&data_raw) {
                Ok(string) => string.to_owned(),
                Err(error) => {
                    eprintln!("ERROR: The data stored under the hash of the vault name is not stored in utf8 which means the vaults db in corrupted!\n{error}");
                    return Err(LoginError::CorruptedVaultsDb(0));
                }
            },
            None => {
                eprintln!("ERROR: There is a key with the vault's name hash, but it has no data, meaning the vaults db is corrupted!");
                return Err(LoginError::CorruptedVaultsDb(0));
            }
        },
        Err(error) => {
            eprintln!("ERROR: There was an error accessing vaults db!\n{error}");
            return Err(LoginError::FailedToAccessVaultsDb(0));
        }
    };

    // parse data into object
    let vault_value: VaultDbValue = match serde_json::from_str(&data) {
        Ok(value) => value,
        Err(error) => {
            eprintln!("ERROR: Value under vault is not stored in estash's format, meaning vaults db is corrupted!\n{error}");
            return Err(LoginError::CorruptedVaultsDb(0));
        }
    };

    // extract data into separate values
    let vault_id = vault_value.id;

    // extract and decrypt the private encryption key for the vault
    let vault_priv_key_encrypted = match estashdb.vault_priv_key_db.get(vault_name_hashed) {
        Ok(data_unchecked) => match data_unchecked {
            Some(data_raw) => data_raw.to_vec(),
            None => {
                eprintln!("ERROR: There is a key with the vault's name hash, but it has no data, meaning the vault priv key db is corrupted!");
                return Err(LoginError::CorruptedVaultsDb(0));
            }
        },
        Err(error) => {
            eprintln!("ERROR: There was an error accessing priv key db!\n{error}");
            return Err(LoginError::FailedToAccessPrivKeyDb(0));
        }
    };
    let vault_priv_key = match key_encrypt
        .decrypt_with_password_bytes(password.as_bytes(), &vault_priv_key_encrypted)
    {
        Ok(key) => key,
        Err(error) => {
            eprintln!("ERROR: The credentials are wrong!\n{error}");
            return Err(LoginError::WrongCredentials(0));
        }
    };

    // extract the public encryption key for the vault
    let vault_pub_key = match estashdb.vault_pub_key_db.get(vault_name_hashed) {
        Ok(data_unchecked) => match data_unchecked {
            Some(pub_key_raw) => pub_key_raw.to_vec(),
            None => {
                eprintln!("ERROR: The data stored under the hash of the vault name is not stored in utf8 which means the pub key db in corrupted!");
                return Err(LoginError::CorruptedPubKeyDb(0));
            }
        },
        Err(error) => {
            eprintln!("ERROR: There was an error trying to access the pub key db!\n{error}");
            return Err(LoginError::FailedToAccessPubKeyDb(0));
        }
    };

    let vault_priv_key_bytes: [u8; 32] = match Vec::to_owned_array(vault_priv_key) {
        Ok(key) => key,
        Err(_) => {
            eprintln!("ERROR: The vault private key we got is not 32 bytes, meaning the vault is probably corrupted, unrecoverable!");
            return Err(LoginError::CorruptedVault(0));
        }
    };

    let vault_pub_key_bytes: [u8; 32] = match Vec::to_owned_array(vault_pub_key) {
        Ok(key) => key,
        Err(_) => {
            eprintln!("ERROR: The vault public key we got is not 32 bytes, meaning the vault is probably corrupted, unrecoverable!");
            return Err(LoginError::CorruptedVault(0));
        }
    };

    Ok(Vault {
        vault_name: vault_name.to_string(),
        id: vault_id,
        priv_key: vault_priv_key_bytes,
        pub_key: vault_pub_key_bytes,
    })
}

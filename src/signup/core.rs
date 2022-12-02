use crate::hasher::blake3;
use crate::{
    encrypter::{ecies::ECIES, key_encrypt::KeyEncrypt},
    utils::{self, db},
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::str;

#[derive(Serialize, Deserialize)]
pub struct VaultDbValue {
    id: u64,
}

#[derive(Debug)]
pub enum SingupError {
    AlreadyExists(u16),
    FailedToAccessVaultsDb(u16),
    CorruptedVaultsDb(u16),
    FailedToCreateVault(u16),
    FailedToStoreCredentials(u16),
    FailedToStorePrivateKey(u16),
    FailedToStorePublicKey(u16),
    UnknownError(u16),
}

///
/// Takes the info parsed from the window and creates a vault with it.
/// Encrypted with keys encrypted with a key derived from the password.
/// Very secure.
///
pub fn create_vault(
    vault_name: &str,
    password: &str,
    estashdb: &mut db::EstashDb,
    ecies: &mut ECIES,
    key_encrypt: &mut KeyEncrypt,
    is_windows: bool,
) -> Result<bool, SingupError> {
    // check if hashed vault_name isn't already present
    let hashed_vault_name = blake3::hash_str(&vault_name);

    match estashdb.vault_db.contains_key(hashed_vault_name) {
        Ok(is_already_present) => {
            if is_already_present {
                return Err(SingupError::AlreadyExists(0));
            }
        }
        Err(error) => {
            eprintln!("ERROR: Failed to access vaults db!\n{error}");
            return Err(SingupError::FailedToAccessVaultsDb(0));
        }
    }

    // get the last db id and increment that by 1 if there is one, if not set it to 1
    let new_id = match estashdb.vault_db.last() {
        Ok(entry) => match entry {
            Some(ent) => {
                let value_str = match str::from_utf8(&ent.1) {
                    Ok(key) => key,
                    Err(error) => {
                        eprintln!("ERROR: The stored value in vaults db is not in utf8 meaning that vaults db is corrupted!\n{error}");
                        return Err(SingupError::CorruptedVaultsDb(0));
                    }
                };

                let parsed: serde_json::Value = match serde_json::from_str(value_str) {
                    Ok(value) => value,
                    Err(error) => {
                        eprintln!("ERROR: Failed to covert the value to a json object, vaults db is probably corrupted!\n{error}");
                        return Err(SingupError::CorruptedVaultsDb(0));
                    }
                };

                let previous_id = match parsed.get("id") {
                    Some(id) => id,
                    None => {
                        eprintln!("ERROR: Somehow successfully parsed the data under the key into the struct, but there is no id field?");
                        return Err(SingupError::UnknownError(0));
                    }
                };

                let previous_id_int = match previous_id.as_u64() {
                    Some(id) => id,
                    None => {
                        eprintln!("ERROR: The previous id doesn't fit in an u64, meaning vaults db is probably corrupted!");
                        return Err(SingupError::CorruptedVaultsDb(0));
                    }
                };

                let new_id = previous_id_int + 1;

                new_id
            }
            None => 1,
        },
        Err(error) => {
            eprintln!("ERROR: There was an error accessing vaults db!\n{error}");
            return Err(SingupError::FailedToAccessVaultsDb(0));
        }
    };

    // create the vaule to store under the key (json of password and vault id)
    let vault_value_obj = VaultDbValue { id: new_id };
    let vault_value_string = match serde_json::to_string(&vault_value_obj) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("ERROR: Well... this somehow failed... unable to covert to json string the struct containing the new vault id!\n{err}");
            return Err(SingupError::UnknownError(0));
        }
    };

    // create encryption keys
    // gen key pair
    let key_pair = ecies.gen_key_pair();
    let public_key = key_pair.0;
    let private_key = key_pair.1;

    // encrypt private key
    let encrypted_private_key = match key_encrypt
        .encrypt_with_password_bytes(password.as_bytes(), &private_key)
    {
        Ok(cipher) => cipher,
        Err(error) => {
            // shouldn't fail
            eprintln!("ERROR: There was an error encrypting the generated key using the password!\n{error}");
            return Err(SingupError::UnknownError(0));
        }
    };

    // create vault
    if is_windows {
        match sled::open(
            utils::constants::VAULTS_ROOT_PATH_WINDOWS.to_string() + &new_id.to_string(),
        ) {
            Ok(db) => db,
            Err(error) => {
                eprintln!("ERROR: There was an error creating the vault!\n{error}");
                return Err(SingupError::FailedToCreateVault(0));
            }
        };
    } else {
        match sled::open(utils::constants::VAULTS_ROOT_PATH_UNIX.to_string() + &new_id.to_string())
        {
            Ok(db) => db,
            Err(error) => {
                eprintln!("ERROR: There was an error creating the vault!\n{error}");

                // cleanup potential created folder
                match fs::remove_dir_all(
                    utils::constants::VAULTS_ROOT_PATH_UNIX.to_string() + &new_id.to_string(),
                ) {
                    Ok(_) => (),
                    Err(_) => (),
                };

                return Err(SingupError::FailedToCreateVault(0));
            }
        };
    }

    // store new vault entry
    match estashdb
        .vault_db
        .insert(hashed_vault_name.as_ref(), vault_value_string.as_bytes())
    {
        Ok(_) => (),
        Err(error) => {
            eprintln!("ERROR: Failed to store newly created vault!\n{error}");
            return Err(SingupError::FailedToStoreCredentials(0));
        }
    };

    // store the private key
    match estashdb
        .vault_priv_key_db
        .insert(hashed_vault_name.as_ref(), encrypted_private_key)
    {
        Ok(_) => (),
        Err(error) => {
            eprintln!("ERROR: Failed to store private key for vault!\n{error}");

            // remove previously stored vault entry in db
            match estashdb.vault_db.remove(hashed_vault_name.as_ref()) {
                Ok(_) => (),
                Err(err) => {
                    eprintln!("ERROR: After failing to store the private key we tried removing the previously added entry, but that didn't succed either, vaults db may be corrupted!\n{err}");
                }
            };

            return Err(SingupError::FailedToStorePrivateKey(0));
        }
    };

    // store the public key
    match estashdb
        .vault_pub_key_db
        .insert(hashed_vault_name.as_ref(), public_key.as_ref())
    {
        Ok(_) => (),
        Err(error) => {
            eprintln!("ERROR: Failed to store public key for vault!\n{error}");

            // remove previously stored vault entry in db and private key
            match estashdb.vault_db.remove(hashed_vault_name.as_ref()) {
                Ok(_) => (),
                Err(err) => {
                    eprintln!("ERROR: After failing to store the public key we tried removing the previously added entry, but that didn't succed either, vaults db may be corrupted!\n{err}");
                }
            };
            match estashdb
                .vault_priv_key_db
                .remove(hashed_vault_name.as_ref())
            {
                Ok(_) => (),
                Err(err) => {
                    eprintln!("ERROR: After failing to store the public key we tried removing the previously stored private key, but that didn't succed either, private key db may be corrupted!\n{err}");
                }
            }

            return Err(SingupError::FailedToStorePublicKey(0));
        }
    };

    Ok(true)
}

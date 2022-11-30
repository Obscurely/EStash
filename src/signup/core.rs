use crate::hasher::blake3;
use crate::{
    encrypter::{ecies::ECIES, key_encrypt::KeyEncrypt},
    utils::{self, db},
};
use serde::{Deserialize, Serialize};
use std::str;

#[derive(Serialize, Deserialize)]
pub struct VaultDbValue {
    id: u64,
}

pub fn create_vault(
    vault_name: &str,
    password: &str,
    estashdb: &mut db::EstashDb,
    ecies: &mut ECIES,
    key_encrypt: &mut KeyEncrypt,
    is_windows: bool,
) -> bool {
    // check if hashed vault_name isn't already present
    let hashed_vault_name = blake3::hash_str(&vault_name);

    match estashdb.vault_db.contains_key(hashed_vault_name) {
        Ok(is_already_present) => {
            if is_already_present {
                // TODO: Handle error
                return false;
            }
        }
        Err(error) => {
            println!("{error}");
            std::process::exit(100);
            // TODO handle error
        }
    }

    // get the last db id and increment that by 1 if there is one, if not set it to 1
    let new_id = match estashdb.vault_db.last() {
        Ok(entry) => match entry {
            Some(ent) => {
                let value_str = match str::from_utf8(&ent.1) {
                    Ok(key) => key,
                    Err(error) => {
                        println!("{error}");
                        std::process::exit(100);
                        // TODO: handle error
                    }
                };

                let parsed: serde_json::Value = match serde_json::from_str(value_str) {
                    Ok(value) => value,
                    Err(error) => {
                        println!("{error}");
                        std::process::exit(100);
                        // TODO: handle error
                    }
                };

                let previous_id = match parsed.get("id") {
                    Some(id) => id,
                    None => {
                        std::process::exit(100);
                        // TODO: handle error
                    }
                };

                let previous_id_int = match previous_id.as_u64() {
                    Some(id) => id,
                    None => {
                        std::process::exit(100);
                        // TODO: handle error
                    }
                };

                let new_id = previous_id_int + 1;

                new_id
            }
            None => 1,
        },
        Err(error) => {
            println!("{error}");
            std::process::exit(100);
            // TODO: handle error
        }
    };

    // create the vaule to store under the key (json of password and vault id)
    let vault_value_obj = VaultDbValue { id: new_id };
    let vault_value_string = match serde_json::to_string(&vault_value_obj) {
        Ok(value) => value,
        Err(err) => {
            println!("{err}");
            std::process::exit(100);
            // TODO: handle error
        }
    };

    // create encryption keys
    // gen key pair
    let key_pair = ecies.gen_key_pair();
    let public_key = key_pair.0;
    let private_key = key_pair.1;

    // encrypt private key
    let encrypted_private_key =
        match key_encrypt.encrypt_with_password_bytes(password.as_bytes(), &private_key) {
            Ok(cipher) => cipher,
            Err(error) => {
                println!("{error}");
                std::process::exit(100);
                // TODO: handle error
            }
        };

    // create vault
    if is_windows {
        match sled::open(
            utils::constants::VAULTS_ROOT_PATH_WINDOWS.to_string() + &new_id.to_string(),
        ) {
            Ok(db) => db,
            Err(error) => {
                println!("{error}");
                std::process::exit(100);
                // TODO: handle error
            }
        };
    } else {
        match sled::open(utils::constants::VAULTS_ROOT_PATH_UNIX.to_string() + &new_id.to_string())
        {
            Ok(db) => db,
            Err(error) => {
                println!("{error}");
                std::process::exit(100);
                // TODO: handle error
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
            println!("{error}");
            std::process::exit(100);
            // TODO: handle error
        }
    };

    // store the private key
    match estashdb
        .vault_priv_key_db
        .insert(hashed_vault_name.as_ref(), encrypted_private_key)
    {
        Ok(_) => (),
        Err(error) => {
            println!("{error}");
            std::process::exit(100);
            // TODO: handle error
        }
    };

    // store the public key
    match estashdb
        .vault_pub_key_db
        .insert(hashed_vault_name.as_ref(), public_key.as_ref())
    {
        Ok(_) => (),
        Err(error) => {
            println!("{error}");
            std::process::exit(100);
            // TODO: handle error
        }
    };

    true
}

use std::process;

use super::constants;
use sled;

#[derive(Debug, Clone)]
pub struct EstashDb {
    pub vault_db: sled::Db,
    pub vault_pub_key_db: sled::Db,
    pub vault_priv_key_db: sled::Db,
}

impl EstashDb {
    ///
    /// Loads the database needed by the program.
    ///
    pub fn new() -> Result<EstashDb, sled::Error> {
        if std::env::consts::FAMILY == "windows" { 
            let document_dir = match dirs::document_dir() {
                Some(dir) => dir,
                None => {
                    eprintln!("ERROR: Failed to get document dir");
                    process::exit(200);
                }
            };
            // unwrap here is alright
            let estash_dir = document_dir.to_str().unwrap().to_owned() + "\\estash\\";
            let vault_db = match sled::open(estash_dir.clone() + constants::VAULT_DB_PATH_WINDOWS) {
                Ok(db) => db,
                Err(error) => return Err(error),
            };
            let vault_pub_key_db = match sled::open(estash_dir.clone() + constants::VAULT_PUB_KEY_DB_PATH_WINDOWS) {
                Ok(db) => db,
                Err(error) => return Err(error),
            };
            let vault_priv_key_db = match sled::open(estash_dir.clone() + constants::VAULT_PRIV_KEY_DB_PATH_WINDOWS) {
                Ok(db) => db,
                Err(error) => return Err(error),
            };

            Ok(EstashDb {
                vault_db,
                vault_pub_key_db,
                vault_priv_key_db,
            })
        } else {
            let home_dir = match dirs::home_dir() {
                Some(dir) => dir,
                None => {
                    eprintln!("ERROR: Failed to get home dir");
                    process::exit(200);
                }
            };
            // unwrap here is alright
            let estash_dir = home_dir.to_str().unwrap().to_owned() + "/.estash/";
            let vault_db = match sled::open(estash_dir.clone() + constants::VAULT_DB_PATH_UNIX) {
                Ok(db) => db,
                Err(error) => return Err(error),
            };
            let vault_pub_key_db = match sled::open(estash_dir.clone() + constants::VAULT_PUB_KEY_DB_PATH_UNIX) {
                Ok(db) => db,
                Err(error) => return Err(error),
            };
            let vault_priv_key_db = match sled::open(estash_dir.clone() + constants::VAULT_PRIV_KEY_DB_PATH_UNIX) {
                Ok(db) => db,
                Err(error) => return Err(error),
            };

            Ok(EstashDb {
                vault_db,
                vault_pub_key_db,
                vault_priv_key_db,
            })
        }
    }
}

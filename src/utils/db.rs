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
            let vault_db = match sled::open(constants::VAULT_DB_PATH_WINDOWS) {
                Ok(db) => db,
                Err(error) => return Err(error),
            };
            let vault_pub_key_db = match sled::open(constants::VAULT_PUB_KEY_DB_PATH_WINDOWS) {
                Ok(db) => db,
                Err(error) => return Err(error),
            };
            let vault_priv_key_db = match sled::open(constants::VAULT_PRIV_KEY_DB_PATH_WINDOWS) {
                Ok(db) => db,
                Err(error) => return Err(error),
            };

            Ok(EstashDb {
                vault_db,
                vault_pub_key_db,
                vault_priv_key_db,
            })
        } else {
            let vault_db = match sled::open(constants::VAULT_DB_PATH_UNIX) {
                Ok(db) => db,
                Err(error) => return Err(error),
            };
            let vault_pub_key_db = match sled::open(constants::VAULT_PUB_KEY_DB_PATH_UNIX) {
                Ok(db) => db,
                Err(error) => return Err(error),
            };
            let vault_priv_key_db = match sled::open(constants::VAULT_PRIV_KEY_DB_PATH_UNIX) {
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

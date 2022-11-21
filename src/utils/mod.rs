pub mod db;
pub mod constants;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Vault {
    pub vault_name: String,
    pub id: u64,
    pub priv_key: Vec<u8>,
    pub pub_key: Vec<u8>,
}

impl Vault {
    pub fn new_empty() -> Vault {
        Vault {
            vault_name: String::new(),
            id: 0,
            priv_key: Vec::new(),
            pub_key: Vec::new(),
        }
    }
}

pub fn is_windows() -> bool {
    if std::env::consts::FAMILY == "windows" {
        true
    } else {
        false
    }
}

pub mod db;
pub mod constants;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Vault {
    pub vault_name: String,
    pub id: u64,
    pub priv_key: [u8; 32],
    pub pub_key: [u8; 32],
}

impl Vault {
    pub fn new_empty() -> Vault {
        Vault {
            vault_name: String::new(),
            id: 0,
            priv_key: [0; 32],
            pub_key: [0; 32],
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

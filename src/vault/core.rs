use std::path;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct VaultValue {
    pub install_path: String,
    pub content: String,
    pub notes: String,
}

impl VaultValue {
    pub fn new_empty() -> VaultValue {
        VaultValue {
            install_path: String::new(),
            content: String::new(),
            notes: String::new(),
        }
    }
}

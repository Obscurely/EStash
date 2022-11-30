use crate::encrypter::ecies::ECIES;
use crate::utils::Vault;
use serde::{Deserialize, Serialize};
use sled::Db;
use std::collections::HashMap;
use std::path;
use std::str;
use std::sync::{Arc, Mutex};

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

pub fn get_entry_value_plain(
    vault_db_arc_clone: Arc<Mutex<Db>>,
    ecies_arc_clone: Arc<Mutex<ECIES>>,
    vault_arc_clone: Arc<Mutex<Vault>>,
    selected_item: &str,
    db_entries_dict_arc_clone: Arc<Mutex<HashMap<String, Vec<u8>>>>,
) -> VaultValue {
    let vault = vault_arc_clone.lock().unwrap();
    // TODO: handle errors
    let entry_value_encrypted = vault_db_arc_clone
        .lock()
        .unwrap()
        .get(
            db_entries_dict_arc_clone
                .lock()
                .unwrap()
                .get(selected_item)
                .unwrap(),
        )
        .unwrap()
        .unwrap()
        .to_vec();
    let entry_value_decrypted = ecies_arc_clone
        .lock()
        .unwrap()
        .decrypt_bytes(&entry_value_encrypted, &vault.priv_key, &vault.pub_key)
        .unwrap();
    let entry_value_string = str::from_utf8(&entry_value_decrypted).unwrap();
    // parase entry value into json
    let entry_value_json: VaultValue = serde_json::from_str(entry_value_string).unwrap();

    entry_value_json
}

pub fn add_new_entry(
    vault_db_arc_clone: Arc<Mutex<Db>>,
    vault_arc_clone: Arc<Mutex<Vault>>,
    entrie_name: &str,
    ecies_arc_clone: Arc<Mutex<ECIES>>,
) -> Vec<u8> {
    let vault = vault_arc_clone.lock().unwrap();
    // TODO: handle errors
    let entrie_add_input_value_encrypted = ecies_arc_clone
        .lock()
        .unwrap()
        .encrypt_bytes_array(entrie_name.as_bytes(), &vault.priv_key, &vault.pub_key)
        .unwrap();
    let empty_value = VaultValue::new_empty();
    let emtpy_value_string = serde_json::to_string(&empty_value).unwrap();
    let emtpy_value_encrypted = ecies_arc_clone
        .lock()
        .unwrap()
        .encrypt_bytes_array(
            emtpy_value_string.as_bytes(),
            &vault.priv_key,
            &vault.pub_key,
        )
        .unwrap();

    // FIXME: add a check for wether the entry is already present or not so we don't
    // overwrite it with empty
    vault_db_arc_clone
        .lock()
        .unwrap()
        .insert(
            (&entrie_add_input_value_encrypted).to_owned(),
            emtpy_value_encrypted,
        )
        .unwrap();

    entrie_add_input_value_encrypted
}

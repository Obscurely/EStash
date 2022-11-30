use crate::encrypter::ecies::ECIES;
use crate::utils::Vault;
use serde::{Deserialize, Serialize};
use sled::Db;
use std::collections::HashMap;
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

#[derive(Debug)]
pub enum VaultValueErr {
    PoisonErr(u16),
    DisplayNotInSync(u16),
    MemoryNotInSync(u16),
    DbCorrupted(u16),
}

#[derive(Debug)]
pub enum NewEntryErr {
    PoisonErr(u16),
    DbCorrupted(u16),
    DbInaccesible(u16),
    UnknownError(u16),
}

pub fn get_entry_value_plain(
    vault_db_arc_clone: Arc<Mutex<Db>>,
    ecies_arc_clone: Arc<Mutex<ECIES>>,
    vault_arc_clone: Arc<Mutex<Vault>>,
    selected_item: &str,
    db_entries_dict_arc_clone: Arc<Mutex<HashMap<String, Vec<u8>>>>,
) -> Result<VaultValue, VaultValueErr> {
    let vault = match vault_arc_clone.lock() {
        Ok(v) => v,
        Err(err) => {
            eprintln!("ERROR: Failed to get Vault value behind ARC!\n{err}");
            return Err(VaultValueErr::PoisonErr(0));
        }
    };

    // get value behind ARC
    let db_entries_dict = match db_entries_dict_arc_clone.lock() {
        Ok(dict) => dict,
        Err(err) => {
            eprintln!("ERROR: Failed to get db_entries_dict value behind ARC!\n{err}");
            return Err(VaultValueErr::PoisonErr(0));
        }
    };

    // get encrypted selected_item version in db
    let selected_item_encrypted = match db_entries_dict.get(selected_item) {
        Some(cipher) => cipher,
        None => {
            eprintln!("ERROR: The Values In Memory are not in sync with the ones on screen!");
            return Err(VaultValueErr::DisplayNotInSync(0));
        }
    };

    // get value behind arc
    let vault_db = match vault_db_arc_clone.lock() {
        Ok(db) => db,
        Err(err) => {
            eprintln!("ERROR: Failed to get vault_db value behind ARC!\n{err}");
            return Err(VaultValueErr::PoisonErr(0));
        }
    };

    let entry_value_encrypted = match vault_db.get(selected_item_encrypted) {
        Ok(val) => match val {
            Some(data) => data.to_vec(),
            None => {
                eprintln!("ERROR: This Database is corrupted, not readable by estash, the key seems to exist, but there is not data associated with it!");
                return Err(VaultValueErr::DbCorrupted(0));
            }
        },
        Err(err) => {
            eprintln!("ERROR: The Values In Storage are not in sync with ones in memory!\n{err}");
            return Err(VaultValueErr::MemoryNotInSync(0));
        }
    };

    // get value behind arc
    let mut ecies = match ecies_arc_clone.lock() {
        Ok(ecies) => ecies,
        Err(err) => {
            eprintln!("Failed to get ecies value behind ARC!\n{err}");
            return Err(VaultValueErr::PoisonErr(0));
        }
    };

    let entry_value_decrypted = match ecies.decrypt_bytes(
        &entry_value_encrypted,
        &vault.priv_key,
        &vault.pub_key,
    ) {
        Ok(plain) => plain,
        Err(err) => {
            eprintln!("ERROR: This db may be corrupted, altough we were able to decrypt the internal key for db we are able to use it for this entry!\n{err}");
            return Err(VaultValueErr::DbCorrupted(0));
        }
    };

    let entry_value_string = match str::from_utf8(&entry_value_decrypted) {
        Ok(s) => s,
        Err(err) => {
            eprintln!("ERROR: This db may be corrupted, altough through the UI you are only able to store utf-8 values, we weren't able to parse one from storage!\n{err}");
            return Err(VaultValueErr::DbCorrupted(0));
        }
    };

    // parase entry value into json
    let entry_value_json: VaultValue = match serde_json::from_str(entry_value_string) {
        Ok(json) => json,
        Err(err) => {
            eprintln!("ERROR: This db may be corrupted, altough we were able to retrive all data there seems to be some adittional data in the db that prevents the conversion to json!\n{err}");
            return Err(VaultValueErr::DbCorrupted(0));
        }
    };

    Ok(entry_value_json)
}

pub fn add_new_entry(
    vault_db_arc_clone: Arc<Mutex<Db>>,
    vault_arc_clone: Arc<Mutex<Vault>>,
    entrie_name: &str,
    ecies_arc_clone: Arc<Mutex<ECIES>>,
) -> Result<Vec<u8>, NewEntryErr> {
    // get value for vault under arc
    let vault = match vault_arc_clone.lock() {
        Ok(v) => v,
        Err(err) => {
            eprintln!("ERROR: Failed to get value under vault ARC!\n{err}");
            return Err(NewEntryErr::PoisonErr(0));
        }
    };

    // get value for ecies under arc
    let mut ecies = match ecies_arc_clone.lock() {
        Ok(ecies) => ecies,
        Err(err) => {
            eprintln!("ERROR: Failed to get value under ecies ARC!\n{err}");
            return Err(NewEntryErr::PoisonErr(0));
        }
    };

    let entrie_add_input_value_encrypted = match ecies.encrypt_bytes_array(
        entrie_name.as_bytes(),
        &vault.priv_key,
        &vault.pub_key,
    ) {
        Ok(cipher) => cipher,
        Err(err) => {
            eprintln!("ERROR: Even though the vault was successfully loaded there is a problem encrypting data with its keys, it's possible that the db may be corrupted");
            return Err(NewEntryErr::DbCorrupted(0));
        }
    };

    let empty_value = VaultValue::new_empty();

    // there ain't no way in hell this fails, but... let's error handle it I guess
    let emtpy_value_string = match serde_json::to_string(&empty_value) {
        Ok(s) => s,
        Err(err) => {
            eprintln!("ERROR: Somehow converting the static empty entry value typed in code failed to convert to a string... here is the error,\n{err}");
            return Err(NewEntryErr::UnknownError(0));
        }
    };

    // how would this even fail if we are encrypting something static with keys that have
    // worked before, regardless here is error handling it yayy
    let emtpy_value_encrypted = match ecies.encrypt_bytes_array(
        emtpy_value_string.as_bytes(),
        &vault.priv_key,
        &vault.pub_key,
    ) {
        Ok(cipher) => cipher,
        Err(err) => {
            eprintln!("ERROR: Even though we are encrypting static values with keys that have already been used for encrypting and have worked we encountered an error... here is the error,\n{err}");
            return Err(NewEntryErr::UnknownError(0));
        }
    };

    // get value for vault db under arc
    let vault_db = match vault_db_arc_clone.lock() {
        Ok(db) => db,
        Err(err) => {
            eprintln!("ERROR: Failed to get value under vault_db ARC!\n{err}");
            return Err(NewEntryErr::PoisonErr(0));
        }
    };

    match vault_db.insert(
        (&entrie_add_input_value_encrypted).to_owned(),
        emtpy_value_encrypted,
    ) {
        Ok(_) => (),
        Err(err) => {
            eprintln!("ERROR: There was an error storing the given value in the db, for some reason the db in not accesible, please report this error on github if you are able to replicate it!\n{err}");
            return Err(NewEntryErr::DbInaccesible(0));
        }
    };

    Ok(entrie_add_input_value_encrypted)
}

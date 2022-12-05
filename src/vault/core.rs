use crate::encrypter::ecies::ECIES;
use crate::utils::constants;
use crate::utils::Vault;
use fltk::tree;
use serde::{Deserialize, Serialize};
use sled::Db;
use std::collections::HashMap;
use std::process;
use std::str;
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize)]
pub struct VaultValue {
    pub install_path: String,
    pub content: Vec<u8>,
    pub notes: String,
}

impl VaultValue {
    pub fn new_empty() -> VaultValue {
        VaultValue {
            install_path: String::new(),
            content: Vec::new(),
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

pub enum UpdateContentErr {
    PoisonErr(u16),
    DisplayNotInSync(u16),
    MemoryNotInSync(u16),
    UnknownError(u16),
}

///
/// Load the Vault object as a database we can use
///
pub fn load_vault(is_windows: bool, vault: &Vault) -> Arc<Mutex<Db>> {
    let vaults_root_path;
    if is_windows {
        let document_dir = match dirs::document_dir() {
            Some(dir) => dir,
            None => {
                eprintln!("ERROR: Failed to get document dir");
                process::exit(200);
            }
        };
        // unwrap here is alright
        let estash_dir = document_dir.to_str().unwrap().to_owned() + "\\estash\\";
        vaults_root_path = (estash_dir + &constants::VAULTS_ROOT_PATH_WINDOWS).to_string();
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
        vaults_root_path = (estash_dir + constants::VAULTS_ROOT_PATH_UNIX).to_string();
    }
    let vault_db = Arc::new(Mutex::new(
        match sled::open(vaults_root_path + &vault.id.to_string()) {
            Ok(db) => db,
            Err(err) => {
                eprintln!("ERROR: Even though the db appears in the list with db's there isn't one that's actually available in storage, or maybe there has been some one-time error, please try again!\n{err}");
                process::exit(100);
            }
        },
    ));

    return vault_db;
}

///
/// Load the entries from the database.
/// Display them in the tree object.
/// And store them in memory for faster access.
///
pub fn load_entries(
    vault: &Vault,
    vault_db: Arc<Mutex<Db>>,
    ecies: Arc<Mutex<ECIES>>,
    entries: &mut tree::Tree,
) -> Arc<Mutex<HashMap<String, Vec<u8>>>> {
    // get value under vault_db arc
    let vault_db_locked = match vault_db.lock() {
        Ok(db) => db,
        Err(err) => {
            eprintln!("ERROR: For some reason we can't get the value for vault_db under ARC, even though we just created and it hasn't been used anywhere else, please try again!\n{err}");
            process::exit(100);
        }
    };

    let mut ecies_locked = match ecies.lock() {
        Ok(object) => object,
        Err(err) => {
            eprintln!("ERROR: Failed to get value under ecies ARC!\n{err}");
            process::exit(100);
        }
    };

    let db_entries = vault_db_locked.iter();
    let db_entries_dict = Arc::new(Mutex::new(HashMap::new()));

    let mut db_entries_dict_locked = match db_entries_dict.lock() {
        Ok(object) => object,
        Err(err) => {
            eprintln!("ERROR: Failed to get the value under db_entries_dict ARC!\n{err}");
            process::exit(100);
        }
    };

    for entry in db_entries {
        let current_entry_encrypted = match entry {
            Ok(cipher) => cipher.0.to_vec(),
            Err(err) => {
                eprintln!("ERROR: Failed to get an entry that we just read, this error message should not be displayed, but if for some reason is, just try again, or post an issue on github!\n{err}");
                process::exit(100);
            }
        };
        let current_entry_plain = match ecies_locked.decrypt_bytes(
            &current_entry_encrypted,
            &vault.priv_key,
            &vault.pub_key,
        ) {
            Ok(plain) => plain,
            Err(err) => {
                eprintln!("ERROR: Failed to decrypt the entry even though the vault keys were validated, try again, if it doesn't work then the db might be corrupted!\n{err}");
                process::exit(100);
            }
        };
        let current_entry_string = match str::from_utf8(&current_entry_plain) {
            Ok(s) => s,
            Err(err) => {
                eprintln!("ERROR: Failed to convert the decrypted bytes into a string, your current db is most likely corrupted, try again maybe!\n{err}");
                process::exit(100);
            }
        };
        entries.add(current_entry_string);

        db_entries_dict_locked.insert(current_entry_string.to_owned(), current_entry_encrypted);
    }

    drop(vault_db_locked);
    drop(ecies_locked);
    drop(db_entries_dict_locked);

    return db_entries_dict;
}

///
/// Decrypt the value under an entry in the vault and return it
///
pub fn get_entry_value_plain(
    vault_db_arc_clone: Arc<Mutex<Db>>,
    ecies_arc_clone: Arc<Mutex<ECIES>>,
    vault_arc_clone: Arc<Mutex<Vault>>,
    selected_item: &str,
    db_entries_dict_arc_clone: Arc<Mutex<HashMap<String, Vec<u8>>>>,
) -> Result<VaultValue, VaultValueErr> {
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
        Some(cipher) => cipher.to_owned(),
        None => {
            eprintln!("ERROR: The Values In Memory are not in sync with the ones on screen!");
            return Err(VaultValueErr::DisplayNotInSync(0));
        }
    };

    // drop arc ref
    drop(db_entries_dict);
    drop(db_entries_dict_arc_clone);

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

    // drop arc ref
    drop(vault_db);
    drop(vault_db_arc_clone);

    // get value behind arc
    let mut ecies = match ecies_arc_clone.lock() {
        Ok(ecies) => ecies,
        Err(err) => {
            eprintln!("Failed to get ecies value behind ARC!\n{err}");
            return Err(VaultValueErr::PoisonErr(0));
        }
    };
    let vault = match vault_arc_clone.lock() {
        Ok(v) => v,
        Err(err) => {
            eprintln!("ERROR: Failed to get Vault value behind ARC!\n{err}");
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

    // drop arc ref
    drop(vault);
    drop(vault_arc_clone);
    drop(ecies);
    drop(ecies_arc_clone);

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

///
/// Add a new entry to the vault database
///
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
            eprintln!("ERROR: Even though the vault was successfully loaded there is a problem encrypting data with its keys, it's possible that the db may be corrupted\n{err}");
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

    // drop arc ref
    drop(vault);
    drop(vault_arc_clone);
    drop(ecies);
    drop(ecies_arc_clone);

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

pub fn update_content_in_entry(
    current_selected_entry_arc: Arc<Mutex<String>>,
    vault_db_arc: Arc<Mutex<Db>>,
    ecies_arc: Arc<Mutex<ECIES>>,
    vault_arc: Arc<Mutex<Vault>>,
    db_entries_dict_arc: Arc<Mutex<HashMap<String, Vec<u8>>>>,
    file_content: Vec<u8>,
) -> Result<bool, UpdateContentErr> {
    //
    // Get current saved data entry in the database
    //

    // get value under arc
    let current_selected_entry = match current_selected_entry_arc.lock() {
        Ok(object) => object,
        Err(err) => {
            eprintln!("ERROR: Failed to get value under current_selected_entry_arc ARC!\n{err}");
            return Err(UpdateContentErr::PoisonErr(0));
        }
    };

    // get entry data
    let entry_value_json = match super::core::get_entry_value_plain(
        vault_db_arc.clone(),
        ecies_arc.clone(),
        vault_arc.clone(),
        &current_selected_entry,
        db_entries_dict_arc.clone(),
    ) {
        Ok(json) => json,
        Err(VaultValueErr::DbCorrupted(_)) => {
            process::exit(100);
        }
        Err(VaultValueErr::PoisonErr(_)) => {
            return Err(UpdateContentErr::PoisonErr(0));
        }
        Err(VaultValueErr::DisplayNotInSync(_)) => {
            return Err(UpdateContentErr::DisplayNotInSync(0));
        }
        Err(VaultValueErr::MemoryNotInSync(_)) => {
            return Err(UpdateContentErr::MemoryNotInSync(0));
        }
    };

    //
    // Save the entry with new data
    //

    let entry_value_json_new = VaultValue {
        install_path: entry_value_json.install_path,
        content: file_content,
        notes: entry_value_json.notes,
    };

    // shouldn't error, hopefully
    let entry_value_string = match serde_json::to_string(&entry_value_json_new) {
        Ok(s) => s,
        Err(err) => {
            eprintln!("ERROR: Somehow converting the struct to json error'd out, shouldn't have, here is the error,\n{err}");
            return Err(UpdateContentErr::UnknownError(0));
        }
    };

    // get value under ecies_arc arc
    let mut ecies = match ecies_arc.lock() {
        Ok(object) => object,
        Err(err) => {
            eprintln!("ERROR: Failed to get value under ecies_arc ARC!\n{err}");
            return Err(UpdateContentErr::PoisonErr(0));
        }
    };

    // get value under arc
    let vault = match vault_arc.lock() {
        Ok(object) => object,
        Err(err) => {
            eprintln!("ERROR: Failed to get value under ecies_arc ARC!\n{err}");
            return Err(UpdateContentErr::PoisonErr(0));
        }
    };

    // there should be no way for this to error out since once the vault is loaded it means the
    // keys work
    let entry_value_encrypted = match ecies.encrypt_bytes_array(
        &entry_value_string.as_bytes(),
        &vault.priv_key,
        &vault.pub_key,
    ) {
        Ok(cipher) => cipher,
        Err(err) => {
            eprintln!("ERROR: Failed to encrypt the data input by user, there should be no way for this to error out since once the vault is loaded it means the keys work, anyways here is the error,\n{err}");
            return Err(UpdateContentErr::UnknownError(0));
        }
    };

    // drop arc ref
    drop(ecies);
    drop(ecies_arc);

    // get value under arc
    let db_entries_dict = match db_entries_dict_arc.lock() {
        Ok(object) => object,
        Err(err) => {
            eprintln!("ERROR: Failed to get value under db_entries_dict_arc ARC!\n{err}");
            return Err(UpdateContentErr::PoisonErr(0));
        }
    };

    // get value under arc
    let selected_item = current_selected_entry.to_owned();

    let selected_item_encrypted = match db_entries_dict.get(&selected_item) {
        Some(cipher) => cipher.to_owned(),
        None => {
            eprintln!("ERROR: The Values In Memory are not in sync with the ones on screen!");
            return Err(UpdateContentErr::DisplayNotInSync(0));
        }
    };

    // drop arc
    drop(selected_item);
    drop(current_selected_entry);
    drop(current_selected_entry_arc);
    drop(db_entries_dict);
    drop(db_entries_dict_arc);

    // get value under arc
    let vault_db = match vault_db_arc.lock() {
        Ok(object) => object,
        Err(err) => {
            eprintln!("ERROR: Failedt to get value under vault_db_arc ARC!\n{err}");
            return Err(UpdateContentErr::PoisonErr(0));
        }
    };

    match vault_db.insert(selected_item_encrypted, entry_value_encrypted) {
        Ok(_) => (),
        Err(err) => {
            eprintln!("ERROR: There was an error storing the input value in the db, for some reason the db in not accesible, please report this error on github if you are able to replicate it!\n{err}");
            process::exit(100);
        }
    };

    Ok(true)
}

pub mod constants;
pub mod db;
use serde::{Deserialize, Serialize};

const FORBIDDEN_WINDOWS_CHARS: [&str; 29] = [
    ">", "<", ":", "/", "|", "?", "*", "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4",
    "COM5", "COM6", "COM7", "COM8", "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7",
    "LPT8", "LPT9",
];

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

pub fn is_path_os_valid(path: &str) -> bool {
    if path.len() == 0 {
        return false;
    }

    if is_windows() {
        if path.chars().next().unwrap() == ' ' {
            return false;
        }

        if path.chars().last().unwrap() == ' ' {
            return false;
        }

        for invalid_char in FORBIDDEN_WINDOWS_CHARS {
            if path.contains(invalid_char) {
                return false;
            }
        }

        for x in path.split("\\") {
            if x == "" {
                return false;
            }

            let mut x_temp_iter = x.chars();

            if x_temp_iter.next().unwrap() == ' ' {
                return false;
            }

            let last = x.chars().last().unwrap();

            if last == ' ' {
                return false;
            }

            if last == '.' {
                return false;
            }
        }
    } else {
        let mut path_replaced: String = String::from("");
        if path.chars().next().unwrap() == '/' {
            path_replaced = path.replacen("/", "", 1);
        }

        if path_replaced.len() == 0 {
            return false;
        }

        let mut path_replaced = path_replaced.as_str();

        if path_replaced.clone().chars().next().unwrap() == ' ' {
            return false;
        }

        if path_replaced.clone().chars().last().unwrap() == ' ' {
            return false;
        }

        for x in path_replaced.split("/") {
            if x == "" {
                return false;
            }

            let mut x_temp_iter1 = x.chars();
            let mut x_temp_iter2 = x_temp_iter1.clone();
            if x_temp_iter1.next().unwrap() == '.' {
                if x_temp_iter1.clone().count() > 0 {
                    if x_temp_iter1.next().unwrap() == '.' {
                        return false;
                    }
                } else {
                    return false;
                }
            }

            if x_temp_iter2.next().unwrap() == ' ' {
                return false;
            }

            let last = x.chars().last().unwrap();

            if last == ' ' {
                return false;
            }

            if last == '.' {
                return false;
            }
        }
    }

    true
}

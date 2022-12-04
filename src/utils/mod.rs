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

///
/// Return whether the current is OS or not
/// windows in order to know which path
/// separator to use
///
pub fn is_windows() -> bool {
    if std::env::consts::FAMILY == "windows" {
        true
    } else {
        false
    }
}

///
/// Checks if the given path is valid on the current OS
///
pub fn is_path_os_valid(path: &str) -> bool {
    if path.len() == 0 {
        return false;
    }

    if is_windows() {
        match path.chars().next() {
            Some(c) => {
                if c == ' ' {
                    return false;
                }
            }
            None => return false,
        };

        match path.chars().last() {
            Some(c) => {
                if c == ' ' {
                    return false;
                }
            }
            None => return false,
        }

        let mut path_replaced_vec: Vec<&str> = path.split("\\").collect();
        path_replaced_vec.reverse();
        path_replaced_vec.pop();
        path_replaced_vec.reverse();
        let path_replaced = path_replaced_vec.join("");

        for invalid_char in FORBIDDEN_WINDOWS_CHARS {
            if path_replaced.contains(invalid_char) {
                return false;
            }
        }

        for x in path_replaced.split("\\") {
            if x == "" {
                return false;
            }

            let mut x_temp_iter = x.chars();

            match x_temp_iter.next() {
                Some(c) => {
                    if c == ' ' {
                        return false;
                    }
                }
                None => return false,
            };

            let last = match x.chars().last() {
                Some(c) => c,
                None => return false,
            };

            if last == ' ' {
                return false;
            }

            if last == '.' {
                return false;
            }
        }
    } else {
        let mut path_replaced: String = String::from("");
        match path.chars().next() {
            Some(c) => {
                if c == '/' {
                    path_replaced = path.replacen("/", "", 1);
                }
            }
            None => return false,
        };

        if path_replaced.len() == 0 {
            return false;
        }

        let path_replaced = path_replaced.as_str();

        match path_replaced.clone().chars().next() {
            Some(c) => {
                if c == ' ' {
                    return false;
                }
            }
            None => return false,
        };

        match path_replaced.clone().chars().last() {
            Some(c) => {
                if c == ' ' {
                    return false;
                }
            }
            None => return false,
        };

        for x in path_replaced.split("/") {
            if x == "" {
                return false;
            }

            let mut x_temp_iter1 = x.chars();
            let mut x_temp_iter2 = x_temp_iter1.clone();

            match x_temp_iter1.next() {
                Some(c) => {
                    if c == '.' {
                        if x_temp_iter1.clone().count() > 0 {
                            match x_temp_iter1.next() {
                                Some(c) => {
                                    if c == '.' {
                                        return false;
                                    }
                                }
                                None => return false,
                            }
                        } else {
                            return false;
                        }
                    }
                }
                None => return false,
            };

            match x_temp_iter2.next() {
                Some(c) => {
                    if c == ' ' {
                        return false;
                    }
                }
                None => return false,
            };

            let last = match x.chars().last() {
                Some(c) => c,
                None => return false,
            };

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

pub mod db;
pub mod constants;

pub fn is_windows() -> bool {
    if std::env::consts::FAMILY == "windows" {
        true
    } else {
        false
    }
}

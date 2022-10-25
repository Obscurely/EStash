use blake3;

pub fn hash_str(text: &str) -> [u8; 32] {
    blake3::hash(text.as_bytes()).as_bytes().to_owned()
}

pub fn hash_bytes(bytes: &[u8]) -> [u8; 32] {
    blake3::hash(bytes).as_bytes().to_owned()
}

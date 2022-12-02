use blake3;

///
/// Hash a string with blake3.
/// A really, really fast hashing algorithm
/// and secure at the same time.
///
pub fn hash_str(text: &str) -> [u8; 32] {
    blake3::hash(text.as_bytes()).as_bytes().to_owned()
}

///
/// Hash bytes with blake3.
/// A really, really fast hashing algorithm
/// and secure at the same time.
///
pub fn hash_bytes(bytes: &[u8]) -> [u8; 32] {
    blake3::hash(bytes).as_bytes().to_owned()
}

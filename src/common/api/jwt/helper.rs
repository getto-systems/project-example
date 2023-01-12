use jsonwebtoken::{DecodingKey, EncodingKey};

pub fn encoding_key_from_ec_pem(key: &str) -> EncodingKey {
    EncodingKey::from_ec_pem(key.as_bytes()).expect("failed to parse ec pem")
}
pub fn decoding_key_from_ec_pem(key: &str) -> DecodingKey {
    DecodingKey::from_ec_pem(key.as_bytes()).expect("failed to parse ec pem")
}

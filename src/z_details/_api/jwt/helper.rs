use jsonwebtoken::EncodingKey;

pub enum JwtTokenEncoderKey {}

impl JwtTokenEncoderKey {
    pub fn ec(key: String) -> EncodingKey {
        EncodingKey::from_ec_pem(key.as_bytes()).expect("failed to parse ec pem")
    }
}

use jsonwebtoken::DecodingKey;

pub struct AuthOutsideService {
    pub service_url: &'static str,
}
pub struct AuthOutsideDecodingKey {
    pub ticket: DecodingKey<'static>,
    pub api: DecodingKey<'static>,
}

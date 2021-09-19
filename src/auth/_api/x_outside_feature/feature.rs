use jsonwebtoken::DecodingKey;

pub struct AuthOutsideFeature {
    pub(in crate::auth) service: AuthOutsideService,
    pub(in crate::auth) cookie: AuthOutsideCookie,
    pub(in crate::auth) key: AuthOutsideKey,
}
pub struct AuthOutsideService {
    pub service_url: &'static str,
}
pub struct AuthOutsideCookie {
    pub domain: &'static str,
    pub cloudfront_key_pair_id: &'static str,
}
pub struct AuthOutsideKey {
    pub ticket: DecodingKey<'static>,
    pub api: DecodingKey<'static>,
    pub reset_token: DecodingKey<'static>,
}

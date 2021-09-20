use jsonwebtoken::DecodingKey;

use crate::auth::_common::x_outside_feature::feature::AuthOutsideService;

pub struct AuthOutsideFeature {
    pub(in crate::auth) service: AuthOutsideService,
    pub(in crate::auth) cookie: AuthOutsideCookie,
    pub key: AuthOutsideKey,
}
pub struct AuthOutsideCookie {
    pub domain: &'static str,
    pub cloudfront_key_pair_id: &'static str,
}
pub struct AuthOutsideKey {
    pub ticket: DecodingKey<'static>,
    pub api: DecodingKey<'static>,
}

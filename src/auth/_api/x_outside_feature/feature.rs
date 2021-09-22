use crate::auth::_common::x_outside_feature::feature::{
    AuthOutsideDecodingKey, AuthOutsideService,
};

pub struct AuthOutsideFeature {
    pub service: AuthOutsideService,
    pub decoding_key: AuthOutsideDecodingKey,
    pub(in crate::auth) cookie: AuthOutsideCookie,
}
pub struct AuthOutsideCookie {
    pub domain: &'static str,
    pub cloudfront_key_pair_id: &'static str,
}

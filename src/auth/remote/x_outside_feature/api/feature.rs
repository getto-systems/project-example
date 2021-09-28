use crate::auth::remote::x_outside_feature::common::feature::{
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

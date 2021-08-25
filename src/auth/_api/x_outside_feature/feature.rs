pub struct AuthOutsideFeature {
    pub(in crate::auth) service: AuthOutsideService,
    pub(in crate::auth) cookie: AuthOutsideCookie,
}
pub struct AuthOutsideService {
    pub service_url: &'static str,
}
pub struct AuthOutsideCookie {
    pub domain: &'static str,
    pub cloudfront_key_pair_id: &'static str,
}

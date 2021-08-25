use crate::x_outside_feature::_api::env::Env;

use super::feature::{AuthOutsideCookie, AuthOutsideFeature, AuthOutsideService};

pub async fn new_auth_outside_feature(env: &'static Env) -> AuthOutsideFeature {
    AuthOutsideFeature {
        service: AuthOutsideService {
            service_url: &env.auth_service_url,
        },
        cookie: AuthOutsideCookie {
            domain: &env.domain,
            cloudfront_key_pair_id: &env.cloudfront_key_pair_id,
        },
    }
}

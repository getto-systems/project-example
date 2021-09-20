use crate::{
    auth::{
        _api::x_outside_feature::feature::{AuthOutsideCookie, AuthOutsideFeature, AuthOutsideKey},
        _common::x_outside_feature::feature::AuthOutsideService,
    },
    x_outside_feature::_api::env::Env,
    z_details::_common::jwt::helper::decoding_key_from_ec_pem,
};

pub async fn new_auth_outside_feature(env: &'static Env) -> AuthOutsideFeature {
    AuthOutsideFeature {
        service: AuthOutsideService {
            service_url: &env.auth_service_url,
        },
        cookie: AuthOutsideCookie {
            domain: &env.domain,
            cloudfront_key_pair_id: &env.cloudfront_key_pair_id,
        },
        key: AuthOutsideKey {
            ticket: decoding_key_from_ec_pem(&env.ticket_public_key),
            api: decoding_key_from_ec_pem(&env.api_public_key),
        },
    }
}

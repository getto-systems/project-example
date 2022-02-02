use crate::z_lib::remote::jwt::helper::decoding_key_from_ec_pem;

use crate::{
    auth::x_outside_feature::remote::{
        api::feature::{AuthOutsideCookie, AuthOutsideFeature},
        common::feature::{AuthOutsideDecodingKey, AuthOutsideService},
    },
    x_outside_feature::remote::api::env::ApiEnv,
};

pub async fn new_auth_outside_feature(env: &'static ApiEnv) -> AuthOutsideFeature {
    AuthOutsideFeature {
        service: AuthOutsideService {
            service_url: &env.auth_service_url,
        },
        decoding_key: AuthOutsideDecodingKey {
            ticket: decoding_key_from_ec_pem(&env.ticket_public_key),
            api: decoding_key_from_ec_pem(&env.api_public_key),
        },
        cookie: AuthOutsideCookie {
            domain: &env.domain,
            cloudfront_key_pair_id: &env.cloudfront_key_pair_id,
        },
    }
}
use crate::{
    auth::x_outside_feature::feature::{
        AuthOutsideCookie, AuthOutsideDecodingKey, AuthOutsideService, AuthProxyOutsideFeature,
    },
    common::x_outside_feature::feature::CommonOutsideService,
    x_outside_feature::proxy::env::ProxyEnv,
    z_lib::service::x_outside_feature::feature::GoogleServiceAuthorizerOutsideFeature,
};

use crate::z_lib::jwt::helper::decoding_key_from_ec_pem;

pub struct ProxyAppFeature {
    pub auth: AuthProxyOutsideFeature,
    pub core: CoreProxyOutsideFeature,
}

pub struct CoreProxyOutsideFeature {
    pub service: CommonOutsideService,
}

impl ProxyAppFeature {
    pub async fn new(env: &'static ProxyEnv) -> Self {
        Self {
            auth: AuthProxyOutsideFeature {
                service: AuthOutsideService {
                    service_url: &env.auth_service_url,
                    google_authorizer: GoogleServiceAuthorizerOutsideFeature::new(
                        &env.auth_service_url,
                    ),
                },
                decoding_key: AuthOutsideDecodingKey {
                    ticket: decoding_key_from_ec_pem(&env.ticket_public_key),
                    api: decoding_key_from_ec_pem(&env.api_public_key),
                },
                cookie: AuthOutsideCookie {
                    domain: &env.domain,
                    cloudfront_key_pair_id: &env.cloudfront_key_pair_id,
                },
            },
            core: CoreProxyOutsideFeature {
                service: CommonOutsideService {
                    service_url: &env.core_service_url,
                    google_authorizer: GoogleServiceAuthorizerOutsideFeature::new(
                        &env.core_service_url,
                    ),
                },
            },
        }
    }
}

use crate::{
    auth::x_outside_feature::feature::{
        AuthOutsideCookie, AuthOutsideDecodingKey, AuthProxyOutsideFeature,
    },
    common::x_outside_feature::feature::{CommonOutsideService, CoreProxyOutsideFeature},
    x_outside_feature::proxy::env::ProxyEnv,
};

use crate::common::api::jwt::helper::decoding_key_from_ec_pem;

use crate::common::api::logger::infra::LogOutputLevel;

pub struct ProxyAppFeature {
    pub log_level: LogOutputLevel,
    pub auth: AuthProxyOutsideFeature,
    pub core: CoreProxyOutsideFeature,
}

impl ProxyAppFeature {
    pub async fn new(env: &'static ProxyEnv) -> Self {
        Self {
            log_level: LogOutputLevel::parse(&env.log_level),
            auth: AuthProxyOutsideFeature {
                service: CommonOutsideService::new(&env.auth_service_url),
                cookie: AuthOutsideCookie {
                    domain: &env.domain,
                    cloudfront_key_pair_id: &env.cloudfront_key_pair_id,
                },
                decoding_key: AuthOutsideDecodingKey {
                    authenticate: decoding_key_from_ec_pem(&env.authenticate_public_key),
                    authorize: decoding_key_from_ec_pem(&env.authorize_public_key),
                },
            },
            core: CoreProxyOutsideFeature {
                service: CommonOutsideService::new(&env.core_service_url),
            },
        }
    }
}

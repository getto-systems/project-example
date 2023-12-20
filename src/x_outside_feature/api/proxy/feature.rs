use std::sync::Arc;

use crate::{
    auth::x_outside_feature::feature::{
        AuthOutsideCookie, AuthOutsideDecodingKey, AuthProxyOutsideFeature,
    },
    common::x_outside_feature::feature::CoreProxyOutsideFeature,
    x_outside_feature::proxy::env::ProxyEnv,
};

use crate::common::api::jwt::helper::decoding_key_from_ec_pem;

pub struct ProxyAppFeature {
    pub auth: AuthProxyOutsideFeature,
    pub core: CoreProxyOutsideFeature,
}

impl ProxyAppFeature {
    pub async fn new(env: &'static ProxyEnv) -> Self {
        Self {
            auth: AuthProxyOutsideFeature {
                service_url: &env.auth_service_url,
                google_authorize_store: Default::default(),
                cookie: AuthOutsideCookie {
                    domain: &env.domain,
                    cloudfront_key_pair_id: &env.cloudfront_key_pair_id,
                },
                decoding_key: AuthOutsideDecodingKey {
                    authenticate: Arc::new(decoding_key_from_ec_pem(&env.authenticate_public_key)),
                    authorize: Arc::new(decoding_key_from_ec_pem(&env.authorize_public_key)),
                },
            },
            core: CoreProxyOutsideFeature {
                service_url: &env.core_service_url,
                google_authorize_store: Default::default(),
            },
        }
    }
}

use std::sync::Arc;

use tonic::Request;

use crate::{
    auth::x_outside_feature::feature::{AuthOutsideAuthorizeKey, AuthServiceOutsideFeature},
    x_outside_feature::core::env::CoreEnv,
};

use crate::common::api::jwt::helper::decoding_key_from_ec_pem;

pub struct CoreAppFeature {
    pub auth: AuthServiceOutsideFeature,
}

impl CoreAppFeature {
    pub fn new(env: &'static CoreEnv) -> Self {
        Self {
            auth: AuthServiceOutsideFeature {
                service_url: &env.auth_service_url,
                google_authorize_store: Default::default(),
                decoding_key: AuthOutsideAuthorizeKey {
                    key: Arc::new(decoding_key_from_ec_pem(&env.authorize_public_key)),
                },
            },
        }
    }

    pub fn from_request<T>(request: &Request<T>) -> Arc<Self> {
        Arc::clone(
            request
                .extensions()
                .get::<Arc<Self>>()
                .expect("failed to get AppFeature"),
        )
    }
}

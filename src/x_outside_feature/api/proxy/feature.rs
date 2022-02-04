use crate::{
    auth::x_outside_feature::api::proxy::{
        feature::AuthOutsideFeature, init::new_auth_outside_feature,
    },
    core::x_outside_feature::api::{
        feature::CoreOutsideFeature, init::new_core_outside_feature,
    },
    x_outside_feature::api::proxy::env::ProxyEnv,
};

pub struct ProxyAppFeature {
    pub auth: AuthOutsideFeature,
    pub core: CoreOutsideFeature,
}

impl ProxyAppFeature {
    pub async fn new(env: &'static ProxyEnv) -> Self {
        Self {
            auth: new_auth_outside_feature(env).await,
            core: new_core_outside_feature(env),
        }
    }
}

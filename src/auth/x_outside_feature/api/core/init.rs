use crate::{
    auth::x_outside_feature::{
        common::feature::AuthOutsideService, core::feature::AuthOutsideFeature,
    },
    x_outside_feature::core::env::CoreEnv,
};

pub async fn new_auth_outside_feature(env: &'static CoreEnv) -> AuthOutsideFeature {
    AuthOutsideFeature {
        service: AuthOutsideService {
            service_url: &env.auth_service_url,
        },
    }
}

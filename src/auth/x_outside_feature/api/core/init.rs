use crate::{
    auth::x_outside_feature::api::{
        common::feature::AuthOutsideService, core::feature::AuthOutsideFeature,
    },
    x_outside_feature::api::example::env::ExampleEnv,
};

pub async fn new_auth_outside_feature(env: &'static ExampleEnv) -> AuthOutsideFeature {
    AuthOutsideFeature {
        service: AuthOutsideService {
            service_url: &env.auth_service_url,
        },
    }
}

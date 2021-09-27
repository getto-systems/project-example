use crate::{
    auth::remote::x_outside_feature::{
        common::feature::AuthOutsideService, example::feature::AuthOutsideFeature,
    },
    x_outside_feature::_example::env::ExampleEnv,
};

pub async fn new_auth_outside_feature(env: &'static ExampleEnv) -> AuthOutsideFeature {
    AuthOutsideFeature {
        service: AuthOutsideService {
            service_url: &env.auth_service_url,
        },
    }
}

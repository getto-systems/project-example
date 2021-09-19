use crate::{
    auth::{
        _common::x_outside_feature::feature::AuthOutsideService,
        _example::x_outside_feature::feature::AuthOutsideFeature,
    },
    x_outside_feature::_example::env::Env,
};

pub async fn new_auth_outside_feature(env: &'static Env) -> AuthOutsideFeature {
    AuthOutsideFeature {
        service: AuthOutsideService {
            service_url: &env.auth_service_url,
        },
    }
}

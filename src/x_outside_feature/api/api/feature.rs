use super::env::ApiEnv;

use crate::{
    auth::x_outside_feature::api::api::{
        feature::AuthOutsideFeature, init::new_auth_outside_feature,
    },
    example::x_outside_feature::api::{
        feature::ExampleOutsideFeature, init::new_example_outside_feature,
    },
};

pub struct ApiAppFeature {
    pub auth: AuthOutsideFeature,
    pub example: ExampleOutsideFeature,
}

impl ApiAppFeature {
    pub async fn new(env: &'static ApiEnv) -> Self {
        Self {
            auth: new_auth_outside_feature(env).await,
            example: new_example_outside_feature(env),
        }
    }
}

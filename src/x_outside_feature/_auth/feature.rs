use std::sync::Arc;

use tonic::Request;

use super::env::Env;

use crate::auth::_auth::x_outside_feature::{
    feature::AuthOutsideFeature, init::new_auth_outside_feature,
};

pub type AppData = Arc<AppFeature>;

pub struct AppFeature {
    pub auth: AuthOutsideFeature,
}

impl AppFeature {
    pub async fn new(env: &'static Env) -> Self {
        Self {
            auth: new_auth_outside_feature(env).await,
        }
    }
}

pub fn app_data<T>(request: &Request<T>) -> &AppData {
    request
        .extensions()
        .get::<AppData>()
        .expect("failed to get AppFeature")
}

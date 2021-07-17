use actix_web::web;

use super::env::Env;

use crate::auth::_api::x_outside_feature::{
    feature::AuthOutsideFeature, init::new_auth_outside_feature,
};

pub type AppData = web::Data<AppFeature>;

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

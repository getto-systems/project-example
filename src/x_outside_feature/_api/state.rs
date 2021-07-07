use actix_web::web;

use super::env::Env;

use crate::auth::_api::x_outside_feature::{
    feature::AuthOutsideFeature, init::new_auth_outside_feature,
};

pub type AppData = web::Data<AppState>;

pub struct AppState {
    pub auth: AuthOutsideFeature,
}

impl AppState {
    pub async fn new(env: &'static Env) -> Self {
        Self {
            auth: new_auth_outside_feature(env).await,
        }
    }
}

impl Into<AppData> for AppState {
    fn into(self) -> AppData {
        web::Data::new(self)
    }
}

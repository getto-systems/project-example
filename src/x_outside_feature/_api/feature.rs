use actix_web::web::Data;

use super::env::Env;

use crate::{
    auth::_api::x_outside_feature::{feature::AuthOutsideFeature, init::new_auth_outside_feature},
    outline::_api::x_outside_feature::{
        feature::OutlineOutsideFeature, init::new_outline_outside_feature,
    },
};

pub type AppData = Data<AppFeature>;

pub struct AppFeature {
    pub auth: AuthOutsideFeature,
    pub outline: OutlineOutsideFeature,
}

impl AppFeature {
    pub async fn new(env: &'static Env) -> Self {
        Self {
            auth: new_auth_outside_feature(env).await,
            outline: new_outline_outside_feature(env),
        }
    }
}

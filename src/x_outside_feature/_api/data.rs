use actix_web::web;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

pub type AppData = web::Data<AppState>;

pub struct AppState {
    pub auth: AuthOutsideFeature,
}

pub struct AppSetting {
    pub origin: String,
    pub port: String,
}

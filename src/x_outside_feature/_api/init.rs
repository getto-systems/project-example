use crate::auth::_api::x_outside_feature::init::new_auth_outside_feature;

use super::{
    data::{AppSetting, AppState},
    env::Env,
    secret::EnvSecret,
};

pub fn new_app_state() -> (AppState, AppSetting) {
    let env = Env::new();
    let secret = EnvSecret::new();

    (
        AppState {
            auth: new_auth_outside_feature(&env, &secret),
        },
        AppSetting {
            origin: env.load("ORIGIN"),
            port: env.load("PORT"),
        },
    )
}

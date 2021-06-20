use actix_web::{post, web, HttpRequest, Responder, Scope};

use getto_application::helper::flatten;

use crate::x_outside_feature::_api::{
    state::AppData,
    logger::{app_logger, Logger},
};

use crate::auth::password::reset::_api::x_actix_web::route::scope_reset;

use crate::auth::password::_api::action_authenticate::action::AuthenticatePasswordAction;

pub fn scope_password() -> Scope {
    web::scope("/password")
        .service(scope_reset())
        .service(authenticate)
}

#[post("/authenticate")]
async fn authenticate(data: AppData, request: HttpRequest, body: String) -> impl Responder {
    let logger = app_logger(&request);
    let mut action = AuthenticatePasswordAction::new(&data.auth, &request, body);
    action.subscribe(move |state| logger.log(state.log_level(), state));
    flatten(action.ignite()).respond_to(&request)
}

use actix_web::{post, web, HttpRequest, Responder, Scope};

use getto_application::helper::flatten;

use crate::x_outside_feature::_api::{
    logger::{app_logger, Logger},
    state::AppData,
};

use crate::auth::password::reset::_api::{
    action_request_token::action::RequestResetTokenAction,
    action_reset::action::ResetPasswordAction,
};

pub fn scope_reset() -> Scope {
    web::scope("/reset").service(request_token).service(reset)
}

#[post("/token")]
async fn request_token(data: AppData, request: HttpRequest, body: String) -> impl Responder {
    let logger = app_logger(&request);
    let mut action = RequestResetTokenAction::new(&data.auth, &request, body);
    action.subscribe(move |state| logger.log(state.log_level(), state));
    flatten(action.ignite().await).respond_to(&request)
}

#[post("")]
async fn reset(data: AppData, request: HttpRequest, body: String) -> impl Responder {
    let logger = app_logger(&request);
    let mut action = ResetPasswordAction::new(&data.auth, &request, body);
    action.subscribe(move |state| logger.log(state.log_level(), state));
    flatten(action.ignite()).respond_to(&request)
}

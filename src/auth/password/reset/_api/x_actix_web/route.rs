use actix_web::{post, web, HttpRequest, Responder, Scope};

use getto_application::helper::flatten;

use crate::z_details::_common::{logger::Logger, response::actix_web::RespondTo};

use crate::x_outside_feature::_api::{
    feature::AppData,
    logger::{app_logger, generate_request_id},
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
    let request_id = generate_request_id();
    let logger = app_logger(request_id.clone(), &request);
    let mut action = RequestResetTokenAction::new(&data.auth, &request_id, &request, body);
    action.subscribe(move |state| logger.log(state.log_level(), state));
    flatten(action.ignite().await).respond_to(&request)
}

#[post("")]
async fn reset(data: AppData, request: HttpRequest, body: String) -> impl Responder {
    let request_id = generate_request_id();
    let logger = app_logger(request_id.clone(), &request);
    let mut action = ResetPasswordAction::new(&data.auth, &request_id, &request, body);
    action.subscribe(move |state| logger.log(state.log_level(), state));
    flatten(action.ignite().await).respond_to(&request)
}

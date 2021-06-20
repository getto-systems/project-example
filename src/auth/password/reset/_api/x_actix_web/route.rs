use actix_web::{post, web, HttpRequest, Responder, Scope};

use getto_application::helper::flatten;

use crate::{
    auth::password::reset::_api::action_request_token::action::RequestResetTokenAction,
    x_outside_feature::_api::{
        state::AppData,
        logger::{app_logger, Logger},
    },
};

pub fn scope_reset() -> Scope {
    web::scope("/reset").service(request_token)
}

#[post("/token")]
async fn request_token(data: AppData, request: HttpRequest, body: String) -> impl Responder {
    let logger = app_logger(&request);
    let mut action = RequestResetTokenAction::new(&data.auth, &request, body);
    action.subscribe(move |state| logger.log(state.log_level(), state));
    flatten(action.ignite().await).respond_to(&request)
}

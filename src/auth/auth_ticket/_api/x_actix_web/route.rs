use actix_web::{delete, patch, web, HttpRequest, Responder, Scope};

use getto_application::helper::flatten;

use crate::z_details::_common::{logger::Logger, response::actix_web::RespondTo};

use crate::x_outside_feature::_api::{
    feature::AppData,
    logger::{app_logger, generate_request_id},
};

use crate::auth::auth_ticket::_api::{
    action_logout::init::LogoutFeature, action_renew::init::RenewAuthTicketFeature,
};

pub fn scope_auth_ticket() -> Scope {
    web::scope("/auth-ticket").service(renew).service(logout)
}

#[patch("")]
async fn renew(data: AppData, request: HttpRequest) -> impl Responder {
    let request_id = generate_request_id();
    let logger = app_logger(request_id.clone(), &request);
    let mut action = RenewAuthTicketFeature::action(&data.auth, &request_id, &request);
    action.subscribe(move |state| logger.log(state.log_level(), state));

    flatten(action.ignite().await).respond_to(&request)
}

#[delete("")]
async fn logout(data: AppData, request: HttpRequest) -> impl Responder {
    let request_id = generate_request_id();
    let logger = app_logger(request_id.clone(), &request);
    let mut action = LogoutFeature::action(&data.auth, &request_id, &request);
    action.subscribe(move |state| logger.log(state.log_level(), state));

    flatten(action.ignite().await).respond_to(&request)
}

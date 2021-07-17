use actix_web::{delete, patch, web, HttpRequest, Responder, Scope};

use getto_application::helper::flatten;

use crate::z_details::_api::logger::Logger;

use crate::x_outside_feature::_api::{
    logger::{app_logger, request_id},
    feature::AppData,
};

use crate::{
    auth::auth_ticket::_api::action_logout::action::LogoutAction,
    auth::auth_ticket::_api::action_renew::action::RenewAuthTicketAction,
};

pub fn scope_auth_ticket() -> Scope {
    web::scope("/auth-ticket").service(renew).service(logout)
}

#[patch("")]
async fn renew(data: AppData, request: HttpRequest) -> impl Responder {
    let logger = app_logger(request_id(), &request);
    let mut action = RenewAuthTicketAction::new(&data.auth, &request);
    action.subscribe(move |state| logger.log(state.log_level(), state));
    flatten(action.ignite().await).respond_to(&request)
}

#[delete("")]
async fn logout(data: AppData, request: HttpRequest) -> impl Responder {
    let logger = app_logger(request_id(), &request);
    let mut action = LogoutAction::new(&data.auth, &request);
    action.subscribe(move |state| logger.log(state.log_level(), state));
    flatten(action.ignite().await).respond_to(&request)
}

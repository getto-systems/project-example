use actix_web::{delete, patch, web, HttpRequest, Responder, Scope};

use getto_application::helper::flatten;

use crate::auth::_api::proxy::call_proxy;
use crate::z_details::_common::{logger::Logger, response::actix_web::RespondTo};

use crate::x_outside_feature::_api::{
    feature::AppData,
    logger::{app_logger, generate_request_id},
};

use crate::auth::auth_ticket::_api::action_renew::init::RenewAuthTicketFeature;
use crate::auth::auth_ticket::_api::proxy_logout::init::LogoutProxyFeature;

pub fn scope_auth_ticket() -> Scope {
    web::scope("/auth-ticket").service(renew).service(logout)
}

#[patch("")]
async fn renew(data: AppData, request: HttpRequest) -> impl Responder {
    let request_id = generate_request_id();
    let logger = app_logger(request_id.clone(), &request);

    let mut action = RenewAuthTicketFeature::action(&data, &request_id, &request);
    action.subscribe(move |state| logger.log(state.log_level(), state));

    flatten(action.ignite().await).respond_to(&request)
}

#[delete("")]
async fn logout(data: AppData, request: HttpRequest) -> impl Responder {
    let request_id = generate_request_id();
    let logger = app_logger(request_id.clone(), &request);

    let mut material = LogoutProxyFeature::new(&data.auth, &request_id, &request);
    material.subscribe(move |state| logger.log(state.log_level(), state));

    flatten(call_proxy(&material).await).respond_to(&request)
}

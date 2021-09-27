use actix_web::{delete, patch, web, HttpRequest, Responder, Scope};

use getto_application::helper::flatten;

use crate::z_details::_common::{logger::Logger, response::actix_web::RespondTo};

use crate::x_outside_feature::_api::{
    feature::ApiAppData,
    logger::{app_logger, generate_request_id},
};

use crate::auth::remote::service::proxy::call_proxy;

use crate::auth::ticket::remote::{
    proxy_logout::init::LogoutProxyStruct, proxy_renew::init::RenewAuthTicketProxyStruct,
};

pub fn scope_ticket() -> Scope {
    web::scope("/ticket").service(renew).service(logout)
}

#[patch("")]
async fn renew(data: ApiAppData, request: HttpRequest) -> impl Responder {
    let request_id = generate_request_id();
    let logger = app_logger(request_id.clone(), &request);

    let mut proxy = RenewAuthTicketProxyStruct::new(&data.auth, &request_id, &request);
    proxy.subscribe(move |state| logger.log(state.log_level(), state));

    flatten(call_proxy(&proxy, Ok(())).await).respond_to(&request)
}

#[delete("")]
async fn logout(data: ApiAppData, request: HttpRequest) -> impl Responder {
    let request_id = generate_request_id();
    let logger = app_logger(request_id.clone(), &request);

    let mut proxy = LogoutProxyStruct::new(&data.auth, &request_id, &request);
    proxy.subscribe(move |state| logger.log(state.log_level(), state));

    flatten(call_proxy(&proxy, Ok(())).await).respond_to(&request)
}

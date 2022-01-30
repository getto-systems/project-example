use actix_web::{
    delete, patch,
    web::{scope, Data},
    HttpRequest, Responder, Scope,
};

use getto_application::helper::flatten;

use crate::x_outside_feature::remote::api::feature::ApiAppFeature;
use crate::z_lib::remote::{logger::Logger, response::actix_web::RespondTo};

use crate::x_outside_feature::remote::api::logger::{app_logger, generate_request_id};

use crate::auth::ticket::remote::{
    check::proxy::init::CheckAuthTicketProxyStruct, logout::proxy::init::LogoutProxyStruct,
};

pub fn scope_ticket() -> Scope {
    scope("/ticket").service(check).service(logout)
}

#[patch("")]
async fn check(feature: Data<ApiAppFeature>, request: HttpRequest) -> impl Responder {
    let request_id = generate_request_id();
    let logger = app_logger(request_id.clone(), &request);

    let mut action = CheckAuthTicketProxyStruct::action(&feature.auth, &request_id, &request);
    action.subscribe(move |state| logger.log(state));

    flatten(action.ignite().await).respond_to(&request)
}

#[delete("")]
async fn logout(feature: Data<ApiAppFeature>, request: HttpRequest) -> impl Responder {
    let request_id = generate_request_id();
    let logger = app_logger(request_id.clone(), &request);

    let mut action = LogoutProxyStruct::action(&feature.auth, &request_id, &request);
    action.subscribe(move |state| logger.log(state));

    flatten(action.ignite().await).respond_to(&request)
}

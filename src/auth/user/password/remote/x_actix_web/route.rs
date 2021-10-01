use actix_web::{post, web, HttpRequest, Responder, Scope};

use getto_application::helper::flatten;

use crate::z_lib::remote::{logger::Logger, response::actix_web::RespondTo};

use crate::x_outside_feature::remote::api::{
    feature::ApiAppData,
    logger::{app_logger, generate_request_id},
};

use crate::auth::user::password::reset::remote::x_actix_web::route::scope_reset;

use crate::auth::remote::service::proxy::call_proxy;

use crate::auth::user::password::remote::{
    proxy_authenticate::{
        infra::AuthenticatePasswordProxyRequestDecoder, init::AuthenticatePasswordProxyStruct,
    },
    proxy_change::{infra::ChangePasswordProxyRequestDecoder, init::ChangePasswordProxyStruct},
};

pub fn scope_password() -> Scope {
    web::scope("/password")
        .service(scope_reset())
        .service(authenticate)
        .service(change)
}

#[post("/authenticate")]
async fn authenticate(data: ApiAppData, request: HttpRequest, body: String) -> impl Responder {
    let request_id = generate_request_id();
    let logger = app_logger(request_id.clone(), &request);

    let mut proxy = AuthenticatePasswordProxyStruct::new(&data.auth, &request_id, &request);
    proxy.subscribe(move |state| logger.log(state.log_level(), state));

    let params = AuthenticatePasswordProxyStruct::request_decoder(body).decode();
    flatten(call_proxy(&proxy, params).await).respond_to(&request)
}

#[post("/change")]
async fn change(data: ApiAppData, request: HttpRequest, body: String) -> impl Responder {
    let request_id = generate_request_id();
    let logger = app_logger(request_id.clone(), &request);

    let mut proxy = ChangePasswordProxyStruct::new(&data.auth, &request_id, &request);
    proxy.subscribe(move |state| logger.log(state.log_level(), state));

    let params = ChangePasswordProxyStruct::request_decoder(body).decode();
    flatten(call_proxy(&proxy, params).await).respond_to(&request)
}
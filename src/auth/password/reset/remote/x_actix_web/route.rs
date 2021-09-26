use actix_web::{post, web, HttpRequest, Responder, Scope};

use getto_application::helper::flatten;

use crate::z_details::_common::{logger::Logger, response::actix_web::RespondTo};

use crate::x_outside_feature::_api::{
    feature::ApiAppData,
    logger::{app_logger, generate_request_id},
};

use crate::auth::_api::proxy::call_proxy;

use crate::auth::password::reset::remote::{
    proxy_request_token::{
        infra::RequestResetTokenProxyRequestDecoder, init::RequestResetTokenProxyStruct,
    },
    proxy_reset::{infra::ResetPasswordProxyRequestDecoder, init::ResetPasswordProxyStruct},
};

pub fn scope_reset() -> Scope {
    web::scope("/reset").service(request_token).service(reset)
}

#[post("/token")]
async fn request_token(data: ApiAppData, request: HttpRequest, body: String) -> impl Responder {
    let request_id = generate_request_id();
    let logger = app_logger(request_id.clone(), &request);

    let mut proxy = RequestResetTokenProxyStruct::new(&data.auth, &request_id, &request);
    proxy.subscribe(move |state| logger.log(state.log_level(), state));

    let params = RequestResetTokenProxyStruct::request_decoder(body).decode();
    flatten(call_proxy(&proxy, params).await).respond_to(&request)
}

#[post("")]
async fn reset(data: ApiAppData, request: HttpRequest, body: String) -> impl Responder {
    let request_id = generate_request_id();
    let logger = app_logger(request_id.clone(), &request);

    let mut proxy = ResetPasswordProxyStruct::new(&data.auth, &request_id, &request);
    proxy.subscribe(move |state| logger.log(state.log_level(), state));

    let params = ResetPasswordProxyStruct::request_decoder(body).decode();
    flatten(call_proxy(&proxy, params).await).respond_to(&request)
}

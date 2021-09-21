use actix_web::{post, web, HttpRequest, Responder, Scope};

use getto_application::helper::flatten;

use crate::z_details::_common::{logger::Logger, response::actix_web::RespondTo};

use crate::x_outside_feature::_api::{
    feature::AppData,
    logger::{app_logger, generate_request_id},
};

use crate::auth::_api::proxy::call_proxy;

use crate::auth::password::reset::_api::{
    proxy_request_token::{
        infra::RequestResetTokenProxyRequestDecoder, init::RequestResetTokenProxyFeature,
    },
    proxy_reset::{infra::ResetPasswordProxyRequestDecoder, init::ResetPasswordProxyFeature},
};

pub fn scope_reset() -> Scope {
    web::scope("/reset").service(request_token).service(reset)
}

#[post("/token")]
async fn request_token(data: AppData, request: HttpRequest, body: String) -> impl Responder {
    let request_id = generate_request_id();
    let logger = app_logger(request_id.clone(), &request);

    let mut material = RequestResetTokenProxyFeature::new(&data.auth, &request_id, &request);
    material.subscribe(move |state| logger.log(state.log_level(), state));

    let params = RequestResetTokenProxyFeature::request_decoder(body).decode();
    flatten(call_proxy(&material, params).await).respond_to(&request)
}

#[post("")]
async fn reset(data: AppData, request: HttpRequest, body: String) -> impl Responder {
    let request_id = generate_request_id();
    let logger = app_logger(request_id.clone(), &request);

    let mut material = ResetPasswordProxyFeature::new(&data.auth, &request_id, &request);
    material.subscribe(move |state| logger.log(state.log_level(), state));

    let params = ResetPasswordProxyFeature::request_decoder(body).decode();
    flatten(call_proxy(&material, params).await).respond_to(&request)
}

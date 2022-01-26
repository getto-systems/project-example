use actix_web::{
    post,
    web::{scope, Data},
    HttpRequest, Responder, Scope,
};

use getto_application::helper::flatten;

use crate::z_lib::remote::{logger::Logger, response::actix_web::RespondTo};

use crate::x_outside_feature::remote::api::{
    feature::ApiAppFeature,
    logger::{app_logger, generate_request_id},
};

use crate::auth::remote::service::proxy::call_proxy;

use crate::auth::user::password::reset::remote::{
    request_token::proxy::{
        infra::RequestResetTokenProxyRequestDecoder, init::RequestResetTokenProxyStruct,
    },
    reset::proxy::{infra::ResetPasswordProxyRequestDecoder, init::ResetPasswordProxyStruct},
};

pub fn scope_reset() -> Scope {
    scope("/reset").service(request_token).service(reset)
}

#[post("/token")]
async fn request_token(
    feature: Data<ApiAppFeature>,
    request: HttpRequest,
    body: String,
) -> impl Responder {
    let request_id = generate_request_id();
    let logger = app_logger(request_id.clone(), &request);

    let mut proxy = RequestResetTokenProxyStruct::new(&feature.auth, &request_id, &request);
    proxy.subscribe(move |state| logger.log(state.log_level(), state));

    let params = RequestResetTokenProxyStruct::request_decoder(body).decode();
    flatten(call_proxy(&proxy, params).await).respond_to(&request)
}

#[post("")]
async fn reset(feature: Data<ApiAppFeature>, request: HttpRequest, body: String) -> impl Responder {
    let request_id = generate_request_id();
    let logger = app_logger(request_id.clone(), &request);

    let mut proxy = ResetPasswordProxyStruct::new(&feature.auth, &request_id, &request);
    proxy.subscribe(move |state| logger.log(state.log_level(), state));

    let params = ResetPasswordProxyStruct::request_decoder(body).decode();
    flatten(call_proxy(&proxy, params).await).respond_to(&request)
}

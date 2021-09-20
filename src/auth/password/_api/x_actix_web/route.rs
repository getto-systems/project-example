use actix_web::{post, web, HttpRequest, Responder, Scope};

use getto_application::helper::flatten;

use crate::auth::_api::proxy::call_proxy;
use crate::auth::password::_api::proxy_authenticate::init::AuthenticatePasswordProxyFeature;
use crate::z_details::_common::{logger::Logger, response::actix_web::RespondTo};

use crate::x_outside_feature::_api::{
    feature::AppData,
    logger::{app_logger, generate_request_id},
};

use crate::auth::password::reset::_api::x_actix_web::route::scope_reset;

use crate::auth::password::_api::{
    action_change::init::ChangePasswordFeature,
    proxy_authenticate::init::AuthenticatePasswordRequestDecoder,
};

pub fn scope_password() -> Scope {
    web::scope("/password")
        .service(scope_reset())
        .service(authenticate)
        .service(change)
}

#[post("/authenticate")]
async fn authenticate(data: AppData, request: HttpRequest, body: String) -> impl Responder {
    let request_id = generate_request_id();
    let logger = app_logger(request_id.clone(), &request);

    let mut material = AuthenticatePasswordProxyFeature::new(&data.auth, &request_id, &request);
    material.subscribe(move |state| logger.log(state.log_level(), state));

    let params = AuthenticatePasswordProxyFeature::request_decoder(body).decode();
    flatten(call_proxy(&material, params).await).respond_to(&request)
}

#[post("/change")]
async fn change(data: AppData, request: HttpRequest, body: String) -> impl Responder {
    let request_id = generate_request_id();
    let logger = app_logger(request_id.clone(), &request);

    let mut action = ChangePasswordFeature::action(&data, &request_id, &request);
    action.subscribe(move |state| logger.log(state.log_level(), state));

    let request_decoder = ChangePasswordFeature::request_decoder(body);
    flatten(action.ignite(request_decoder).await).respond_to(&request)
}

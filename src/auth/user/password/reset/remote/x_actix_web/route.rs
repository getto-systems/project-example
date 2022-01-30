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

use crate::auth::user::password::reset::remote::{
    request_token::proxy::init::RequestResetTokenProxyStruct,
    reset::proxy::init::ResetPasswordProxyStruct,
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

    let mut action =
        RequestResetTokenProxyStruct::action(&feature.auth, &request_id, &request, body);
    action.subscribe(move |state| logger.log(state.log_level(), state));

    flatten(action.ignite().await).respond_to(&request)
}

#[post("")]
async fn reset(feature: Data<ApiAppFeature>, request: HttpRequest, body: String) -> impl Responder {
    let request_id = generate_request_id();
    let logger = app_logger(request_id.clone(), &request);

    let mut action = ResetPasswordProxyStruct::action(&feature.auth, &request_id, &request, body);
    action.subscribe(move |state| logger.log(state.log_level(), state));

    flatten(action.ignite().await).respond_to(&request)
}

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

use crate::auth::user::password::reset::request_token::remote::x_actix_web::route::service_request_token;

use crate::auth::user::password::reset::remote::reset::proxy::init::ResetPasswordProxyStruct;

pub fn scope_reset() -> Scope {
    scope("/reset")
        .service(service_request_token)
        .service(reset)
}

#[post("")]
async fn reset(feature: Data<ApiAppFeature>, request: HttpRequest, body: String) -> impl Responder {
    let request_id = generate_request_id();
    let logger = app_logger(request_id.clone(), &request);

    let mut action = ResetPasswordProxyStruct::action(&feature.auth, &request_id, &request, body);
    action.subscribe(move |state| logger.log(state));

    flatten(action.ignite().await).respond_to(&request)
}

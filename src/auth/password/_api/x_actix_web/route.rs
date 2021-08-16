use actix_web::{post, web, HttpRequest, Responder, Scope};

use getto_application::helper::flatten;

use crate::z_details::_common::{logger::Logger, response::actix_web::RespondTo};

use crate::x_outside_feature::_api::{
    feature::AppData,
    logger::{app_logger, generate_request_id},
};

use crate::auth::password::reset::_api::x_actix_web::route::scope_reset;

use crate::auth::password::_api::action_authenticate::init::AuthenticatePasswordFeature;

pub fn scope_password() -> Scope {
    web::scope("/password")
        .service(scope_reset())
        .service(authenticate)
}

#[post("/authenticate")]
async fn authenticate(data: AppData, request: HttpRequest, body: String) -> impl Responder {
    let request_id = generate_request_id();
    let logger = app_logger(request_id.clone(), &request);
    let mut action = AuthenticatePasswordFeature::action(&data.auth, &request_id, &request);
    action.subscribe(move |state| logger.log(state.log_level(), state));

    let request_decoder = AuthenticatePasswordFeature::request_decoder(body);
    flatten(action.ignite(request_decoder).await).respond_to(&request)
}

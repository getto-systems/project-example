use actix_web::{post, web::Data, HttpRequest, Responder};

use getto_application::helper::flatten;

use crate::z_lib::api::{logger::Logger, response::actix_web::ProxyResponder};

use crate::x_outside_feature::api::proxy::{
    feature::ProxyAppFeature,
    logger::{app_logger, generate_request_id},
};

use crate::auth::user::password::authenticate::api::proxy::init::AuthenticatePasswordProxyStruct;

#[post("/authenticate")]
async fn service_authenticate(
    feature: Data<ProxyAppFeature>,
    request: HttpRequest,
    body: String,
) -> impl Responder {
    let request_id = generate_request_id();
    let logger = app_logger(request_id.clone(), &request);

    let mut action =
        AuthenticatePasswordProxyStruct::action(&feature.auth, &request_id, &request, body);
    action.subscribe(move |state| logger.log(state));

    flatten(action.ignite().await).respond_to()
}

use actix_web::{post, web::Data, HttpRequest, Responder};

use getto_application::helper::flatten;

use crate::x_outside_feature::proxy::{
    feature::ProxyAppFeature,
    logger::{app_logger, generate_request_id},
};

use crate::auth::user::account::register::proxy::init::RegisterAuthUserAccountProxyStruct;

use crate::z_lib::{logger::infra::Logger, response::actix_web::ProxyResponder};

#[post("")]
pub async fn service_register_user(
    feature: Data<ProxyAppFeature>,
    request: HttpRequest,
    body: String,
) -> impl Responder {
    let request_id = generate_request_id();
    let logger = app_logger(request_id.clone(), &request);

    let mut action =
        RegisterAuthUserAccountProxyStruct::action(&feature.auth, &request_id, &request, body);
    action.subscribe(move |state| logger.log(state));

    flatten(action.ignite().await).respond_to()
}

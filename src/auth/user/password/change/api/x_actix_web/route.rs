use actix_web::{patch, web::Data, HttpRequest, Responder};

use getto_application::helper::flatten;

use crate::{
    auth::user::password::change::proxy::init::OverwritePasswordProxyStruct,
    x_outside_feature::proxy::{
        feature::ProxyAppFeature,
        logger::{app_logger, generate_request_id},
    },
};

use crate::auth::user::password::change::proxy::init::ChangePasswordProxyStruct;

use crate::z_lib::{logger::infra::Logger, response::actix_web::ProxyResponder};

#[patch("")]
pub async fn service_change_password(
    feature: Data<ProxyAppFeature>,
    request: HttpRequest,
    body: String,
) -> impl Responder {
    let request_id = generate_request_id();
    let logger = app_logger(request_id.clone(), &request);

    let mut action = ChangePasswordProxyStruct::action(&feature.auth, &request_id, &request, body);
    action.subscribe(move |state| logger.log(state));

    flatten(action.ignite().await).respond_to()
}

#[patch("/overwrite")]
pub async fn service_overwrite_password(
    feature: Data<ProxyAppFeature>,
    request: HttpRequest,
    body: String,
) -> impl Responder {
    let request_id = generate_request_id();
    let logger = app_logger(request_id.clone(), &request);

    let mut action =
        OverwritePasswordProxyStruct::action(&feature.auth, &request_id, &request, body);
    action.subscribe(move |state| logger.log(state));

    flatten(action.ignite().await).respond_to()
}

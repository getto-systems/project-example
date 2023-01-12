use actix_web::{patch, web::Data, HttpRequest, Responder};

use getto_application::helper::flatten;

use crate::{
    auth::user::password::change::proxy::init::ActiveOverwritePasswordProxyMaterial,
    x_outside_feature::proxy::{feature::ProxyAppFeature, logger::ProxyLogger},
};

use crate::auth::user::password::change::proxy::init::ActiveChangePasswordProxyMaterial;

use crate::common::api::{logger::infra::Logger, response::actix_web::ProxyResponder};

#[patch("")]
pub async fn service_change_password(
    feature: Data<ProxyAppFeature>,
    request: HttpRequest,
    body: String,
) -> impl Responder {
    let (request_id, logger) = ProxyLogger::default(&feature, &request);

    let mut action = ActiveChangePasswordProxyMaterial::action(&feature, request_id);
    action.subscribe(move |state| logger.log(state));

    flatten(action.ignite(&request, body).await).respond_to()
}

#[patch("/overwrite")]
pub async fn service_overwrite_password(
    feature: Data<ProxyAppFeature>,
    request: HttpRequest,
    body: String,
) -> impl Responder {
    let (request_id, logger) = ProxyLogger::default(&feature, &request);

    let mut action = ActiveOverwritePasswordProxyMaterial::action(&feature, request_id);
    action.subscribe(move |state| logger.log(state));

    flatten(action.ignite(&request, body).await).respond_to()
}

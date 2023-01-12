use actix_web::{post, web::Data, HttpRequest, Responder};

use getto_application::helper::flatten;

use crate::x_outside_feature::proxy::{feature::ProxyAppFeature, logger::ProxyLogger};

use crate::avail::unexpected_error::notify::proxy::init::ActiveNotifyUnexpectedErrorProxyMaterial;

use crate::common::api::{logger::infra::Logger, response::actix_web::ProxyResponder};

#[post("")]
async fn service_notify(
    feature: Data<ProxyAppFeature>,
    request: HttpRequest,
    body: String,
) -> impl Responder {
    let (request_id, logger) = ProxyLogger::default(&feature, &request);

    let mut action = ActiveNotifyUnexpectedErrorProxyMaterial::action(&feature, request_id);
    action.subscribe(move |state| logger.log(state));

    flatten(action.ignite(&request, body).await).respond_to()
}

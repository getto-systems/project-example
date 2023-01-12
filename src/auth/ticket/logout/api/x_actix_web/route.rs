use actix_web::{delete, web::Data, HttpRequest, Responder};

use getto_application::helper::flatten;

use crate::x_outside_feature::proxy::{feature::ProxyAppFeature, logger::ProxyLogger};

use crate::auth::ticket::logout::proxy::init::ActiveLogoutProxyMaterial;

use crate::common::api::{logger::infra::Logger, response::actix_web::ProxyResponder};

#[delete("")]
async fn service_logout(feature: Data<ProxyAppFeature>, request: HttpRequest) -> impl Responder {
    let (request_id, logger) = ProxyLogger::default(&feature, &request);

    let mut action = ActiveLogoutProxyMaterial::action(&feature, request_id);
    action.subscribe(move |state| logger.log(state));

    flatten(action.ignite(&request).await).respond_to()
}

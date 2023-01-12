use actix_web::{patch, web::Data, HttpRequest, Responder};

use getto_application::helper::flatten;

use crate::x_outside_feature::proxy::{feature::ProxyAppFeature, logger::ProxyLogger};

use crate::auth::ticket::authenticate::proxy::init::ActiveAuthenticateWithTokenProxyMaterial;

use crate::common::api::{logger::infra::Logger, response::actix_web::ProxyResponder};

#[patch("")]
pub async fn service_authenticate_with_token(
    feature: Data<ProxyAppFeature>,
    request: HttpRequest,
) -> impl Responder {
    let (request_id, logger) = ProxyLogger::default(&feature, &request);

    let mut action = ActiveAuthenticateWithTokenProxyMaterial::action(&feature, request_id);
    action.subscribe(move |state| logger.log(state));

    flatten(action.ignite(&request).await).respond_to()
}

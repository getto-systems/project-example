use actix_web::{patch, web::Data, HttpRequest, Responder};

use getto_application::helper::flatten;

use crate::x_outside_feature::proxy::{feature::ProxyAppFeature, logger::ProxyLogger};

use crate::auth::user::account::modify::proxy::init::ActiveModifyAuthUserAccountProxyMaterial;

use crate::common::api::{logger::infra::Logger, response::actix_web::ProxyResponder};

#[patch("")]
pub async fn service_modify_user(
    feature: Data<ProxyAppFeature>,
    request: HttpRequest,
    body: String,
) -> impl Responder {
    let (request_id, logger) = ProxyLogger::default(&feature, &request);

    let mut action = ActiveModifyAuthUserAccountProxyMaterial::action(&feature, request_id);
    action.subscribe(move |state| logger.log(state));

    flatten(action.ignite(&request, body).await).respond_to()
}

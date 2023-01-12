use actix_web::{get, web::Data, HttpRequest, Responder};

use getto_application::helper::flatten;

use crate::x_outside_feature::proxy::{feature::ProxyAppFeature, logger::ProxyLogger};

use crate::common::outline::load::proxy::init::ActiveGetOutlineMenuBadgeProxyMaterial;

use crate::common::api::{logger::infra::Logger, response::actix_web::ProxyResponder};

#[get("/menu-badge")]
pub async fn service_get_menu_badge(
    feature: Data<ProxyAppFeature>,
    request: HttpRequest,
) -> impl Responder {
    let (request_id, logger) = ProxyLogger::default(&feature, &request);

    let mut action = ActiveGetOutlineMenuBadgeProxyMaterial::action(&feature, request_id);
    action.subscribe(move |state| logger.log(state));

    flatten(action.ignite(&request, ()).await).respond_to()
}

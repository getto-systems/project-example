use actix_web::{get, web::Data, HttpRequest, Responder};

use getto_application::helper::flatten;

use crate::z_lib::{logger::Logger, response::actix_web::ProxyResponder};

use crate::x_outside_feature::proxy::{
    feature::ProxyAppFeature,
    logger::{app_logger, generate_request_id},
};

use crate::core::outline::load::proxy::init::GetOutlineMenuBadgeProxyStruct;

#[get("/menu-badge")]
pub async fn service_get_menu_badge(
    feature: Data<ProxyAppFeature>,
    request: HttpRequest,
) -> impl Responder {
    let request_id = generate_request_id();
    let logger = app_logger(request_id.clone(), &request);

    let mut action = GetOutlineMenuBadgeProxyStruct::action(&feature, &request_id, &request);
    action.subscribe(move |state| logger.log(state));

    flatten(action.ignite().await).respond_to()
}
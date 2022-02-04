use actix_web::{get, web::Data, HttpRequest, Responder};

use getto_application::helper::flatten;

use crate::z_lib::api::{logger::Logger, response::actix_web::RespondTo};

use crate::x_outside_feature::api::api::{
    feature::ApiAppFeature,
    logger::{app_logger, generate_request_id},
};

use crate::example::outline::load::api::proxy::init::GetOutlineMenuBadgeProxyStruct;

#[get("/menu-badge")]
pub async fn service_get_menu_badge(
    feature: Data<ApiAppFeature>,
    request: HttpRequest,
) -> impl Responder {
    let request_id = generate_request_id();
    let logger = app_logger(request_id.clone(), &request);

    let mut action = GetOutlineMenuBadgeProxyStruct::action(&feature, &request_id, &request);
    action.subscribe(move |state| logger.log(state));

    flatten(action.ignite().await).respond_to(&request)
}

use actix_web::{get, web, HttpRequest, Responder, Scope};

use getto_application::helper::flatten;

use crate::z_details::_common::{logger::Logger, response::actix_web::RespondTo};

use crate::x_outside_feature::_api::{
    feature::AppData,
    logger::{app_logger, generate_request_id},
};

use crate::outline::_api::action_get_menu_badge::init::GetOutlineMenuBadgeFeature;

pub fn scope_outline() -> Scope {
    web::scope("/outline").service(get_menu_badge)
}

#[get("/menu-badge")]
async fn get_menu_badge(data: AppData, request: HttpRequest) -> impl Responder {
    let request_id = generate_request_id();
    let logger = app_logger(request_id.clone(), &request);
    let mut action = GetOutlineMenuBadgeFeature::action(&data, &request_id, &request);
    action.subscribe(move |state| logger.log(state.log_level(), state));

    flatten(action.ignite().await).respond_to(&request)
}

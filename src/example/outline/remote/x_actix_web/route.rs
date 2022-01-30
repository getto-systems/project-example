use actix_web::{
    get,
    web::{scope, Data},
    HttpRequest, Responder, Scope,
};

use getto_application::helper::flatten;

use crate::z_lib::remote::{logger::Logger, response::actix_web::RespondTo};

use crate::x_outside_feature::remote::api::{
    feature::ApiAppFeature,
    logger::{app_logger, generate_request_id},
};

use crate::example::outline::remote::get_menu_badge::proxy::init::GetOutlineMenuBadgeProxyStruct;

pub fn scope_outline() -> Scope {
    scope("/outline").service(get_menu_badge)
}

#[get("/menu-badge")]
async fn get_menu_badge(feature: Data<ApiAppFeature>, request: HttpRequest) -> impl Responder {
    let request_id = generate_request_id();
    let logger = app_logger(request_id.clone(), &request);

    let mut action = GetOutlineMenuBadgeProxyStruct::action(&feature, &request_id, &request);
    action.subscribe(move |state| logger.log(state));

    flatten(action.ignite().await).respond_to(&request)
}

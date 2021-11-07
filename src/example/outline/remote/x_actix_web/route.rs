use actix_web::{get, web, HttpRequest, Responder, Scope};

use getto_application::helper::flatten;

use crate::z_lib::remote::{logger::Logger, response::actix_web::RespondTo};

use crate::x_outside_feature::remote::api::{
    feature::ApiAppData,
    logger::{app_logger, generate_request_id},
};

use crate::example::remote::proxy::call_proxy;

use crate::example::outline::remote::proxy_get_menu_badge::init::GetOutlineMenuBadgeProxyStruct;

pub fn scope_outline() -> Scope {
    web::scope("/outline").service(get_menu_badge)
}

#[get("/menu-badge")]
async fn get_menu_badge(data: ApiAppData, request: HttpRequest) -> impl Responder {
    let request_id = generate_request_id();
    let logger = app_logger(request_id.clone(), &request);

    let mut proxy = GetOutlineMenuBadgeProxyStruct::new(&data, &request_id, &request);
    proxy.subscribe(move |state| logger.log(state.log_level(), state));

    flatten(call_proxy(&proxy, Ok(())).await).respond_to(&request)
}

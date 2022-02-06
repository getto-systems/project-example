use actix_web::{patch, web::Data, HttpRequest, Responder};

use getto_application::helper::flatten;

use crate::z_lib::api::{logger::Logger, response::actix_web::ProxyResponder};

use crate::x_outside_feature::api::proxy::{
    feature::ProxyAppFeature,
    logger::{app_logger, generate_request_id},
};

use crate::auth::ticket::check::api::proxy::init::CheckAuthTicketProxyStruct;

#[patch("")]
pub async fn service_check(feature: Data<ProxyAppFeature>, request: HttpRequest) -> impl Responder {
    let request_id = generate_request_id();
    let logger = app_logger(request_id.clone(), &request);

    let mut action = CheckAuthTicketProxyStruct::action(&feature.auth, &request_id, &request);
    action.subscribe(move |state| logger.log(state));

    flatten(action.ignite().await).respond_to()
}

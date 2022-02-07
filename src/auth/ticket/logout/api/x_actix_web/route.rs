use actix_web::{delete, web::Data, HttpRequest, Responder};

use getto_application::helper::flatten;

use crate::z_lib::{logger::Logger, response::actix_web::ProxyResponder};

use crate::x_outside_feature::proxy::{
    feature::ProxyAppFeature,
    logger::{app_logger, generate_request_id},
};

use crate::auth::ticket::logout::proxy::init::LogoutProxyStruct;

#[delete("")]
async fn service_logout(feature: Data<ProxyAppFeature>, request: HttpRequest) -> impl Responder {
    let request_id = generate_request_id();
    let logger = app_logger(request_id.clone(), &request);

    let mut action = LogoutProxyStruct::action(&feature.auth, &request_id, &request);
    action.subscribe(move |state| logger.log(state));

    flatten(action.ignite().await).respond_to()
}

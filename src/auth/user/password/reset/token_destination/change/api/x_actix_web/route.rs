use actix_web::{patch, web::Data, HttpRequest, Responder};

use getto_application::helper::flatten;

use crate::x_outside_feature::proxy::{
    feature::ProxyAppFeature,
    logger::{app_logger, generate_request_id},
};

use crate::auth::user::password::reset::token_destination::change::proxy::init::ChangeResetTokenDestinationProxyStruct;

use crate::z_lib::{logger::infra::Logger, response::actix_web::ProxyResponder};

#[patch("")]
pub async fn service_change_destination(
    feature: Data<ProxyAppFeature>,
    request: HttpRequest,
    body: String,
) -> impl Responder {
    let request_id = generate_request_id();
    let logger = app_logger(request_id.clone(), &request);

    let mut action =
        ChangeResetTokenDestinationProxyStruct::action(&feature.auth, &request_id, &request, body);
    action.subscribe(move |state| logger.log(state));

    flatten(action.ignite().await).respond_to()
}

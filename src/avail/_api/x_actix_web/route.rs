use actix_web::{post, web, HttpRequest, Responder, Scope};

use getto_application::helper::flatten;

use crate::z_details::_common::{logger::Logger, response::actix_web::RespondTo};

use crate::x_outside_feature::_api::{
    feature::AppData,
    logger::{app_logger, generate_request_id},
};

use crate::avail::_api::action_notify_unexpected_error::init::NotifyUnexpectedErrorFeature;

pub fn scope_avail() -> Scope {
    web::scope("/avail").service(notify)
}

#[post("unexpected-error")]
async fn notify(data: AppData, request: HttpRequest, body: String) -> impl Responder {
    let request_id = generate_request_id();
    let logger = app_logger(request_id.clone(), &request);
    let mut action = NotifyUnexpectedErrorFeature::action(&data.auth, &request_id, &request);
    action.subscribe(move |state| logger.log(state.log_level(), state));

    let request_decoder = NotifyUnexpectedErrorFeature::request_decoder(body);
    flatten(action.ignite(request_decoder).await).respond_to(&request)
}

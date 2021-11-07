use actix_web::{post, web, HttpRequest, Responder, Scope};

use getto_application::helper::flatten;

use crate::z_lib::remote::{logger::Logger, response::actix_web::RespondTo};

use crate::x_outside_feature::remote::api::{
    feature::ApiAppData,
    logger::{app_logger, generate_request_id},
};

use crate::example::remote::proxy::call_proxy;

use crate::avail::unexpected_error::remote::proxy_notify::{
    infra::NotifyUnexpectedErrorProxyRequestDecoder, init::NotifyUnexpectedErrorProxyStruct,
};

pub fn scope_unexpected_error() -> Scope {
    web::scope("/unexpected-error").service(notify)
}

#[post("")]
async fn notify(data: ApiAppData, request: HttpRequest, body: String) -> impl Responder {
    let request_id = generate_request_id();
    let logger = app_logger(request_id.clone(), &request);

    let mut proxy = NotifyUnexpectedErrorProxyStruct::new(&data, &request_id, &request);
    proxy.subscribe(move |state| logger.log(state.log_level(), state));

    let params = NotifyUnexpectedErrorProxyStruct::request_decoder(body).decode();
    flatten(call_proxy(&proxy, params).await).respond_to(&request)
}

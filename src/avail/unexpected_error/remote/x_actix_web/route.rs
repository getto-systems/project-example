use actix_web::{
    post,
    web::{scope, Data},
    HttpRequest, Responder, Scope,
};

use getto_application::helper::flatten;

use crate::z_lib::remote::{logger::Logger, response::actix_web::RespondTo};

use crate::x_outside_feature::remote::api::{
    feature::ApiAppFeature,
    logger::{app_logger, generate_request_id},
};

use crate::example::remote::proxy::call_proxy;

use crate::avail::unexpected_error::remote::notify::proxy::{
    infra::NotifyUnexpectedErrorProxyRequestDecoder, init::NotifyUnexpectedErrorProxyStruct,
};

pub fn scope_unexpected_error() -> Scope {
    scope("/unexpected-error").service(notify)
}

#[post("")]
async fn notify(
    feature: Data<ApiAppFeature>,
    request: HttpRequest,
    body: String,
) -> impl Responder {
    let request_id = generate_request_id();
    let logger = app_logger(request_id.clone(), &request);

    let mut proxy = NotifyUnexpectedErrorProxyStruct::new(&feature, &request_id, &request);
    proxy.subscribe(move |state| logger.log(state));

    let params = NotifyUnexpectedErrorProxyStruct::request_decoder(body).decode();
    flatten(call_proxy(&proxy, params).await).respond_to(&request)
}

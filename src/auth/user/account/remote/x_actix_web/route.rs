use actix_web::{get, web, HttpRequest, Responder, Scope};

use getto_application::helper::flatten;

use crate::z_lib::remote::{logger::Logger, response::actix_web::RespondTo};

use crate::x_outside_feature::remote::api::{
    feature::ApiAppData,
    logger::{app_logger, generate_request_id},
};

use crate::auth::remote::service::proxy::call_proxy;

use crate::auth::user::account::remote::search::proxy::{
    infra::SearchAuthUserAccountProxyRequestDecoder, init::SearchAuthUserAccountProxyStruct,
};

pub fn scope_account() -> Scope {
    web::scope("/account").service(search)
}

#[get("/search/{body}")]
async fn search(data: ApiAppData, request: HttpRequest, info: web::Path<String>) -> impl Responder {
    let request_id = generate_request_id();
    let logger = app_logger(request_id.clone(), &request);

    let mut proxy = SearchAuthUserAccountProxyStruct::new(&data.auth, &request_id, &request);
    proxy.subscribe(move |state| logger.log(state.log_level(), state));

    let params = SearchAuthUserAccountProxyStruct::request_decoder(info.into_inner()).decode();
    flatten(call_proxy(&proxy, params).await).respond_to(&request)
}

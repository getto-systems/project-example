use actix_web::{
    get,
    web::{Data, Path},
    HttpRequest, Responder,
};

use getto_application::helper::flatten;

use crate::x_outside_feature::proxy::{feature::ProxyAppFeature, logger::ProxyLogger};

use crate::auth::user::account::search::proxy::init::ActiveSearchAuthUserAccountProxyMaterial;

use crate::common::api::{logger::infra::Logger, response::actix_web::ProxyResponder};

#[get("/search/{body}")]
async fn service_search(
    feature: Data<ProxyAppFeature>,
    request: HttpRequest,
    info: Path<String>,
) -> impl Responder {
    let (request_id, logger) = ProxyLogger::default(&feature, &request);

    let mut action = ActiveSearchAuthUserAccountProxyMaterial::action(&feature, request_id);
    action.subscribe(move |state| logger.log(state));

    flatten(action.ignite(&request, info.into_inner()).await).respond_to()
}

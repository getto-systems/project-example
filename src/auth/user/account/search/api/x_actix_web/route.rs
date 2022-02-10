use actix_web::{
    get,
    web::{Data, Path},
    HttpRequest, Responder,
};

use getto_application::helper::flatten;

use crate::x_outside_feature::proxy::{
    feature::ProxyAppFeature,
    logger::{app_logger, generate_request_id},
};

use crate::auth::user::account::search::proxy::init::SearchAuthUserAccountProxyStruct;

use crate::z_lib::{logger::infra::Logger, response::actix_web::ProxyResponder};

#[get("/search/{body}")]
async fn service_search(
    feature: Data<ProxyAppFeature>,
    request: HttpRequest,
    info: Path<String>,
) -> impl Responder {
    let request_id = generate_request_id();
    let logger = app_logger(request_id.clone(), &request);

    let mut action = SearchAuthUserAccountProxyStruct::action(
        &feature.auth,
        &request_id,
        &request,
        info.into_inner(),
    );
    action.subscribe(move |state| logger.log(state));

    flatten(action.ignite().await).respond_to()
}

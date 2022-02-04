use actix_web::{
    get,
    web::{Data, Path},
    HttpRequest, Responder,
};

use getto_application::helper::flatten;

use crate::z_lib::api::{logger::Logger, response::actix_web::RespondTo};

use crate::x_outside_feature::api::api::{
    feature::ApiAppFeature,
    logger::{app_logger, generate_request_id},
};

use crate::auth::user::account::search::api::proxy::init::SearchAuthUserAccountProxyStruct;

#[get("/search/{body}")]
async fn service_search(
    feature: Data<ApiAppFeature>,
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

    flatten(action.ignite().await).respond_to(&request)
}

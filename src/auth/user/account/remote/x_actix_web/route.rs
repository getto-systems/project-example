use actix_web::{
    get,
    web::{scope, Data, Path},
    HttpRequest, Responder, Scope,
};

use getto_application::helper::flatten;

use crate::z_lib::remote::{logger::Logger, response::actix_web::RespondTo};

use crate::x_outside_feature::remote::api::{
    feature::ApiAppFeature,
    logger::{app_logger, generate_request_id},
};

use crate::auth::user::account::remote::search::proxy::init::SearchAuthUserAccountProxyStruct;

pub fn scope_account() -> Scope {
    scope("/account").service(search)
}

#[get("/search/{body}")]
async fn search(
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
    action.subscribe(move |state| logger.log(state.log_level(), state));

    flatten(action.ignite().await).respond_to(&request)
}

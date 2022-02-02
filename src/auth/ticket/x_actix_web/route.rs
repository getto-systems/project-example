use actix_web::{
    delete,
    web::{scope, Data},
    HttpRequest, Responder, Scope,
};

use getto_application::helper::flatten;

use crate::z_lib::remote::{logger::Logger, response::actix_web::RespondTo};

use crate::{
    auth::ticket::check::remote::x_actix_web::route::service_check,
    x_outside_feature::remote::api::{
        feature::ApiAppFeature,
        logger::{app_logger, generate_request_id},
    },
};

use crate::auth::ticket::remote::logout::proxy::init::LogoutProxyStruct;

pub fn scope_ticket() -> Scope {
    scope("/ticket").service(service_check).service(logout)
}

#[delete("")]
async fn logout(feature: Data<ApiAppFeature>, request: HttpRequest) -> impl Responder {
    let request_id = generate_request_id();
    let logger = app_logger(request_id.clone(), &request);

    let mut action = LogoutProxyStruct::action(&feature.auth, &request_id, &request);
    action.subscribe(move |state| logger.log(state));

    flatten(action.ignite().await).respond_to(&request)
}

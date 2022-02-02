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

use crate::auth::user::password::{
    authenticate::remote::x_actix_web::route::service_authenticate,
    reset::remote::x_actix_web::route::scope_reset,
};

use crate::auth::user::password::remote::change::proxy::init::ChangePasswordProxyStruct;

pub fn scope_password() -> Scope {
    scope("/password")
        .service(scope_reset())
        .service(service_authenticate)
        .service(change)
}

#[post("/change")]
async fn change(
    feature: Data<ApiAppFeature>,
    request: HttpRequest,
    body: String,
) -> impl Responder {
    let request_id = generate_request_id();
    let logger = app_logger(request_id.clone(), &request);

    let mut action = ChangePasswordProxyStruct::action(&feature.auth, &request_id, &request, body);
    action.subscribe(move |state| logger.log(state));

    flatten(action.ignite().await).respond_to(&request)
}

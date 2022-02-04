use actix_web::{web::scope, Scope};

use crate::auth::user::password::reset::{
    request_token::api::x_actix_web::route::service_request_token,
    reset::api::x_actix_web::route::service_reset,
};

pub fn scope_reset() -> Scope {
    scope("/reset")
        .service(service_request_token)
        .service(service_reset)
}

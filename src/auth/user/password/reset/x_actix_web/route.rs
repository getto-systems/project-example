use actix_web::{web::scope, Scope};

use crate::auth::user::password::reset::{
    request_token::x_actix_web::route::service_request_token,
    reset::x_actix_web::route::service_reset,
    token_destination::x_actix_web::route::scope_token_destination,
};

pub fn scope_reset() -> Scope {
    scope("/reset")
        .service(service_request_token)
        .service(service_reset)
        .service(scope_token_destination())
}

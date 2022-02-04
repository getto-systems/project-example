use actix_web::{web::scope, Scope};

use crate::auth::ticket::{
    check::api::x_actix_web::route::service_check,
    logout::api::x_actix_web::route::service_logout,
};

pub fn scope_ticket() -> Scope {
    scope("/ticket")
        .service(service_check)
        .service(service_logout)
}

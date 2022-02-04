use actix_web::{web::scope, Scope};

use crate::auth::user::password::{
    authenticate::api::x_actix_web::route::service_authenticate,
    change::api::x_actix_web::route::service_change,
    reset::x_actix_web::route::scope_reset,
};

pub fn scope_password() -> Scope {
    scope("/password")
        .service(scope_reset())
        .service(service_authenticate)
        .service(service_change)
}

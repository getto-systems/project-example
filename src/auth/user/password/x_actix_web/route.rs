use actix_web::{web::scope, Scope};

use crate::auth::user::password::{
    authenticate::x_actix_web::route::service_authenticate,
    change::x_actix_web::route::{service_change_password, service_overwrite_password},
    reset::x_actix_web::route::scope_reset,
};

pub fn scope_password() -> Scope {
    scope("/password")
        .service(scope_reset())
        .service(service_authenticate)
        .service(service_change_password)
        .service(service_overwrite_password)
}

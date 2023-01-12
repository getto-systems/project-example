use actix_web::{web::scope, Scope};

use crate::auth::ticket::{
    authenticate::x_actix_web::route::service_authenticate_with_token,
    logout::x_actix_web::route::service_logout,
};

pub fn scope_ticket() -> Scope {
    scope("/ticket")
        .service(service_authenticate_with_token)
        .service(service_logout)
}

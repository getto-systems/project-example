use actix_web::{web, Scope};

use crate::auth::{
    ticket::remote::x_actix_web::route::scope_ticket,
    user::password::remote::x_actix_web::route::scope_password,
};

pub fn scope_auth() -> Scope {
    web::scope("/auth")
        .service(scope_ticket())
        .service(scope_password())
}
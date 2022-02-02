use actix_web::{web, Scope};

use crate::auth::{
    ticket::x_actix_web::route::scope_ticket, user::remote::x_actix_web::route::scope_user,
};

pub fn scope_auth() -> Scope {
    web::scope("/auth")
        .service(scope_ticket())
        .service(scope_user())
}

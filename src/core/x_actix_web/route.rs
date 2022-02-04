use actix_web::{web, Scope};

use crate::core::outline::x_actix_web::route::scope_outline;

pub fn scope_core() -> Scope {
    web::scope("/core").service(scope_outline())
}

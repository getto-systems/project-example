use actix_web::{web, Scope};

use crate::common::outline::x_actix_web::route::scope_outline;

pub fn scope_common() -> Scope {
    web::scope("/common").service(scope_outline())
}

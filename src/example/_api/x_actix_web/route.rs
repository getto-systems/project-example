use actix_web::{web, Scope};

use crate::example::outline::_api::x_actix_web::route::scope_outline;

pub fn scope_example() -> Scope {
    web::scope("/example").service(scope_outline())
}

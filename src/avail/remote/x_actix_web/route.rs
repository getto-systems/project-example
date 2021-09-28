use actix_web::{Scope, web};

use crate::avail::unexpected_error::remote::x_actix_web::route::scope_unexpected_error;

pub fn scope_avail() -> Scope {
    web::scope("/avail").service(scope_unexpected_error())
}

use actix_web::{web::scope, Scope};

use crate::avail::unexpected_error::notify::x_actix_web::route::service_notify;

pub fn scope_unexpected_error() -> Scope {
    scope("/unexpected-error").service(service_notify)
}

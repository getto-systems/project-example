use actix_web::{web::scope, Scope};

use crate::common::outline::load::x_actix_web::route::service_get_menu_badge;

pub fn scope_outline() -> Scope {
    scope("/outline").service(service_get_menu_badge)
}

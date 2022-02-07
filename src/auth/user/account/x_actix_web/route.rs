use actix_web::{web::scope, Scope};

use crate::auth::user::account::search::x_actix_web::route::service_search;

pub fn scope_account() -> Scope {
    scope("/account").service(service_search)
}

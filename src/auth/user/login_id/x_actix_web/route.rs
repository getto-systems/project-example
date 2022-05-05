use actix_web::{web::scope, Scope};

use crate::auth::user::login_id::change::x_actix_web::route::service_overwrite_login_id;

pub fn scope_login_id() -> Scope {
    scope("/login-id").service(service_overwrite_login_id)
}

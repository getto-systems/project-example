use actix_web::{web, Scope};

use crate::auth::user::{
    account::x_actix_web::route::scope_account, login_id::x_actix_web::route::scope_login_id,
    password::x_actix_web::route::scope_password,
};

pub fn scope_user() -> Scope {
    web::scope("/user")
        .service(scope_account())
        .service(scope_login_id())
        .service(scope_password())
}

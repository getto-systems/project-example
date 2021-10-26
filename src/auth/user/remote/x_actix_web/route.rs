use actix_web::{web, Scope};

use crate::auth::user::{
    account::remote::x_actix_web::route::scope_account,
    password::remote::x_actix_web::route::scope_password,
};

pub fn scope_user() -> Scope {
    web::scope("/user")
        .service(scope_account())
        .service(scope_password())
}

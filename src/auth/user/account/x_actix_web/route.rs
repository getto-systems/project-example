use actix_web::{web::scope, Scope};

use crate::auth::user::account::{
    modify::x_actix_web::route::service_modify_user,
    register::x_actix_web::route::service_register_user,
    search::x_actix_web::route::service_search,
    unregister::x_actix_web::route::service_unregister_user,
};

pub fn scope_account() -> Scope {
    scope("/account")
        .service(service_search)
        .service(service_modify_user)
        .service(service_register_user)
        .service(service_unregister_user)
}

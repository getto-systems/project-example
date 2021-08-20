use actix_web::{web, Scope};

pub fn scope_outline() -> Scope {
    web::scope("/outline")
}

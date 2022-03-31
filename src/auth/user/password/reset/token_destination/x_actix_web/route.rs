use actix_web::{web::scope, Scope};

use crate::auth::user::password::reset::token_destination::change::x_actix_web::route::service_change_destination;

pub fn scope_token_destination() -> Scope {
    scope("/token-destination").service(service_change_destination)
}

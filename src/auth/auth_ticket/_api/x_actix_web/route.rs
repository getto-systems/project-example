use actix_web::{post, web, HttpRequest, Responder, Scope};

use getto_application::helper::flatten;

use crate::x_outside_feature::_api::{
    data::AppData,
    logger::{app_logger, Logger},
};

use crate::auth::auth_ticket::_api::action_renew::action::RenewAuthTicketAction;

pub fn scope_auth_ticket() -> Scope {
    web::scope("/auth-ticket").service(renew)
}

#[post("/renew")]
async fn renew(data: AppData, request: HttpRequest) -> impl Responder {
    let logger = app_logger(&request);
    let mut action = RenewAuthTicketAction::new(&request, &data.auth);
    action.subscribe(move |state| logger.log(state.log_level(), state));
    flatten(action.ignite()).respond_to(&request)
}

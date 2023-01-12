use actix_web::HttpResponse;

use crate::common::api::response::actix_web::ProxyResponder;

use crate::auth::user::password::reset::request_token::proxy::action::RequestResetTokenProxyState;

impl ProxyResponder for RequestResetTokenProxyState {
    fn respond_to(self) -> HttpResponse {
        match self {
            Self::ProxyCall(event) => event.respond_to(),
        }
    }
}

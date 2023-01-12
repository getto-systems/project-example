use actix_web::HttpResponse;

use crate::common::api::response::actix_web::ProxyResponder;

use crate::auth::ticket::authenticate::proxy::action::AuthenticateWithTokenProxyState;

impl ProxyResponder for AuthenticateWithTokenProxyState {
    fn respond_to(self) -> HttpResponse {
        match self {
            Self::AuthenticateWithToken(event) => event.respond_to(),
            Self::ProxyCall(event) => event.respond_to(),
        }
    }
}

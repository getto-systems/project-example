use actix_web::HttpResponse;

use crate::common::api::response::actix_web::ProxyResponder;

use crate::auth::user::password::authenticate::proxy::action::AuthenticateWithPasswordProxyState;

impl ProxyResponder for AuthenticateWithPasswordProxyState {
    fn respond_to(self) -> HttpResponse {
        match self {
            Self::ProxyCall(event) => event.respond_to(),
        }
    }
}

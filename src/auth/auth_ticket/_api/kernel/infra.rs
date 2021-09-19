use crate::auth::auth_ticket::_api::kernel::data::{AuthTokenMessage, AuthTokenResponse};

pub trait AuthTokenResponseBuilder {
    fn build(&self, message: AuthTokenMessage) -> AuthTokenResponse;
}

use actix_web::HttpRequest;

use crate::auth::auth_ticket::_api::kernel::x_actix_web::header::{
    COOKIE_API_TOKEN, COOKIE_TICKET_TOKEN,
};

use crate::z_details::_api::request::helper::cookie;

use crate::auth::auth_ticket::_api::validate::infra::AuthTokenHeader;

use crate::auth::auth_ticket::_api::kernel::data::AuthTokenValue;
use crate::z_details::_api::request::data::HeaderError;

pub struct TicketAuthTokenHeader {
    request: HttpRequest,
}

impl TicketAuthTokenHeader {
    pub const fn new(request: HttpRequest) -> Self {
        Self { request }
    }
}

impl AuthTokenHeader for TicketAuthTokenHeader {
    fn token(&self) -> Result<AuthTokenValue, HeaderError> {
        cookie(&self.request, COOKIE_TICKET_TOKEN).map(AuthTokenValue::new)
    }
}

pub struct ApiAuthTokenHeader {
    request: HttpRequest,
}

impl ApiAuthTokenHeader {
    pub const fn new(request: HttpRequest) -> Self {
        Self { request }
    }
}

impl AuthTokenHeader for ApiAuthTokenHeader {
    fn token(&self) -> Result<AuthTokenValue, HeaderError> {
        cookie(&self.request, COOKIE_API_TOKEN).map(AuthTokenValue::new)
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::auth_ticket::_api::validate::infra::AuthTokenHeader;

    use crate::auth::auth_ticket::_api::kernel::data::AuthTokenValue;
    use crate::z_details::_api::request::data::HeaderError;

    pub enum StaticAuthTokenHeader {
        Valid(AuthTokenValue),
        NotFound, // TODO これのテスト
    }

    impl AuthTokenHeader for StaticAuthTokenHeader {
        fn token(&self) -> Result<AuthTokenValue, HeaderError> {
            match self {
                Self::NotFound => Err(HeaderError::NotFound),
                Self::Valid(token) => Ok(token.clone()),
            }
        }
    }
}

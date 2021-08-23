use actix_web::HttpRequest;

use crate::auth::auth_ticket::_api::kernel::x_actix_web::header::{
    COOKIE_API_TOKEN, COOKIE_TICKET_TOKEN,
};

use crate::z_details::_api::request::helper::cookie;

use crate::auth::auth_ticket::_api::kernel::infra::AuthTokenHeader;

use crate::{
    auth::auth_ticket::_common::kernel::data::AuthToken,
    z_details::_api::request::data::HeaderError,
};

pub struct TicketAuthTokenHeader<'a> {
    request: &'a HttpRequest,
}

impl<'a> TicketAuthTokenHeader<'a> {
    pub const fn new(request: &'a HttpRequest) -> Self {
        Self { request }
    }
}

impl<'a> AuthTokenHeader for TicketAuthTokenHeader<'a> {
    fn token(&self) -> Result<Option<AuthToken>, HeaderError> {
        Ok(cookie(&self.request, COOKIE_TICKET_TOKEN).map(AuthToken::restore))
    }
}

pub struct ApiAuthTokenHeader<'a> {
    request: &'a HttpRequest,
}

impl<'a> ApiAuthTokenHeader<'a> {
    pub const fn new(request: &'a HttpRequest) -> Self {
        Self { request }
    }
}

impl<'a> AuthTokenHeader for ApiAuthTokenHeader<'a> {
    fn token(&self) -> Result<Option<AuthToken>, HeaderError> {
        Ok(cookie(&self.request, COOKIE_API_TOKEN).map(AuthToken::restore))
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::auth_ticket::_api::kernel::infra::AuthTokenHeader;

    use crate::{
        auth::auth_ticket::_common::kernel::data::AuthToken,
        z_details::_api::request::data::HeaderError,
    };

    pub struct StaticAuthTokenHeader {
        token: AuthToken,
    }
    impl StaticAuthTokenHeader {
        pub fn new(token: &str) -> Self {
            Self {
                token: AuthToken::restore(token.into()),
            }
        }
    }

    impl AuthTokenHeader for StaticAuthTokenHeader {
        fn token(&self) -> Result<Option<AuthToken>, HeaderError> {
            Ok(Some(self.token.clone()))
        }
    }
}
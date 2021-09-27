use actix_web::HttpRequest;

use crate::z_details::_api::request::helper::cookie;

use crate::auth::auth_ticket::remote::kernel::x_actix_web::header::{
    COOKIE_API_TOKEN, COOKIE_TICKET_TOKEN,
};

use crate::auth::auth_ticket::remote::kernel::infra::AuthTokenMetadata;

use crate::{
    auth::auth_ticket::remote::kernel::data::AuthToken,
    z_details::_common::request::data::MetadataError,
};

pub struct TicketTokenMetadata<'a> {
    request: &'a HttpRequest,
}

impl<'a> TicketTokenMetadata<'a> {
    pub const fn new(request: &'a HttpRequest) -> Self {
        Self { request }
    }
}

impl<'a> AuthTokenMetadata for TicketTokenMetadata<'a> {
    fn token(&self) -> Result<Option<AuthToken>, MetadataError> {
        Ok(cookie(&self.request, COOKIE_TICKET_TOKEN).map(AuthToken::restore))
    }
}

pub struct ApiTokenMetadata<'a> {
    request: &'a HttpRequest,
}

impl<'a> ApiTokenMetadata<'a> {
    pub const fn new(request: &'a HttpRequest) -> Self {
        Self { request }
    }
}

impl<'a> AuthTokenMetadata for ApiTokenMetadata<'a> {
    fn token(&self) -> Result<Option<AuthToken>, MetadataError> {
        Ok(cookie(&self.request, COOKIE_API_TOKEN).map(AuthToken::restore))
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::auth_ticket::remote::kernel::infra::AuthTokenMetadata;

    use crate::{
        auth::auth_ticket::remote::kernel::data::AuthToken,
        z_details::_common::request::data::MetadataError,
    };

    pub struct StaticAuthTokenMetadata {
        token: AuthToken,
    }
    impl StaticAuthTokenMetadata {
        pub const fn new(token: String) -> Self {
            Self {
                token: AuthToken::restore(token),
            }
        }
    }

    impl AuthTokenMetadata for StaticAuthTokenMetadata {
        fn token(&self) -> Result<Option<AuthToken>, MetadataError> {
            Ok(Some(self.token.clone()))
        }
    }
}

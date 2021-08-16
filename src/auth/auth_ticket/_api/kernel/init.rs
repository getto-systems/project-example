pub(in crate::auth) mod nonce_header;
pub(in crate::auth) mod token_header;
pub(in crate::auth) mod response_builder;

use actix_web::HttpRequest;

use crate::auth::{
    _api::x_outside_feature::feature::AuthOutsideFeature,
    auth_ticket::_api::kernel::init::response_builder::CookieAuthTokenResponseBuilder,
};

use nonce_header::ActixWebAuthNonceHeader;
use token_header::TicketAuthTokenHeader;

use crate::auth::auth_ticket::_api::kernel::infra::{AuthHeaderInfra, AuthTokenInfra};

pub struct TicketAuthHeaderStruct<'a> {
    nonce_header: ActixWebAuthNonceHeader<'a>,
    token_header: TicketAuthTokenHeader<'a>,
}

impl<'a> TicketAuthHeaderStruct<'a> {
    pub fn new(request: &'a HttpRequest) -> Self {
        Self {
            nonce_header: ActixWebAuthNonceHeader::new(request),
            token_header: TicketAuthTokenHeader::new(request),
        }
    }
}

impl<'a> AuthHeaderInfra for TicketAuthHeaderStruct<'a> {
    type NonceHeader = ActixWebAuthNonceHeader<'a>;
    type TokenHeader = TicketAuthTokenHeader<'a>;

    fn nonce_header(&self) -> &Self::NonceHeader {
        &self.nonce_header
    }
    fn token_header(&self) -> &Self::TokenHeader {
        &self.token_header
    }
}

pub struct AuthTokenStruct<'a> {
    response_builder: CookieAuthTokenResponseBuilder<'a>,
}

impl<'a> AuthTokenStruct<'a> {
    pub const fn new(feature: &'a AuthOutsideFeature) -> Self {
        Self {
            response_builder: CookieAuthTokenResponseBuilder::new(&feature.cookie),
        }
    }
}

impl<'a> AuthTokenInfra for AuthTokenStruct<'a> {
    type ResponseBuilder = CookieAuthTokenResponseBuilder<'a>;

    fn response_builder(&self) -> &Self::ResponseBuilder {
        &self.response_builder
    }
}

#[cfg(test)]
pub mod test {
    use super::nonce_header::test::StaticAuthNonceHeader;
    use super::token_header::test::StaticAuthTokenHeader;
    use super::response_builder::test::StaticAuthTokenResponseBuilder;

    use crate::auth::auth_ticket::_api::kernel::infra::{AuthHeaderInfra, AuthTokenInfra};

    pub struct StaticAuthHeaderStruct {
        pub nonce_header: StaticAuthNonceHeader,
        pub token_header: StaticAuthTokenHeader,
    }

    impl AuthHeaderInfra for StaticAuthHeaderStruct {
        type NonceHeader = StaticAuthNonceHeader;
        type TokenHeader = StaticAuthTokenHeader;

        fn nonce_header(&self) -> &Self::NonceHeader {
            &self.nonce_header
        }
        fn token_header(&self) -> &Self::TokenHeader {
            &self.token_header
        }
    }

    pub struct StaticAuthTokenStruct {
        pub response_builder: StaticAuthTokenResponseBuilder,
    }

    impl AuthTokenInfra for StaticAuthTokenStruct {
        type ResponseBuilder = StaticAuthTokenResponseBuilder;

        fn response_builder(&self) -> &Self::ResponseBuilder {
            &self.response_builder
        }
    }
}

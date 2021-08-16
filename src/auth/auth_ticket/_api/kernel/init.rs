pub(in crate::auth) mod nonce_header;
pub(in crate::auth) mod token_header;
pub(in crate::auth) mod token_messenger;

use actix_web::HttpRequest;

use crate::auth::{
    _api::x_outside_feature::feature::AuthOutsideFeature,
    auth_ticket::_api::kernel::init::token_messenger::CookieAuthTokenMessenger,
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
    token_messenger: CookieAuthTokenMessenger<'a>,
}

impl<'a> AuthTokenStruct<'a> {
    pub const fn new(feature: &'a AuthOutsideFeature) -> Self {
        Self {
            token_messenger: CookieAuthTokenMessenger::new(&feature.cookie),
        }
    }
}

impl<'a> AuthTokenInfra for AuthTokenStruct<'a> {
    type TokenMessenger = CookieAuthTokenMessenger<'a>;

    fn token_messenger(&self) -> &Self::TokenMessenger {
        &self.token_messenger
    }
}

#[cfg(test)]
pub mod test {
    use super::nonce_header::test::StaticAuthNonceHeader;
    use super::token_header::test::StaticAuthTokenHeader;
    use super::token_messenger::test::StaticAuthTokenMessenger;

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
        pub token_messenger: StaticAuthTokenMessenger,
    }

    impl AuthTokenInfra for StaticAuthTokenStruct {
        type TokenMessenger = StaticAuthTokenMessenger;

        fn token_messenger(&self) -> &Self::TokenMessenger {
            &self.token_messenger
        }
    }
}

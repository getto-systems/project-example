pub(in crate::auth) mod logout_service;

use actix_web::HttpRequest;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::auth_ticket::_api::{
    kernel::init::{nonce_header::ActixWebAuthNonceHeader, token_header::TicketAuthTokenHeader},
    logout::init::logout_service::TonicLogoutService,
};

use super::infra::LogoutInfra;

pub struct LogoutStruct<'a> {
    nonce_header: ActixWebAuthNonceHeader<'a>,
    token_header: TicketAuthTokenHeader<'a>,
    logout_service: TonicLogoutService<'a>,
}

impl<'a> LogoutStruct<'a> {
    pub fn new(
        feature: &'a AuthOutsideFeature,
        request_id: &'a str,
        request: &'a HttpRequest,
    ) -> Self {
        Self {
            nonce_header: ActixWebAuthNonceHeader::new(request),
            token_header: TicketAuthTokenHeader::new(request),
            logout_service: TonicLogoutService::new(&feature.service, request_id),
        }
    }
}

impl<'a> LogoutInfra for LogoutStruct<'a> {
    type NonceHeader = ActixWebAuthNonceHeader<'a>;
    type TokenHeader = TicketAuthTokenHeader<'a>;
    type LogoutService = TonicLogoutService<'a>;

    fn nonce_header(&self) -> &Self::NonceHeader {
        &self.nonce_header
    }
    fn token_header(&self) -> &Self::TokenHeader {
        &self.token_header
    }
    fn logout_service(&self) -> &Self::LogoutService {
        &self.logout_service
    }
}

#[cfg(test)]
pub mod test {
    use super::logout_service::test::StaticLogoutService;
    use crate::auth::auth_ticket::_api::kernel::init::{
        nonce_header::test::StaticAuthNonceHeader, token_header::test::StaticAuthTokenHeader,
    };

    use super::super::infra::LogoutInfra;

    pub struct StaticLogoutStruct {
        pub nonce_header: StaticAuthNonceHeader,
        pub token_header: StaticAuthTokenHeader,
        pub logout_service: StaticLogoutService,
    }

    impl LogoutInfra for StaticLogoutStruct {
        type NonceHeader = StaticAuthNonceHeader;
        type TokenHeader = StaticAuthTokenHeader;
        type LogoutService = StaticLogoutService;

        fn nonce_header(&self) -> &Self::NonceHeader {
            &self.nonce_header
        }
        fn token_header(&self) -> &Self::TokenHeader {
            &self.token_header
        }
        fn logout_service(&self) -> &Self::LogoutService {
            &self.logout_service
        }
    }
}

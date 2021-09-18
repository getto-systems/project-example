pub(in crate::auth) mod logout_service;

use actix_web::HttpRequest;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::auth_ticket::_api::{
    kernel::init::{
        nonce_metadata::ActixWebAuthNonceMetadata, token_metadata::TicketAuthTokenMetadata,
    },
    logout::init::logout_service::TonicLogoutService,
};

use super::infra::LogoutInfra;

pub struct LogoutStruct<'a> {
    nonce_metadata: ActixWebAuthNonceMetadata<'a>,
    token_metadata: TicketAuthTokenMetadata<'a>,
    logout_service: TonicLogoutService<'a>,
}

impl<'a> LogoutStruct<'a> {
    pub fn new(
        feature: &'a AuthOutsideFeature,
        request_id: &'a str,
        request: &'a HttpRequest,
    ) -> Self {
        Self {
            nonce_metadata: ActixWebAuthNonceMetadata::new(request),
            token_metadata: TicketAuthTokenMetadata::new(request),
            logout_service: TonicLogoutService::new(&feature.service, request_id),
        }
    }
}

impl<'a> LogoutInfra for LogoutStruct<'a> {
    type NonceMetadata = ActixWebAuthNonceMetadata<'a>;
    type TokenMetadata = TicketAuthTokenMetadata<'a>;
    type LogoutService = TonicLogoutService<'a>;

    fn nonce_metadata(&self) -> &Self::NonceMetadata {
        &self.nonce_metadata
    }
    fn token_metadata(&self) -> &Self::TokenMetadata {
        &self.token_metadata
    }
    fn logout_service(&self) -> &Self::LogoutService {
        &self.logout_service
    }
}

#[cfg(test)]
pub mod test {
    use super::logout_service::test::StaticLogoutService;
    use crate::auth::auth_ticket::_common::kernel::init::{
        nonce_metadata::test::StaticAuthNonceMetadata,
        token_metadata::test::StaticAuthTokenMetadata,
    };

    use super::super::infra::LogoutInfra;

    pub struct StaticLogoutStruct {
        pub nonce_metadata: StaticAuthNonceMetadata,
        pub token_metadata: StaticAuthTokenMetadata,
        pub logout_service: StaticLogoutService,
    }

    impl LogoutInfra for StaticLogoutStruct {
        type NonceMetadata = StaticAuthNonceMetadata;
        type TokenMetadata = StaticAuthTokenMetadata;
        type LogoutService = StaticLogoutService;

        fn nonce_metadata(&self) -> &Self::NonceMetadata {
            &self.nonce_metadata
        }
        fn token_metadata(&self) -> &Self::TokenMetadata {
            &self.token_metadata
        }
        fn logout_service(&self) -> &Self::LogoutService {
            &self.logout_service
        }
    }
}

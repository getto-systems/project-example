pub(in crate::auth) mod authenticate_service;
pub(in crate::auth) mod request_decoder;
pub(in crate::auth) mod response_encoder;

use actix_web::HttpRequest;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::auth_ticket::_api::kernel::init::{
    nonce_header::ActixWebAuthNonceHeader, response_builder::CookieAuthTokenResponseBuilder,
    token_header::TicketAuthTokenHeader,
};
use authenticate_service::TonicAuthenticatePasswordService;
use response_encoder::ProstAuthenticatePasswordResponseEncoder;

use super::infra::AuthenticatePasswordInfra;

pub struct AuthenticatePasswordStruct<'a> {
    nonce_header: ActixWebAuthNonceHeader<'a>,
    token_header: TicketAuthTokenHeader<'a>,
    authenticate_service: TonicAuthenticatePasswordService<'a>,
    response_encoder: ProstAuthenticatePasswordResponseEncoder,
    response_builder: CookieAuthTokenResponseBuilder<'a>,
}

impl<'a> AuthenticatePasswordStruct<'a> {
    pub fn new(
        feature: &'a AuthOutsideFeature,
        request_id: &'a str,
        request: &'a HttpRequest,
    ) -> Self {
        Self {
            nonce_header: ActixWebAuthNonceHeader::new(request),
            token_header: TicketAuthTokenHeader::new(request),
            authenticate_service: TonicAuthenticatePasswordService::new(
                &feature.service,
                request_id,
            ),
            response_encoder: ProstAuthenticatePasswordResponseEncoder,
            response_builder: CookieAuthTokenResponseBuilder::new(&feature.cookie),
        }
    }
}

impl<'a> AuthenticatePasswordInfra for AuthenticatePasswordStruct<'a> {
    type NonceHeader = ActixWebAuthNonceHeader<'a>;
    type TokenHeader = TicketAuthTokenHeader<'a>;
    type AuthenticateService = TonicAuthenticatePasswordService<'a>;
    type ResponseEncoder = ProstAuthenticatePasswordResponseEncoder;
    type ResponseBuilder = CookieAuthTokenResponseBuilder<'a>;

    fn nonce_header(&self) -> &Self::NonceHeader {
        &self.nonce_header
    }
    fn token_header(&self) -> &Self::TokenHeader {
        &self.token_header
    }
    fn authenticate_service(&self) -> &Self::AuthenticateService {
        &self.authenticate_service
    }
    fn response_encoder(&self) -> &Self::ResponseEncoder {
        &self.response_encoder
    }
    fn response_builder(&self) -> &Self::ResponseBuilder {
        &self.response_builder
    }
}

#[cfg(test)]
pub mod test {
    use super::authenticate_service::test::StaticAuthenticatePasswordService;
    use super::response_encoder::test::StaticAuthenticatePasswordResponseEncoder;

    use crate::auth::auth_ticket::_api::kernel::init::{
        nonce_header::test::StaticAuthNonceHeader,
        response_builder::test::StaticAuthTokenResponseBuilder,
        token_header::test::StaticAuthTokenHeader,
    };

    use super::super::infra::AuthenticatePasswordInfra;

    pub struct StaticAuthenticatePasswordStruct {
        pub nonce_header: StaticAuthNonceHeader,
        pub token_header: StaticAuthTokenHeader,
        pub authenticate_service: StaticAuthenticatePasswordService,
        pub response_encoder: StaticAuthenticatePasswordResponseEncoder,
        pub response_builder: StaticAuthTokenResponseBuilder,
    }

    impl AuthenticatePasswordInfra for StaticAuthenticatePasswordStruct {
        type NonceHeader = StaticAuthNonceHeader;
        type TokenHeader = StaticAuthTokenHeader;
        type AuthenticateService = StaticAuthenticatePasswordService;
        type ResponseEncoder = StaticAuthenticatePasswordResponseEncoder;
        type ResponseBuilder = StaticAuthTokenResponseBuilder;

        fn nonce_header(&self) -> &Self::NonceHeader {
            &self.nonce_header
        }
        fn token_header(&self) -> &Self::TokenHeader {
            &self.token_header
        }
        fn authenticate_service(&self) -> &Self::AuthenticateService {
            &self.authenticate_service
        }
        fn response_encoder(&self) -> &Self::ResponseEncoder {
            &self.response_encoder
        }
        fn response_builder(&self) -> &Self::ResponseBuilder {
            &self.response_builder
        }
    }
}

mod authenticate_service;
mod request_decoder;
mod response_encoder;

use actix_web::HttpRequest;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::auth_ticket::_api::kernel::init::{AuthTokenStruct, TicketAuthHeaderStruct};
use authenticate_service::TonicAuthenticatePasswordService;
use request_decoder::ProstAuthenticatePasswordRequestDecoder;
use response_encoder::ProstAuthenticatePasswordResponseEncoder;

use super::infra::AuthenticatePasswordInfra;

pub struct AuthenticatePasswordStruct<'a> {
    header_infra: TicketAuthHeaderStruct<'a>,
    token_infra: AuthTokenStruct<'a>,
    request_decoder: ProstAuthenticatePasswordRequestDecoder,
    authenticate_service: TonicAuthenticatePasswordService<'a>,
    response_encoder: ProstAuthenticatePasswordResponseEncoder,
}

impl<'a> AuthenticatePasswordStruct<'a> {
    pub fn new(
        feature: &'a AuthOutsideFeature,
        request_id: &'a str,
        request: &'a HttpRequest,
        body: String,
    ) -> Self {
        Self {
            header_infra: TicketAuthHeaderStruct::new(request),
            token_infra: AuthTokenStruct::new(feature),
            request_decoder: ProstAuthenticatePasswordRequestDecoder::new(body),
            authenticate_service: TonicAuthenticatePasswordService::new(
                &feature.service,
                request_id,
            ),
            response_encoder: ProstAuthenticatePasswordResponseEncoder,
        }
    }
}

impl<'a> AuthenticatePasswordInfra for AuthenticatePasswordStruct<'a> {
    type HeaderInfra = TicketAuthHeaderStruct<'a>;
    type TokenInfra = AuthTokenStruct<'a>;
    type RequestDecoder = ProstAuthenticatePasswordRequestDecoder;
    type AuthenticateService = TonicAuthenticatePasswordService<'a>;
    type ResponseEncoder = ProstAuthenticatePasswordResponseEncoder;

    fn header_infra(&self) -> &Self::HeaderInfra {
        &self.header_infra
    }
    fn token_infra(&self) -> &Self::TokenInfra {
        &self.token_infra
    }
    fn request_decoder(&self) -> &Self::RequestDecoder {
        &self.request_decoder
    }
    fn authenticate_service(&self) -> &Self::AuthenticateService {
        &self.authenticate_service
    }
    fn response_encoder(&self) -> &Self::ResponseEncoder {
        &self.response_encoder
    }
}

#[cfg(test)]
pub mod test {
    pub use super::authenticate_service::test::StaticAuthenticatePasswordService;
    pub use super::request_decoder::test::StaticAuthenticatePasswordRequestDecoder;
    pub use super::response_encoder::test::StaticAuthenticatePasswordResponseEncoder;

    use crate::auth::auth_ticket::_api::kernel::init::test::{
        StaticAuthHeaderStruct, StaticAuthTokenStruct,
    };

    use super::super::infra::AuthenticatePasswordInfra;

    pub struct StaticAuthenticatePasswordStruct {
        pub header_infra: StaticAuthHeaderStruct,
        pub token_infra: StaticAuthTokenStruct,
        pub request_decoder: StaticAuthenticatePasswordRequestDecoder,
        pub authenticate_service: StaticAuthenticatePasswordService,
        pub response_encoder: StaticAuthenticatePasswordResponseEncoder,
    }

    impl AuthenticatePasswordInfra for StaticAuthenticatePasswordStruct {
        type HeaderInfra = StaticAuthHeaderStruct;
        type TokenInfra = StaticAuthTokenStruct;
        type RequestDecoder = StaticAuthenticatePasswordRequestDecoder;
        type AuthenticateService = StaticAuthenticatePasswordService;
        type ResponseEncoder = StaticAuthenticatePasswordResponseEncoder;

        fn header_infra(&self) -> &Self::HeaderInfra {
            &self.header_infra
        }
        fn token_infra(&self) -> &Self::TokenInfra {
            &self.token_infra
        }
        fn request_decoder(&self) -> &Self::RequestDecoder {
            &self.request_decoder
        }
        fn authenticate_service(&self) -> &Self::AuthenticateService {
            &self.authenticate_service
        }
        fn response_encoder(&self) -> &Self::ResponseEncoder {
            &self.response_encoder
        }
    }
}

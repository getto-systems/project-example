pub(in crate::auth) mod request_decoder;
pub(in crate::auth) mod reset_service;
pub(in crate::auth) mod response_encoder;

use actix_web::HttpRequest;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::auth_ticket::_api::kernel::init::{AuthTokenStruct, TicketAuthHeaderStruct};
use request_decoder::ProstResetPasswordRequestDecoder;
use reset_service::TonicResetPasswordService;
use response_encoder::ProstResetPasswordResponseEncoder;

use crate::auth::password::reset::_api::reset::infra::ResetPasswordInfra;

pub struct ResetPasswordStruct<'a> {
    header_infra: TicketAuthHeaderStruct<'a>,
    token_infra: AuthTokenStruct<'a>,
    request_decoder: ProstResetPasswordRequestDecoder,
    reset_service: TonicResetPasswordService<'a>,
    response_encoder: ProstResetPasswordResponseEncoder,
}

impl<'a> ResetPasswordStruct<'a> {
    pub fn new(
        feature: &'a AuthOutsideFeature,
        request_id: &'a str,
        request: &'a HttpRequest,
        body: String,
    ) -> Self {
        Self {
            header_infra: TicketAuthHeaderStruct::new(request),
            token_infra: AuthTokenStruct::new(feature),
            request_decoder: ProstResetPasswordRequestDecoder::new(body),
            reset_service: TonicResetPasswordService::new(&feature.service, request_id),
            response_encoder: ProstResetPasswordResponseEncoder,
        }
    }
}

impl<'a> ResetPasswordInfra for ResetPasswordStruct<'a> {
    type HeaderInfra = TicketAuthHeaderStruct<'a>;
    type TokenInfra = AuthTokenStruct<'a>;
    type RequestDecoder = ProstResetPasswordRequestDecoder;
    type ResetService = TonicResetPasswordService<'a>;
    type ResponseEncoder = ProstResetPasswordResponseEncoder;

    fn header_infra(&self) -> &Self::HeaderInfra {
        &self.header_infra
    }
    fn token_infra(&self) -> &Self::TokenInfra {
        &self.token_infra
    }
    fn request_decoder(&self) -> &Self::RequestDecoder {
        &self.request_decoder
    }
    fn reset_service(&self) -> &Self::ResetService {
        &self.reset_service
    }
    fn response_encoder(&self) -> &Self::ResponseEncoder {
        &self.response_encoder
    }
}

#[cfg(test)]
pub mod test {
    pub use super::reset_service::test::StaticResetPasswordService;
    pub use super::request_decoder::test::StaticResetPasswordRequestDecoder;
    pub use super::response_encoder::test::StaticResetPasswordResponseEncoder;

    use crate::auth::auth_ticket::_api::kernel::init::test::{
        StaticAuthHeaderStruct, StaticAuthTokenStruct,
    };

    use super::super::infra::ResetPasswordInfra;

    pub struct StaticResetPasswordStruct {
        pub header_infra: StaticAuthHeaderStruct,
        pub token_infra: StaticAuthTokenStruct,
        pub request_decoder: StaticResetPasswordRequestDecoder,
        pub reset_service: StaticResetPasswordService,
        pub response_encoder: StaticResetPasswordResponseEncoder,
    }

    impl ResetPasswordInfra for StaticResetPasswordStruct {
        type HeaderInfra = StaticAuthHeaderStruct;
        type TokenInfra = StaticAuthTokenStruct;
        type RequestDecoder = StaticResetPasswordRequestDecoder;
        type ResetService = StaticResetPasswordService;
        type ResponseEncoder = StaticResetPasswordResponseEncoder;

        fn header_infra(&self) -> &Self::HeaderInfra {
            &self.header_infra
        }
        fn token_infra(&self) -> &Self::TokenInfra {
            &self.token_infra
        }
        fn request_decoder(&self) -> &Self::RequestDecoder {
            &self.request_decoder
        }
        fn reset_service(&self) -> &Self::ResetService {
            &self.reset_service
        }
        fn response_encoder(&self) -> &Self::ResponseEncoder {
            &self.response_encoder
        }
    }
}

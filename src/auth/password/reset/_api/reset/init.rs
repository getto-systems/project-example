pub(in crate::auth) mod request_decoder;
pub(in crate::auth) mod reset_service;
pub(in crate::auth) mod response_encoder;

use actix_web::HttpRequest;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::auth_ticket::_api::kernel::init::response_builder::CookieAuthTokenResponseBuilder;
use crate::auth::auth_ticket::_api::kernel::init::{
    nonce_header::ActixWebAuthNonceHeader, token_header::TicketAuthTokenHeader,
};
use reset_service::TonicResetPasswordService;
use response_encoder::ProstResetPasswordResponseEncoder;

use crate::auth::password::reset::_api::reset::infra::ResetPasswordInfra;

pub struct ResetPasswordStruct<'a> {
    nonce_header: ActixWebAuthNonceHeader<'a>,
    token_header: TicketAuthTokenHeader<'a>,
    reset_service: TonicResetPasswordService<'a>,
    response_encoder: ProstResetPasswordResponseEncoder,
    response_builder: CookieAuthTokenResponseBuilder<'a>,
}

impl<'a> ResetPasswordStruct<'a> {
    pub fn new(
        feature: &'a AuthOutsideFeature,
        request_id: &'a str,
        request: &'a HttpRequest,
    ) -> Self {
        Self {
            nonce_header: ActixWebAuthNonceHeader::new(request),
            token_header: TicketAuthTokenHeader::new(request),
            reset_service: TonicResetPasswordService::new(&feature.service, request_id),
            response_encoder: ProstResetPasswordResponseEncoder,
            response_builder: CookieAuthTokenResponseBuilder::new(&feature.cookie),
        }
    }
}

impl<'a> ResetPasswordInfra for ResetPasswordStruct<'a> {
    type NonceHeader = ActixWebAuthNonceHeader<'a>;
    type TokenHeader = TicketAuthTokenHeader<'a>;
    type ResetService = TonicResetPasswordService<'a>;
    type ResponseEncoder = ProstResetPasswordResponseEncoder;
    type ResponseBuilder = CookieAuthTokenResponseBuilder<'a>;

    fn nonce_header(&self) -> &Self::NonceHeader {
        &self.nonce_header
    }
    fn token_header(&self) -> &Self::TokenHeader {
        &self.token_header
    }
    fn reset_service(&self) -> &Self::ResetService {
        &self.reset_service
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
    use super::reset_service::test::StaticResetPasswordService;
    use super::response_encoder::test::StaticResetPasswordResponseEncoder;

    use crate::auth::auth_ticket::_api::kernel::init::{
        nonce_header::test::StaticAuthNonceHeader,
        response_builder::test::StaticAuthTokenResponseBuilder,
        token_header::test::StaticAuthTokenHeader,
    };

    use super::super::infra::ResetPasswordInfra;

    pub struct StaticResetPasswordStruct {
        pub nonce_header: StaticAuthNonceHeader,
        pub token_header: StaticAuthTokenHeader,
        pub reset_service: StaticResetPasswordService,
        pub response_encoder: StaticResetPasswordResponseEncoder,
        pub response_builder: StaticAuthTokenResponseBuilder,
    }

    impl ResetPasswordInfra for StaticResetPasswordStruct {
        type NonceHeader = StaticAuthNonceHeader;
        type TokenHeader = StaticAuthTokenHeader;
        type ResetService = StaticResetPasswordService;
        type ResponseEncoder = StaticResetPasswordResponseEncoder;
        type ResponseBuilder = StaticAuthTokenResponseBuilder;

        fn nonce_header(&self) -> &Self::NonceHeader {
            &self.nonce_header
        }
        fn token_header(&self) -> &Self::TokenHeader {
            &self.token_header
        }
        fn reset_service(&self) -> &Self::ResetService {
            &self.reset_service
        }
        fn response_encoder(&self) -> &Self::ResponseEncoder {
            &self.response_encoder
        }
        fn response_builder(&self) -> &Self::ResponseBuilder {
            &self.response_builder
        }
    }
}

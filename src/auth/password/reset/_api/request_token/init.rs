pub(in crate::auth) mod request_decoder;
pub(in crate::auth) mod request_token_service;
pub(in crate::auth) mod response_encoder;

use actix_web::HttpRequest;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::auth_ticket::_api::kernel::init::{
    nonce_header::ActixWebAuthNonceHeader, token_header::TicketAuthTokenHeader,
};
use request_decoder::ProtobufRequestResetTokenRequestDecoder;
use request_token_service::TonicRequestResetTokenService;
use response_encoder::ProstRequestResetTokenResponseEncoder;

use super::infra::RequestResetTokenInfra;

pub struct RequestResetTokenStruct<'a> {
    nonce_header: ActixWebAuthNonceHeader<'a>,
    token_header: TicketAuthTokenHeader<'a>,
    request_decoder: ProtobufRequestResetTokenRequestDecoder,
    request_token_service: TonicRequestResetTokenService<'a>,
    response_encoder: ProstRequestResetTokenResponseEncoder,
}

impl<'a> RequestResetTokenStruct<'a> {
    pub fn new(
        feature: &'a AuthOutsideFeature,
        request_id: &'a str,
        request: &'a HttpRequest,
        body: String,
    ) -> Self {
        Self {
            nonce_header: ActixWebAuthNonceHeader::new(request),
            token_header: TicketAuthTokenHeader::new(request),
            request_decoder: ProtobufRequestResetTokenRequestDecoder::new(body),
            request_token_service: TonicRequestResetTokenService::new(&feature.service, request_id),
            response_encoder: ProstRequestResetTokenResponseEncoder,
        }
    }
}

impl<'a> RequestResetTokenInfra for RequestResetTokenStruct<'a> {
    type NonceHeader = ActixWebAuthNonceHeader<'a>;
    type TokenHeader = TicketAuthTokenHeader<'a>;
    type RequestDecoder = ProtobufRequestResetTokenRequestDecoder;
    type RequestTokenService = TonicRequestResetTokenService<'a>;
    type ResponseEncoder = ProstRequestResetTokenResponseEncoder;

    fn nonce_header(&self) -> &Self::NonceHeader {
        &self.nonce_header
    }
    fn token_header(&self) -> &Self::TokenHeader {
        &self.token_header
    }
    fn request_decoder(&self) -> &Self::RequestDecoder {
        &self.request_decoder
    }
    fn request_token_service(&self) -> &Self::RequestTokenService {
        &self.request_token_service
    }
    fn response_encoder(&self) -> &Self::ResponseEncoder {
        &self.response_encoder
    }
}

#[cfg(test)]
pub mod test {
    use super::request_decoder::test::StaticRequestResetTokenRequestDecoder;
    use super::request_token_service::test::StaticRequestResetTokenService;
    use super::response_encoder::test::StaticRequestResetTokenResponseEncoder;
    use crate::auth::auth_ticket::_api::kernel::init::{
        nonce_header::test::StaticAuthNonceHeader, token_header::test::StaticAuthTokenHeader,
    };

    use super::super::infra::RequestResetTokenInfra;

    pub struct StaticRequestResetTokenStruct {
        pub nonce_header: StaticAuthNonceHeader,
        pub token_header: StaticAuthTokenHeader,
        pub request_decoder: StaticRequestResetTokenRequestDecoder,
        pub request_token_service: StaticRequestResetTokenService,
        pub response_encoder: StaticRequestResetTokenResponseEncoder,
    }

    impl RequestResetTokenInfra for StaticRequestResetTokenStruct {
        type NonceHeader = StaticAuthNonceHeader;
        type TokenHeader = StaticAuthTokenHeader;
        type RequestDecoder = StaticRequestResetTokenRequestDecoder;
        type RequestTokenService = StaticRequestResetTokenService;
        type ResponseEncoder = StaticRequestResetTokenResponseEncoder;

        fn nonce_header(&self) -> &Self::NonceHeader {
            &self.nonce_header
        }
        fn token_header(&self) -> &Self::TokenHeader {
            &self.token_header
        }
        fn request_decoder(&self) -> &Self::RequestDecoder {
            &self.request_decoder
        }
        fn request_token_service(&self) -> &Self::RequestTokenService {
            &self.request_token_service
        }
        fn response_encoder(&self) -> &Self::ResponseEncoder {
            &self.response_encoder
        }
    }
}

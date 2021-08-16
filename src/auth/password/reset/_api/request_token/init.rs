pub(in crate::auth) mod request_decoder;
pub(in crate::auth) mod request_token_service;
pub(in crate::auth) mod response_encoder;

use actix_web::HttpRequest;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::auth_ticket::_api::kernel::init::TicketAuthHeaderStruct;
use request_decoder::ProtobufRequestResetTokenRequestDecoder;
use request_token_service::TonicRequestResetTokenService;
use response_encoder::ProstRequestResetTokenResponseEncoder;

use super::infra::RequestResetTokenInfra;

pub struct RequestResetTokenStruct<'a> {
    header_infra: TicketAuthHeaderStruct<'a>,
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
            header_infra: TicketAuthHeaderStruct::new(request),
            request_decoder: ProtobufRequestResetTokenRequestDecoder::new(body),
            request_token_service: TonicRequestResetTokenService::new(&feature.service, request_id),
            response_encoder: ProstRequestResetTokenResponseEncoder,
        }
    }
}

impl<'a> RequestResetTokenInfra for RequestResetTokenStruct<'a> {
    type HeaderInfra = TicketAuthHeaderStruct<'a>;
    type RequestDecoder = ProtobufRequestResetTokenRequestDecoder;
    type RequestTokenService = TonicRequestResetTokenService<'a>;
    type ResponseEncoder = ProstRequestResetTokenResponseEncoder;

    fn header_infra(&self) -> &Self::HeaderInfra {
        &self.header_infra
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
    pub use super::request_decoder::test::StaticRequestResetTokenRequestDecoder;
    pub use super::request_token_service::test::StaticRequestResetTokenService;
    pub use super::response_encoder::test::StaticRequestResetTokenResponseEncoder;

    use crate::auth::auth_ticket::_api::kernel::init::test::StaticAuthHeaderStruct;

    use super::super::infra::RequestResetTokenInfra;

    pub struct StaticRequestResetTokenStruct {
        pub header_infra: StaticAuthHeaderStruct,
        pub request_decoder: StaticRequestResetTokenRequestDecoder,
        pub request_token_service: StaticRequestResetTokenService,
        pub response_encoder: StaticRequestResetTokenResponseEncoder,
    }

    impl RequestResetTokenInfra for StaticRequestResetTokenStruct {
        type HeaderInfra = StaticAuthHeaderStruct;
        type RequestDecoder = StaticRequestResetTokenRequestDecoder;
        type RequestTokenService = StaticRequestResetTokenService;
        type ResponseEncoder = StaticRequestResetTokenResponseEncoder;

        fn header_infra(&self) -> &Self::HeaderInfra {
            &self.header_infra
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

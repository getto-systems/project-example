pub(in crate::auth) mod request_decoder;
pub(in crate::auth) mod request_token_service;
pub(in crate::auth) mod response_encoder;

use actix_web::HttpRequest;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::auth_ticket::_api::kernel::init::auth_metadata::NoAuthMetadata;
use request_token_service::TonicRequestResetTokenService;
use response_encoder::ProstRequestResetTokenResponseEncoder;

use super::infra::RequestResetTokenInfra;

pub struct RequestResetTokenStruct<'a> {
    auth_metadata: NoAuthMetadata<'a>,
    request_token_service: TonicRequestResetTokenService<'a>,
    response_encoder: ProstRequestResetTokenResponseEncoder,
}

impl<'a> RequestResetTokenStruct<'a> {
    pub fn new(
        feature: &'a AuthOutsideFeature,
        request_id: &'a str,
        request: &'a HttpRequest,
    ) -> Self {
        Self {
            auth_metadata: NoAuthMetadata::new(request),
            request_token_service: TonicRequestResetTokenService::new(&feature.service, request_id),
            response_encoder: ProstRequestResetTokenResponseEncoder,
        }
    }
}

impl<'a> RequestResetTokenInfra for RequestResetTokenStruct<'a> {
    type AuthMetadata = NoAuthMetadata<'a>;
    type RequestTokenService = TonicRequestResetTokenService<'a>;
    type ResponseEncoder = ProstRequestResetTokenResponseEncoder;

    fn auth_metadata(&self) -> &Self::AuthMetadata {
        &self.auth_metadata
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
    use super::request_token_service::test::StaticRequestResetTokenService;
    use super::response_encoder::test::StaticRequestResetTokenResponseEncoder;
    use crate::auth::auth_ticket::_common::kernel::init::auth_metadata::test::StaticAuthMetadata;

    use super::super::infra::RequestResetTokenInfra;

    pub struct StaticRequestResetTokenStruct {
        pub auth_metadata: StaticAuthMetadata,
        pub request_token_service: StaticRequestResetTokenService,
        pub response_encoder: StaticRequestResetTokenResponseEncoder,
    }

    impl RequestResetTokenInfra for StaticRequestResetTokenStruct {
        type AuthMetadata = StaticAuthMetadata;
        type RequestTokenService = StaticRequestResetTokenService;
        type ResponseEncoder = StaticRequestResetTokenResponseEncoder;

        fn auth_metadata(&self) -> &Self::AuthMetadata {
            &self.auth_metadata
        }
        fn request_token_service(&self) -> &Self::RequestTokenService {
            &self.request_token_service
        }
        fn response_encoder(&self) -> &Self::ResponseEncoder {
            &self.response_encoder
        }
    }
}

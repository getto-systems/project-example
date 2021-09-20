pub(in crate::auth) mod request_decoder;
pub(in crate::auth) mod reset_service;
pub(in crate::auth) mod response_encoder;

use actix_web::HttpRequest;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::auth_ticket::_api::kernel::init::{
    response_builder::CookieAuthTokenResponseBuilder, auth_metadata::NoAuthMetadata,
};
use reset_service::TonicResetPasswordService;
use response_encoder::ProstResetPasswordResponseEncoder;

use crate::auth::password::reset::_api::reset::infra::ResetPasswordInfra;

pub struct ResetPasswordStruct<'a> {
    auth_metadata: NoAuthMetadata<'a>,
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
            auth_metadata: NoAuthMetadata::new(request),
            reset_service: TonicResetPasswordService::new(&feature.service, request_id),
            response_encoder: ProstResetPasswordResponseEncoder,
            response_builder: CookieAuthTokenResponseBuilder::new(&feature.cookie),
        }
    }
}

impl<'a> ResetPasswordInfra for ResetPasswordStruct<'a> {
    type AuthMetadata = NoAuthMetadata<'a>;
    type ResetService = TonicResetPasswordService<'a>;
    type ResponseEncoder = ProstResetPasswordResponseEncoder;
    type ResponseBuilder = CookieAuthTokenResponseBuilder<'a>;

    fn auth_metadata(&self) -> &Self::AuthMetadata {
        &self.auth_metadata
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

    use crate::auth::auth_ticket::{
        _api::kernel::init::response_builder::test::StaticAuthTokenResponseBuilder,
        _common::kernel::init::auth_metadata::test::StaticAuthMetadata,
    };

    use super::super::infra::ResetPasswordInfra;

    pub struct StaticResetPasswordStruct {
        pub auth_metadata: StaticAuthMetadata,
        pub reset_service: StaticResetPasswordService,
        pub response_encoder: StaticResetPasswordResponseEncoder,
        pub response_builder: StaticAuthTokenResponseBuilder,
    }

    impl ResetPasswordInfra for StaticResetPasswordStruct {
        type AuthMetadata = StaticAuthMetadata;
        type ResetService = StaticResetPasswordService;
        type ResponseEncoder = StaticResetPasswordResponseEncoder;
        type ResponseBuilder = StaticAuthTokenResponseBuilder;

        fn auth_metadata(&self) -> &Self::AuthMetadata {
            &self.auth_metadata
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

pub(in crate::auth) mod authenticate_service;
pub(in crate::auth) mod request_decoder;
pub(in crate::auth) mod response_encoder;

use actix_web::HttpRequest;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::auth_ticket::_api::kernel::init::{
    response_builder::CookieAuthTokenResponseBuilder, auth_metadata::TicketAuthMetadata,
};
use authenticate_service::TonicAuthenticatePasswordService;
use response_encoder::ProstAuthenticatePasswordResponseEncoder;

use super::infra::AuthenticatePasswordInfra;

pub struct AuthenticatePasswordStruct<'a> {
    auth_metadata: TicketAuthMetadata<'a>,
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
            auth_metadata: TicketAuthMetadata::new(&feature.key, request),
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
    type AuthMetadata = TicketAuthMetadata<'a>;
    type AuthenticateService = TonicAuthenticatePasswordService<'a>;
    type ResponseEncoder = ProstAuthenticatePasswordResponseEncoder;
    type ResponseBuilder = CookieAuthTokenResponseBuilder<'a>;

    fn auth_metadata(&self) -> &Self::AuthMetadata {
        &self.auth_metadata
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

    use crate::auth::auth_ticket::{
        _api::kernel::init::response_builder::test::StaticAuthTokenResponseBuilder,
        _common::kernel::init::auth_metadata::test::StaticAuthMetadata,
    };

    use super::super::infra::AuthenticatePasswordInfra;

    pub struct StaticAuthenticatePasswordStruct {
        pub auth_metadata: StaticAuthMetadata,
        pub authenticate_service: StaticAuthenticatePasswordService,
        pub response_encoder: StaticAuthenticatePasswordResponseEncoder,
        pub response_builder: StaticAuthTokenResponseBuilder,
    }

    impl AuthenticatePasswordInfra for StaticAuthenticatePasswordStruct {
        type AuthMetadata = StaticAuthMetadata;
        type AuthenticateService = StaticAuthenticatePasswordService;
        type ResponseEncoder = StaticAuthenticatePasswordResponseEncoder;
        type ResponseBuilder = StaticAuthTokenResponseBuilder;

        fn auth_metadata(&self) -> &Self::AuthMetadata {
            &self.auth_metadata
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

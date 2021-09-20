pub(in crate::auth) mod change_service;
pub(in crate::auth) mod request_decoder;
pub(in crate::auth) mod response_encoder;

use actix_web::HttpRequest;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::auth_ticket::_api::kernel::init::auth_metadata::ApiAuthMetadata;
use change_service::TonicChangePasswordService;
use response_encoder::ProstChangePasswordResponseEncoder;

use super::infra::ChangePasswordInfra;

pub struct ChangePasswordStruct<'a> {
    auth_metadata: ApiAuthMetadata<'a>,
    change_service: TonicChangePasswordService<'a>,
    response_encoder: ProstChangePasswordResponseEncoder,
}

impl<'a> ChangePasswordStruct<'a> {
    pub fn new(
        feature: &'a AuthOutsideFeature,
        request_id: &'a str,
        request: &'a HttpRequest,
    ) -> Self {
        Self {
            auth_metadata: ApiAuthMetadata::new(&feature.key, request),
            change_service: TonicChangePasswordService::new(&feature.service, request_id),
            response_encoder: ProstChangePasswordResponseEncoder,
        }
    }
}

impl<'a> ChangePasswordInfra for ChangePasswordStruct<'a> {
    type AuthMetadata = ApiAuthMetadata<'a>;
    type ChangeService = TonicChangePasswordService<'a>;
    type ResponseEncoder = ProstChangePasswordResponseEncoder;

    fn auth_metadata(&self) -> &Self::AuthMetadata {
        &self.auth_metadata
    }
    fn change_service(&self) -> &Self::ChangeService {
        &self.change_service
    }
    fn response_encoder(&self) -> &Self::ResponseEncoder {
        &self.response_encoder
    }
}

#[cfg(test)]
pub mod test {
    use super::change_service::test::StaticChangePasswordService;
    use super::response_encoder::test::StaticChangePasswordResponseEncoder;

    use crate::auth::auth_ticket::_common::kernel::init::auth_metadata::test::StaticAuthMetadata;

    use super::super::infra::ChangePasswordInfra;

    pub struct StaticChangePasswordStruct {
        pub auth_metadata: StaticAuthMetadata,
        pub change_service: StaticChangePasswordService,
        pub response_encoder: StaticChangePasswordResponseEncoder,
    }

    impl ChangePasswordInfra for StaticChangePasswordStruct {
        type AuthMetadata = StaticAuthMetadata;
        type ChangeService = StaticChangePasswordService;
        type ResponseEncoder = StaticChangePasswordResponseEncoder;

        fn auth_metadata(&self) -> &Self::AuthMetadata {
            &self.auth_metadata
        }
        fn change_service(&self) -> &Self::ChangeService {
            &self.change_service
        }
        fn response_encoder(&self) -> &Self::ResponseEncoder {
            &self.response_encoder
        }
    }
}

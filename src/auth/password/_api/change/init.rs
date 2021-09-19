pub(in crate::auth) mod change_service;
pub(in crate::auth) mod request_decoder;
pub(in crate::auth) mod response_encoder;

use actix_web::HttpRequest;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::auth_ticket::_api::kernel::init::service_metadata::ApiServiceMetadata;
use change_service::TonicChangePasswordService;
use response_encoder::ProstChangePasswordResponseEncoder;

use super::infra::ChangePasswordInfra;

pub struct ChangePasswordStruct<'a> {
    service_metadata: ApiServiceMetadata<'a>,
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
            service_metadata: ApiServiceMetadata::new(request, &feature.key),
            change_service: TonicChangePasswordService::new(&feature.service, request_id),
            response_encoder: ProstChangePasswordResponseEncoder,
        }
    }
}

impl<'a> ChangePasswordInfra for ChangePasswordStruct<'a> {
    type ServiceMetadata = ApiServiceMetadata<'a>;
    type ChangeService = TonicChangePasswordService<'a>;
    type ResponseEncoder = ProstChangePasswordResponseEncoder;

    fn service_metadata(&self) -> &Self::ServiceMetadata {
        &self.service_metadata
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

    use crate::auth::auth_ticket::_common::kernel::init::service_metadata::test::StaticAuthServiceMetadata;

    use super::super::infra::ChangePasswordInfra;

    pub struct StaticChangePasswordStruct {
        pub service_metadata: StaticAuthServiceMetadata,
        pub change_service: StaticChangePasswordService,
        pub response_encoder: StaticChangePasswordResponseEncoder,
    }

    impl ChangePasswordInfra for StaticChangePasswordStruct {
        type ServiceMetadata = StaticAuthServiceMetadata;
        type ChangeService = StaticChangePasswordService;
        type ResponseEncoder = StaticChangePasswordResponseEncoder;

        fn service_metadata(&self) -> &Self::ServiceMetadata {
            &self.service_metadata
        }
        fn change_service(&self) -> &Self::ChangeService {
            &self.change_service
        }
        fn response_encoder(&self) -> &Self::ResponseEncoder {
            &self.response_encoder
        }
    }
}

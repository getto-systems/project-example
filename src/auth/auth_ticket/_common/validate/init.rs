pub(in crate::auth) mod validate_service;

use tonic::metadata::MetadataMap;

use crate::auth::_common::x_outside_feature::feature::AuthOutsideService;

use crate::auth::auth_ticket::_common::{
    kernel::init::service_metadata::TonicAuthServiceMetadata,
    validate::init::validate_service::TonicValidateService,
};

use crate::auth::_common::infra::ValidateApiTokenInfra;

pub struct ValidateApiTokenStruct<'a> {
    service_metadata: TonicAuthServiceMetadata<'a>,
    validate_service: TonicValidateService<'a>,
}

impl<'a> ValidateApiTokenStruct<'a> {
    pub fn new(
        service: &'a AuthOutsideService,
        request_id: &'a str,
        metadata: &'a MetadataMap,
    ) -> Self {
        Self {
            service_metadata: TonicAuthServiceMetadata::new(metadata),
            validate_service: TonicValidateService::new(service, request_id),
        }
    }
}

impl<'a> ValidateApiTokenInfra for ValidateApiTokenStruct<'a> {
    type ServiceMetadata = TonicAuthServiceMetadata<'a>;
    type ValidateService = TonicValidateService<'a>;

    fn service_metadata(&self) -> &Self::ServiceMetadata {
        &self.service_metadata
    }
    fn validate_service(&self) -> &Self::ValidateService {
        &self.validate_service
    }
}

#[cfg(test)]
pub mod test {
    use super::validate_service::test::StaticValidateService;
    use crate::auth::auth_ticket::_common::kernel::init::service_metadata::test::StaticAuthServiceMetadata;

    use super::super::infra::ValidateApiTokenInfra;

    pub struct StaticValidateApiTokenStruct {
        pub service_metadata: StaticAuthServiceMetadata,
        pub validate_service: StaticValidateService,
    }

    impl ValidateApiTokenInfra for StaticValidateApiTokenStruct {
        type ServiceMetadata = StaticAuthServiceMetadata;
        type ValidateService = StaticValidateService;

        fn service_metadata(&self) -> &Self::ServiceMetadata {
            &self.service_metadata
        }
        fn validate_service(&self) -> &Self::ValidateService {
            &self.validate_service
        }
    }
}

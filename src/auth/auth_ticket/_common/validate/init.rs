pub(in crate::auth) mod validate_service;

use tonic::metadata::MetadataMap;

use crate::auth::_common::x_outside_feature::feature::AuthOutsideService;

use crate::auth::auth_ticket::_common::{
    kernel::init::auth_metadata::TonicAuthMetadata,
    validate::init::validate_service::TonicValidateService,
};

use crate::auth::_common::infra::ValidateApiTokenInfra;

pub struct ValidateApiTokenStruct<'a> {
    auth_metadata: TonicAuthMetadata<'a>,
    validate_service: TonicValidateService<'a>,
}

impl<'a> ValidateApiTokenStruct<'a> {
    pub fn new(
        service: &'a AuthOutsideService,
        request_id: &'a str,
        metadata: &'a MetadataMap,
    ) -> Self {
        Self {
            auth_metadata: TonicAuthMetadata::new(metadata),
            validate_service: TonicValidateService::new(service, request_id),
        }
    }
}

impl<'a> ValidateApiTokenInfra for ValidateApiTokenStruct<'a> {
    type AuthMetadata = TonicAuthMetadata<'a>;
    type ValidateService = TonicValidateService<'a>;

    fn auth_metadata(&self) -> &Self::AuthMetadata {
        &self.auth_metadata
    }
    fn validate_service(&self) -> &Self::ValidateService {
        &self.validate_service
    }
}

#[cfg(test)]
pub mod test {
    use super::validate_service::test::StaticValidateService;
    use crate::auth::auth_ticket::_common::kernel::init::auth_metadata::test::StaticAuthMetadata;

    use super::super::infra::ValidateApiTokenInfra;

    pub struct StaticValidateApiTokenStruct {
        pub auth_metadata: StaticAuthMetadata,
        pub validate_service: StaticValidateService,
    }

    impl ValidateApiTokenInfra for StaticValidateApiTokenStruct {
        type AuthMetadata = StaticAuthMetadata;
        type ValidateService = StaticValidateService;

        fn auth_metadata(&self) -> &Self::AuthMetadata {
            &self.auth_metadata
        }
        fn validate_service(&self) -> &Self::ValidateService {
            &self.validate_service
        }
    }
}

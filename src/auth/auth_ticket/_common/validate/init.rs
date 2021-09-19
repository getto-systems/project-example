pub(in crate::auth) mod validate_service;

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

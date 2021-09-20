pub use crate::auth::auth_ticket::_common::validate::init::ValidateApiTokenStruct;

#[cfg(test)]
pub mod test {
    pub use crate::auth::auth_ticket::_common::{
        kernel::init::service_metadata::test::StaticAuthServiceMetadata,
        validate::init::{
            test::StaticValidateApiTokenStruct, validate_service::test::StaticValidateService,
        },
    };
}

// TODO api から validate はしないようにしたいのであとで消す
use actix_web::HttpRequest;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::auth_ticket::{
    _api::kernel::init::service_metadata::ApiServiceMetadata,
    _common::validate::init::validate_service::TonicValidateService,
};

use crate::auth::_common::infra::ValidateApiTokenInfra;

pub struct ValidateApiTokenStructForApi<'a> {
    service_metadata: ApiServiceMetadata<'a>,
    validate_service: TonicValidateService<'a>,
}

impl<'a> ValidateApiTokenStructForApi<'a> {
    pub fn new(
        feature: &'a AuthOutsideFeature,
        request_id: &'a str,
        request: &'a HttpRequest,
    ) -> Self {
        Self {
            service_metadata: ApiServiceMetadata::new(&feature.key, request),
            validate_service: TonicValidateService::new(&feature.service, request_id),
        }
    }
}

impl<'a> ValidateApiTokenInfra for ValidateApiTokenStructForApi<'a> {
    type ServiceMetadata = ApiServiceMetadata<'a>;
    type ValidateService = TonicValidateService<'a>;

    fn service_metadata(&self) -> &Self::ServiceMetadata {
        &self.service_metadata
    }
    fn validate_service(&self) -> &Self::ValidateService {
        &self.validate_service
    }
}

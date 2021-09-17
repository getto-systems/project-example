pub(in crate::auth) mod change_service;
pub(in crate::auth) mod request_decoder;
pub(in crate::auth) mod response_encoder;

use actix_web::HttpRequest;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::auth_ticket::_api::validate::init::ValidateApiTokenStruct;
use change_service::TonicChangePasswordService;
use response_encoder::ProstChangePasswordResponseEncoder;

use super::infra::ChangePasswordInfra;

pub struct ChangePasswordStruct<'a> {
    validate_infra: ValidateApiTokenStruct<'a>,
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
            validate_infra: ValidateApiTokenStruct::new(&feature, request_id, request),
            change_service: TonicChangePasswordService::new(&feature.service, request_id),
            response_encoder: ProstChangePasswordResponseEncoder,
        }
    }
}

impl<'a> ChangePasswordInfra for ChangePasswordStruct<'a> {
    type ValidateInfra = ValidateApiTokenStruct<'a>;
    type ChangeService = TonicChangePasswordService<'a>;
    type ResponseEncoder = ProstChangePasswordResponseEncoder;

    fn validate_infra(&self) -> &Self::ValidateInfra {
        &self.validate_infra
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

    use crate::auth::auth_ticket::_api::validate::init::test::StaticValidateApiTokenStruct;

    use super::super::infra::ChangePasswordInfra;

    pub struct StaticChangePasswordStruct {
        pub validate_infra: StaticValidateApiTokenStruct,
        pub change_service: StaticChangePasswordService,
        pub response_encoder: StaticChangePasswordResponseEncoder,
    }

    impl ChangePasswordInfra for StaticChangePasswordStruct {
        type ValidateInfra = StaticValidateApiTokenStruct;
        type ChangeService = StaticChangePasswordService;
        type ResponseEncoder = StaticChangePasswordResponseEncoder;

        fn validate_infra(&self) -> &Self::ValidateInfra {
            &self.validate_infra
        }
        fn change_service(&self) -> &Self::ChangeService {
            &self.change_service
        }
        fn response_encoder(&self) -> &Self::ResponseEncoder {
            &self.response_encoder
        }
    }
}

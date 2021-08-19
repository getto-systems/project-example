pub(in crate::avail) mod request_decoder;

use actix_web::HttpRequest;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::_api::common::init::ValidateApiTokenStruct;

use super::infra::NotifyUnexpectedErrorInfra;

pub struct NotifyUnexpectedErrorStruct<'a> {
    validate_infra: ValidateApiTokenStruct<'a>,
}

impl<'a> NotifyUnexpectedErrorStruct<'a> {
    pub fn new(
        feature: &'a AuthOutsideFeature,
        request_id: &'a str,
        request: &'a HttpRequest,
    ) -> Self {
        Self {
            validate_infra: ValidateApiTokenStruct::new(feature, request_id, request),
        }
    }
}

impl<'a> NotifyUnexpectedErrorInfra for NotifyUnexpectedErrorStruct<'a> {
    type ValidateInfra = ValidateApiTokenStruct<'a>;

    fn validate_infra(&self) -> &Self::ValidateInfra {
        &self.validate_infra
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::_api::common::init::test::StaticValidateApiTokenStruct;

    use super::super::infra::NotifyUnexpectedErrorInfra;

    pub struct StaticNotifyUnexpectedErrorStruct {
        pub validate_infra: StaticValidateApiTokenStruct,
    }

    impl NotifyUnexpectedErrorInfra for StaticNotifyUnexpectedErrorStruct {
        type ValidateInfra = StaticValidateApiTokenStruct;

        fn validate_infra(&self) -> &Self::ValidateInfra {
            &self.validate_infra
        }
    }
}

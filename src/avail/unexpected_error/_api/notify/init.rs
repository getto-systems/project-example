pub(in crate::avail) mod request_decoder;

use actix_web::HttpRequest;

use crate::x_outside_feature::_api::feature::AppFeature;

use crate::auth::_common::init::ValidateApiTokenStructForApi;

use super::infra::NotifyUnexpectedErrorInfra;

pub struct NotifyUnexpectedErrorStruct<'a> {
    validate_infra: ValidateApiTokenStructForApi<'a>,
}

impl<'a> NotifyUnexpectedErrorStruct<'a> {
    pub fn new(feature: &'a AppFeature, request_id: &'a str, request: &'a HttpRequest) -> Self {
        Self {
            validate_infra: ValidateApiTokenStructForApi::new(&feature.auth, request_id, request),
        }
    }
}

impl<'a> NotifyUnexpectedErrorInfra for NotifyUnexpectedErrorStruct<'a> {
    type ValidateInfra = ValidateApiTokenStructForApi<'a>;

    fn validate_infra(&self) -> &Self::ValidateInfra {
        &self.validate_infra
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::_common::init::test::StaticValidateApiTokenStruct;

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

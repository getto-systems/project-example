pub(in crate::avail) mod request_decoder;

use tonic::metadata::MetadataMap;

use crate::x_outside_feature::_example::feature::ExampleAppFeature;

use crate::auth::remote::init::ValidateApiTokenStruct;

use crate::avail::unexpected_error::_example::notify::infra::NotifyUnexpectedErrorInfra;

pub struct NotifyUnexpectedErrorStruct<'a> {
    validate_infra: ValidateApiTokenStruct<'a>,
}

impl<'a> NotifyUnexpectedErrorStruct<'a> {
    pub fn new(
        feature: &'a ExampleAppFeature,
        request_id: &'a str,
        metadata: &'a MetadataMap,
    ) -> Self {
        Self {
            validate_infra: ValidateApiTokenStruct::new(
                &feature.auth.service,
                request_id,
                metadata,
            ),
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
    use crate::auth::remote::init::test::StaticValidateApiTokenStruct;

    use crate::avail::unexpected_error::_example::notify::infra::NotifyUnexpectedErrorInfra;

    pub struct StaticNotifyUnexpectedErrorStruct {
        pub validate_infra: StaticValidateApiTokenStruct,
    }

    impl<'a> NotifyUnexpectedErrorInfra for StaticNotifyUnexpectedErrorStruct {
        type ValidateInfra = StaticValidateApiTokenStruct;

        fn validate_infra(&self) -> &Self::ValidateInfra {
            &self.validate_infra
        }
    }
}

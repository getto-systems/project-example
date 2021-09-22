pub(in crate::avail) mod request_decoder;

use tonic::metadata::MetadataMap;

use crate::avail::unexpected_error::_common::y_protobuf::service::NotifyRequestPb;

use crate::x_outside_feature::_example::feature::ExampleAppFeature;

use crate::auth::_common::init::ValidateApiTokenStruct;
use request_decoder::PbNotifyUnexpectedErrorRequestDecoder;

use crate::avail::unexpected_error::_example::notify::infra::{
    NotifyUnexpectedErrorInfra, NotifyUnexpectedErrorRequestDecoder,
};

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

    pub fn request_decoder(request: NotifyRequestPb) -> impl NotifyUnexpectedErrorRequestDecoder {
        PbNotifyUnexpectedErrorRequestDecoder::new(request)
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
    use crate::auth::_common::init::test::StaticValidateApiTokenStruct;

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

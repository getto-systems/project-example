pub mod request_decoder;

use tonic::metadata::MetadataMap;

use crate::auth::remote::init::ValidateApiTokenStruct;
use crate::avail::unexpected_error::remote::y_protobuf::service::NotifyRequestPb;

use crate::x_outside_feature::remote::example::feature::ExampleAppFeature;

use crate::avail::unexpected_error::remote::notify::init::request_decoder::PbNotifyUnexpectedErrorRequestDecoder;

use super::action::{NotifyUnexpectedErrorAction, NotifyUnexpectedErrorMaterial};

pub struct NotifyUnexpectedErrorFeature<'a> {
    validate: ValidateApiTokenStruct<'a>,
}

impl<'a> NotifyUnexpectedErrorFeature<'a> {
    pub fn action(
        feature: &'a ExampleAppFeature,
        request_id: &'a str,
        metadata: &'a MetadataMap,
        request: NotifyRequestPb,
    ) -> NotifyUnexpectedErrorAction<PbNotifyUnexpectedErrorRequestDecoder, Self> {
        NotifyUnexpectedErrorAction::with_material(
            PbNotifyUnexpectedErrorRequestDecoder::new(request),
            Self {
                validate: ValidateApiTokenStruct::new(&feature.auth.service, request_id, metadata),
            },
        )
    }
}

#[async_trait::async_trait]
impl<'a> NotifyUnexpectedErrorMaterial for NotifyUnexpectedErrorFeature<'a> {
    type Validate = ValidateApiTokenStruct<'a>;

    fn validate(&self) -> &Self::Validate {
        &self.validate
    }
}

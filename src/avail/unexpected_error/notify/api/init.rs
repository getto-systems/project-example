pub mod request_decoder;

use tonic::metadata::MetadataMap;

use crate::avail::unexpected_error::notify::y_protobuf::service::NotifyRequestPb;

use crate::x_outside_feature::core::feature::CoreAppFeature;

use crate::{
    auth::init::AuthorizeStruct,
    avail::unexpected_error::notify::init::request_decoder::PbNotifyUnexpectedErrorRequestDecoder,
};

use super::action::{NotifyUnexpectedErrorAction, NotifyUnexpectedErrorMaterial};

pub struct NotifyUnexpectedErrorFeature<'a> {
    authorize: AuthorizeStruct<'a>,
}

impl<'a> NotifyUnexpectedErrorFeature<'a> {
    pub fn action(
        feature: &'a CoreAppFeature,
        request_id: &'a str,
        metadata: &'a MetadataMap,
        request: NotifyRequestPb,
    ) -> NotifyUnexpectedErrorAction<PbNotifyUnexpectedErrorRequestDecoder, Self> {
        NotifyUnexpectedErrorAction::with_material(
            PbNotifyUnexpectedErrorRequestDecoder::new(request),
            Self {
                authorize: AuthorizeStruct::new(
                    &feature.auth.service,
                    request_id,
                    metadata,
                ),
            },
        )
    }
}

#[async_trait::async_trait]
impl<'a> NotifyUnexpectedErrorMaterial for NotifyUnexpectedErrorFeature<'a> {
    type Authorize = AuthorizeStruct<'a>;

    fn authorize(&self) -> &Self::Authorize {
        &self.authorize
    }
}

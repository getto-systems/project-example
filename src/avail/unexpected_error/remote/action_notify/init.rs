use tonic::metadata::MetadataMap;

use crate::avail::unexpected_error::remote::y_protobuf::service::NotifyRequestPb;

use crate::x_outside_feature::remote::example::feature::ExampleAppFeature;

use crate::avail::unexpected_error::remote::notify::init::{
    request_decoder::PbNotifyUnexpectedErrorRequestDecoder, NotifyUnexpectedErrorStruct,
};

use super::action::{NotifyUnexpectedErrorAction, NotifyUnexpectedErrorMaterial};

use crate::avail::unexpected_error::remote::notify::infra::NotifyUnexpectedErrorRequestDecoder;

pub struct NotifyUnexpectedErrorFeature<'a> {
    notify: NotifyUnexpectedErrorStruct<'a>,
}

impl<'a> NotifyUnexpectedErrorFeature<'a> {
    pub fn action(
        feature: &'a ExampleAppFeature,
        request_id: &'a str,
        metadata: &'a MetadataMap,
    ) -> NotifyUnexpectedErrorAction<Self> {
        NotifyUnexpectedErrorAction::with_material(Self {
            notify: NotifyUnexpectedErrorStruct::new(feature, request_id, metadata),
        })
    }

    pub fn request_decoder(request: NotifyRequestPb) -> impl NotifyUnexpectedErrorRequestDecoder {
        PbNotifyUnexpectedErrorRequestDecoder::new(request)
    }
}

#[async_trait::async_trait]
impl<'a> NotifyUnexpectedErrorMaterial for NotifyUnexpectedErrorFeature<'a> {
    type Notify = NotifyUnexpectedErrorStruct<'a>;

    fn notify(&self) -> &Self::Notify {
        &self.notify
    }
}

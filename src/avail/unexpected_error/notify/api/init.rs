pub mod request_decoder;

use tonic::metadata::MetadataMap;

use crate::avail::unexpected_error::notify::y_protobuf::service::NotifyRequestPb;

use crate::x_outside_feature::api::core::feature::CoreAppFeature;

use crate::{
    auth::init::CheckPermissionStruct,
    avail::unexpected_error::notify::api::init::request_decoder::PbNotifyUnexpectedErrorRequestDecoder,
};

use super::action::{NotifyUnexpectedErrorAction, NotifyUnexpectedErrorMaterial};

pub struct NotifyUnexpectedErrorFeature<'a> {
    check_permission: CheckPermissionStruct<'a>,
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
                check_permission: CheckPermissionStruct::new(
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
    type CheckPermission = CheckPermissionStruct<'a>;

    fn check_permission(&self) -> &Self::CheckPermission {
        &self.check_permission
    }
}

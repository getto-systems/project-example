use tonic::metadata::MetadataMap;

use crate::auth::password::reset::_common::y_protobuf::service::RequestResetTokenRequestPb;

use crate::auth::_auth::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::password::reset::_auth::request_token::init::RequestResetTokenStruct;

use super::action::{RequestResetTokenAction, RequestResetTokenMaterial};

impl<'a> RequestResetTokenAction<RequestResetTokenFeature<'a>> {
    pub fn new(
        feature: &'a AuthOutsideFeature,
        metadata: MetadataMap,
        request: RequestResetTokenRequestPb,
    ) -> Self {
        Self::with_material(RequestResetTokenFeature::new(feature, metadata, request))
    }
}

pub struct RequestResetTokenFeature<'a> {
    request_token: RequestResetTokenStruct<'a>,
}

impl<'a> RequestResetTokenFeature<'a> {
    fn new(
        feature: &'a AuthOutsideFeature,
        metadata: MetadataMap,
        request: RequestResetTokenRequestPb,
    ) -> Self {
        Self {
            request_token: RequestResetTokenStruct::new(feature, metadata, request),
        }
    }
}

impl<'a> RequestResetTokenMaterial for RequestResetTokenFeature<'a> {
    type RequestToken = RequestResetTokenStruct<'a>;

    fn extract(self) -> Self::RequestToken {
        self.request_token
    }
}

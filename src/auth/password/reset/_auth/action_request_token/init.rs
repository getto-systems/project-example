use tonic::metadata::MetadataMap;

use crate::auth::password::reset::{
    _auth::request_token::infra::RequestResetTokenRequestDecoder,
    _common::y_protobuf::service::RequestResetTokenRequestPb,
};

use crate::auth::_auth::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::password::reset::_auth::request_token::init::{
    request_decoder::PbRequestResetTokenRequestDecoder, RequestResetTokenStruct,
};

use super::action::{RequestResetTokenAction, RequestResetTokenMaterial};

pub struct RequestResetTokenFeature<'a> {
    request_token: RequestResetTokenStruct<'a>,
}

impl<'a> RequestResetTokenFeature<'a> {
    pub fn action(
        feature: &'a AuthOutsideFeature,
        metadata: &'a MetadataMap,
    ) -> RequestResetTokenAction<Self> {
        RequestResetTokenAction::with_material(Self {
            request_token: RequestResetTokenStruct::new(feature, metadata),
        })
    }
    pub fn request_decoder(
        request: RequestResetTokenRequestPb,
    ) -> impl RequestResetTokenRequestDecoder {
        PbRequestResetTokenRequestDecoder::new(request)
    }
}

impl<'a> RequestResetTokenMaterial for RequestResetTokenFeature<'a> {
    type RequestToken = RequestResetTokenStruct<'a>;

    fn extract(self) -> Self::RequestToken {
        self.request_token
    }
}

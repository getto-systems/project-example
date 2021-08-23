use tonic::metadata::MetadataMap;

use crate::auth::password::reset::_common::y_protobuf::service::RequestResetTokenRequestPb;

use crate::x_outside_feature::_auth::feature::AppFeature;

use crate::auth::password::reset::_auth::request_token::init::{
    request_decoder::PbRequestResetTokenRequestDecoder, RequestResetTokenStruct,
};

use super::action::{RequestResetTokenAction, RequestResetTokenMaterial};

use crate::auth::password::reset::_auth::request_token::infra::RequestResetTokenRequestDecoder;

pub struct RequestResetTokenFeature<'a> {
    request_token: RequestResetTokenStruct<'a>,
}

impl<'a> RequestResetTokenFeature<'a> {
    pub fn action(
        feature: &'a AppFeature,
        metadata: &'a MetadataMap,
    ) -> RequestResetTokenAction<Self> {
        RequestResetTokenAction::with_material(Self {
            request_token: RequestResetTokenStruct::new(&feature.auth, metadata),
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

    fn request_token(&self) -> &Self::RequestToken {
        &self.request_token
    }
}
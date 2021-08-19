use actix_web::HttpRequest;

use crate::x_outside_feature::_api::feature::AppFeature;

use crate::auth::password::reset::_api::request_token::init::{
    request_decoder::ProtobufRequestResetTokenRequestDecoder, RequestResetTokenStruct,
};

use super::action::{RequestResetTokenAction, RequestResetTokenMaterial};

use crate::auth::password::reset::_api::request_token::infra::RequestResetTokenRequestDecoder;

pub struct RequestResetTokenFeature<'a> {
    request_token: RequestResetTokenStruct<'a>,
}

impl<'a> RequestResetTokenFeature<'a> {
    pub fn action(
        feature: &'a AppFeature,
        request_id: &'a str,
        request: &'a HttpRequest,
    ) -> RequestResetTokenAction<Self> {
        RequestResetTokenAction::with_material(Self {
            request_token: RequestResetTokenStruct::new(&feature.auth, request_id, request),
        })
    }
    pub fn request_decoder(body: String) -> impl RequestResetTokenRequestDecoder {
        ProtobufRequestResetTokenRequestDecoder::new(body)
    }
}

impl<'a> RequestResetTokenMaterial for RequestResetTokenFeature<'a> {
    type RequestToken = RequestResetTokenStruct<'a>;

    fn request_token(&self) -> &Self::RequestToken {
        &self.request_token
    }
}

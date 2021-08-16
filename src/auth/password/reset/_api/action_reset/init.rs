use actix_web::HttpRequest;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::password::reset::_api::reset::init::{
    request_decoder::ProstResetPasswordRequestDecoder, ResetPasswordStruct,
};

use super::action::{ResetPasswordAction, ResetPasswordMaterial};

use crate::auth::password::reset::_api::reset::infra::ResetPasswordRequestDecoder;

pub struct ResetPasswordFeature<'a> {
    reset: ResetPasswordStruct<'a>,
}

impl<'a> ResetPasswordFeature<'a> {
    pub fn action(
        feature: &'a AuthOutsideFeature,
        request_id: &'a str,
        request: &'a HttpRequest,
    ) -> ResetPasswordAction<Self> {
        ResetPasswordAction::with_material(Self {
            reset: ResetPasswordStruct::new(feature, request_id, request),
        })
    }
    pub fn request_decoder(body: String) -> impl ResetPasswordRequestDecoder {
        ProstResetPasswordRequestDecoder::new(body)
    }
}

impl<'a> ResetPasswordMaterial for ResetPasswordFeature<'a> {
    type Reset = ResetPasswordStruct<'a>;

    fn reset(&self) -> &Self::Reset {
        &self.reset
    }
}

use actix_web::HttpRequest;

use crate::x_outside_feature::_api::feature::AppFeature;

use crate::auth::password::_api::change::init::{
    request_decoder::ProstChangePasswordRequestDecoder, ChangePasswordStruct,
};

use super::action::{ChangePasswordAction, ChangePasswordMaterial};

use crate::auth::password::_api::change::infra::ChangePasswordRequestDecoder;

pub struct ChangePasswordFeature<'a> {
    change: ChangePasswordStruct<'a>,
}

impl<'a> ChangePasswordFeature<'a> {
    pub fn action(
        feature: &'a AppFeature,
        request_id: &'a str,
        request: &'a HttpRequest,
    ) -> ChangePasswordAction<Self> {
        ChangePasswordAction::with_material(Self {
            change: ChangePasswordStruct::new(&feature.auth, request_id, request),
        })
    }
    pub fn request_decoder(body: String) -> impl ChangePasswordRequestDecoder {
        ProstChangePasswordRequestDecoder::new(body)
    }
}

impl<'a> ChangePasswordMaterial for ChangePasswordFeature<'a> {
    type Change = ChangePasswordStruct<'a>;

    fn change(&self) -> &Self::Change {
        &self.change
    }
}

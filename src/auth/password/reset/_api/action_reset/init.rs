use actix_web::HttpRequest;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::password::reset::_api::reset::init::ResetPasswordStruct;

use super::action::{ResetPasswordAction, ResetPasswordMaterial};

impl<'a> ResetPasswordAction<ResetPasswordFeature<'a>> {
    pub fn new(
        feature: &'a AuthOutsideFeature,
        request_id: &'a str,
        request: &'a HttpRequest,
        body: String,
    ) -> Self {
        Self::with_material(ResetPasswordFeature::new(
            feature, request_id, request, body,
        ))
    }
}

pub struct ResetPasswordFeature<'a> {
    reset: ResetPasswordStruct<'a>,
}

impl<'a> ResetPasswordFeature<'a> {
    fn new(
        feature: &'a AuthOutsideFeature,
        request_id: &'a str,
        request: &'a HttpRequest,
        body: String,
    ) -> Self {
        Self {
            reset: ResetPasswordStruct::new(feature, request_id, request, body),
        }
    }
}

impl<'a> ResetPasswordMaterial for ResetPasswordFeature<'a> {
    type Reset = ResetPasswordStruct<'a>;

    fn reset(&self) -> &Self::Reset {
        &self.reset
    }
}

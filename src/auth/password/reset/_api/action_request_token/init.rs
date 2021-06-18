use actix_web::HttpRequest;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::password::reset::_api::request_token::init::RequestResetTokenStruct;

use super::action::{RequestResetTokenAction, RequestResetTokenMaterial};

impl<'a> RequestResetTokenAction<RequestResetTokenFeature<'a>> {
    pub fn new(feature: &'a AuthOutsideFeature, request: &'a HttpRequest, body: String) -> Self {
        Self::with_material(RequestResetTokenFeature::new(feature, request, body))
    }
}

pub struct RequestResetTokenFeature<'a> {
    request_token: RequestResetTokenStruct<'a>,
}

impl<'a> RequestResetTokenFeature<'a> {
    fn new(feature: &'a AuthOutsideFeature, request: &'a HttpRequest, body: String) -> Self {
        Self {
            request_token: RequestResetTokenStruct::new(feature, request, body),
        }
    }
}

impl<'a> RequestResetTokenMaterial for RequestResetTokenFeature<'a> {
    type RequestToken = RequestResetTokenStruct<'a>;

    fn request_token(&self) -> &Self::RequestToken {
        &self.request_token
    }
}

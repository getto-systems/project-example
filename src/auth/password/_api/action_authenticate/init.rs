use actix_web::HttpRequest;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::password::_api::authenticate::init::AuthenticatePasswordStruct;

use super::action::{AuthenticatePasswordAction, AuthenticatePasswordMaterial};

impl<'a> AuthenticatePasswordAction<AuthenticatePasswordFeature<'a>> {
    pub fn new(
        feature: &'a AuthOutsideFeature,
        request_id: &'a str,
        request: &'a HttpRequest,
        body: String,
    ) -> Self {
        Self::with_material(AuthenticatePasswordFeature::new(
            feature, request_id, request, body,
        ))
    }
}

pub struct AuthenticatePasswordFeature<'a> {
    authenticate: AuthenticatePasswordStruct<'a>,
}

impl<'a> AuthenticatePasswordFeature<'a> {
    fn new(
        feature: &'a AuthOutsideFeature,
        request_id: &'a str,
        request: &'a HttpRequest,
        body: String,
    ) -> Self {
        Self {
            authenticate: AuthenticatePasswordStruct::new(feature, request_id, request, body),
        }
    }
}

impl<'a> AuthenticatePasswordMaterial for AuthenticatePasswordFeature<'a> {
    type Authenticate = AuthenticatePasswordStruct<'a>;

    fn authenticate(&self) -> &Self::Authenticate {
        &self.authenticate
    }
}

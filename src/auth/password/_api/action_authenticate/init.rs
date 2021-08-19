use actix_web::HttpRequest;

use crate::x_outside_feature::_api::feature::AppFeature;

use crate::auth::password::_api::authenticate::init::{
    request_decoder::ProstAuthenticatePasswordRequestDecoder, AuthenticatePasswordStruct,
};

use super::action::{AuthenticatePasswordAction, AuthenticatePasswordMaterial};

use crate::auth::password::_api::authenticate::infra::AuthenticatePasswordRequestDecoder;

pub struct AuthenticatePasswordFeature<'a> {
    authenticate: AuthenticatePasswordStruct<'a>,
}

impl<'a> AuthenticatePasswordFeature<'a> {
    pub fn action(
        feature: &'a AppFeature,
        request_id: &'a str,
        request: &'a HttpRequest,
    ) -> AuthenticatePasswordAction<Self> {
        AuthenticatePasswordAction::with_material(Self {
            authenticate: AuthenticatePasswordStruct::new(&feature.auth, request_id, request),
        })
    }
    pub fn request_decoder(body: String) -> impl AuthenticatePasswordRequestDecoder {
        ProstAuthenticatePasswordRequestDecoder::new(body)
    }
}

impl<'a> AuthenticatePasswordMaterial for AuthenticatePasswordFeature<'a> {
    type Authenticate = AuthenticatePasswordStruct<'a>;

    fn authenticate(&self) -> &Self::Authenticate {
        &self.authenticate
    }
}

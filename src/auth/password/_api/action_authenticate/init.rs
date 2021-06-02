use actix_web::HttpRequest;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::{
    auth_ticket::_api::{
        encode::init::EncodeAuthenticatePasswordAuthTicketStruct,
        issue::init::IssueAuthTicketStruct,
    },
    password::_api::authenticate::init::AuthenticatePasswordStruct,
};

use super::action::AuthenticatePasswordAction;
use super::action::AuthenticatePasswordMaterial;

impl<'a> AuthenticatePasswordAction<AuthenticatePasswordFeature<'a>> {
    pub fn new(request: HttpRequest, body: String, feature: &'a AuthOutsideFeature) -> Self {
        Self::with_material(AuthenticatePasswordFeature::new(request, body, feature))
    }
}

pub struct AuthenticatePasswordFeature<'a> {
    authenticate: AuthenticatePasswordStruct<'a>,
    issue: IssueAuthTicketStruct<'a>,
    encode: EncodeAuthenticatePasswordAuthTicketStruct<'a>,
}

impl<'a> AuthenticatePasswordFeature<'a> {
    fn new(request: HttpRequest, body: String, feature: &'a AuthOutsideFeature) -> Self {
        Self {
            authenticate: AuthenticatePasswordStruct::new(request, body, feature),
            issue: IssueAuthTicketStruct::new(feature),
            encode: EncodeAuthenticatePasswordAuthTicketStruct::new(feature),
        }
    }
}

impl<'a> AuthenticatePasswordMaterial for AuthenticatePasswordFeature<'a> {
    type Authenticate = AuthenticatePasswordStruct<'a>;
    type Issue = IssueAuthTicketStruct<'a>;
    type Encode = EncodeAuthenticatePasswordAuthTicketStruct<'a>;

    fn authenticate(&self) -> &Self::Authenticate {
        &self.authenticate
    }
    fn issue(&self) -> &Self::Issue {
        &self.issue
    }
    fn encode(&self) -> &Self::Encode {
        &self.encode
    }
}

use tonic::metadata::MetadataMap;

use crate::auth::password::_common::y_protobuf::service::AuthenticatePasswordRequestPb;

use crate::auth::_auth::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::{
    auth_ticket::_auth::{
        encode::init::EncodeAuthTicketStruct, issue::init::IssueAuthTicketStruct,
    },
    password::_auth::authenticate::init::AuthenticatePasswordStruct,
};

use super::action::{AuthenticatePasswordAction, AuthenticatePasswordMaterial};

impl<'a> AuthenticatePasswordAction<AuthenticatePasswordFeature<'a>> {
    pub fn new(
        feature: &'a AuthOutsideFeature,
        metadata: &'a MetadataMap,
        request: AuthenticatePasswordRequestPb,
    ) -> Self {
        Self::with_material(AuthenticatePasswordFeature::new(feature, metadata, request))
    }
}

pub struct AuthenticatePasswordFeature<'a> {
    authenticate: AuthenticatePasswordStruct<'a>,
    issue: IssueAuthTicketStruct<'a>,
    encode: EncodeAuthTicketStruct<'a>,
}

impl<'a> AuthenticatePasswordFeature<'a> {
    fn new(
        feature: &'a AuthOutsideFeature,
        metadata: &'a MetadataMap,
        request: AuthenticatePasswordRequestPb,
    ) -> Self {
        Self {
            authenticate: AuthenticatePasswordStruct::new(feature, metadata, request),
            issue: IssueAuthTicketStruct::new(feature),
            encode: EncodeAuthTicketStruct::new(feature),
        }
    }
}

impl<'a> AuthenticatePasswordMaterial for AuthenticatePasswordFeature<'a> {
    type Authenticate = AuthenticatePasswordStruct<'a>;
    type Issue = IssueAuthTicketStruct<'a>;
    type Encode = EncodeAuthTicketStruct<'a>;

    fn extract(self) -> (Self::Authenticate, Self::Issue, Self::Encode) {
        (self.authenticate, self.issue, self.encode)
    }
}

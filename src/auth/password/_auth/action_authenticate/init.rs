use tonic::metadata::MetadataMap;

use crate::auth::password::_auth::authenticate::infra::AuthenticatePasswordRequestDecoder;
use crate::auth::password::_auth::authenticate::init::request_decoder::PbAuthenticatePasswordRequestDecoder;
use crate::auth::password::_common::y_protobuf::service::AuthenticatePasswordRequestPb;

use crate::auth::_auth::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::{
    auth_ticket::_auth::{
        encode::init::EncodeAuthTicketStruct, issue::init::IssueAuthTicketStruct,
    },
    password::_auth::authenticate::init::AuthenticatePasswordStruct,
};

use super::action::{AuthenticatePasswordAction, AuthenticatePasswordMaterial};

pub struct AuthenticatePasswordFeature<'a> {
    authenticate: AuthenticatePasswordStruct<'a>,
    issue: IssueAuthTicketStruct<'a>,
    encode: EncodeAuthTicketStruct<'a>,
}

impl<'a> AuthenticatePasswordFeature<'a> {
    pub fn action(
        feature: &'a AuthOutsideFeature,
        metadata: &'a MetadataMap,
    ) -> AuthenticatePasswordAction<Self> {
        AuthenticatePasswordAction::with_material(Self {
            authenticate: AuthenticatePasswordStruct::new(feature, metadata),
            issue: IssueAuthTicketStruct::new(feature),
            encode: EncodeAuthTicketStruct::new(feature),
        })
    }
    pub fn request_decoder(
        request: AuthenticatePasswordRequestPb,
    ) -> impl AuthenticatePasswordRequestDecoder {
        PbAuthenticatePasswordRequestDecoder::new(request)
    }
}

impl<'a> AuthenticatePasswordMaterial for AuthenticatePasswordFeature<'a> {
    type Authenticate = AuthenticatePasswordStruct<'a>;
    type Issue = IssueAuthTicketStruct<'a>;
    type Encode = EncodeAuthTicketStruct<'a>;

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

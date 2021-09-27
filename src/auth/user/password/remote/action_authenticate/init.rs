use tonic::metadata::MetadataMap;

use crate::auth::user::password::_common::y_protobuf::service::AuthenticatePasswordRequestPb;

use crate::x_outside_feature::_auth::feature::AuthAppFeature;

use crate::auth::{
    ticket::remote::{
        encode::init::EncodeAuthTicketStruct, issue::init::IssueAuthTicketStruct,
    },
    user::password::remote::authenticate::init::{
        request_decoder::PbAuthenticatePasswordRequestDecoder, AuthenticatePasswordStruct,
    },
};

use super::action::{AuthenticatePasswordAction, AuthenticatePasswordMaterial};

use crate::auth::user::password::remote::authenticate::infra::AuthenticatePasswordRequestDecoder;

pub struct AuthenticatePasswordFeature<'a> {
    authenticate: AuthenticatePasswordStruct<'a>,
    issue: IssueAuthTicketStruct<'a>,
    encode: EncodeAuthTicketStruct<'a>,
}

impl<'a> AuthenticatePasswordFeature<'a> {
    pub fn action(
        feature: &'a AuthAppFeature,
        metadata: &'a MetadataMap,
    ) -> AuthenticatePasswordAction<Self> {
        AuthenticatePasswordAction::with_material(Self {
            authenticate: AuthenticatePasswordStruct::new(&feature.auth, metadata),
            issue: IssueAuthTicketStruct::new(&feature.auth),
            encode: EncodeAuthTicketStruct::new(&feature.auth),
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

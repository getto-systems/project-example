use tonic::metadata::MetadataMap;

use crate::auth::user::password::reset::_common::y_protobuf::service::ResetPasswordRequestPb;

use crate::x_outside_feature::remote::auth::feature::AuthAppFeature;

use crate::auth::{
    ticket::remote::{
        encode::init::EncodeAuthTicketStruct, issue::init::IssueAuthTicketStruct,
    },
    user::password::reset::remote::reset::init::{
        request_decoder::PbResetPasswordRequestDecoder, ResetPasswordStruct,
    },
};

use super::action::{ResetPasswordAction, ResetPasswordMaterial};

use crate::auth::user::password::reset::remote::reset::infra::ResetPasswordRequestDecoder;

pub struct ResetPasswordFeature<'a> {
    reset: ResetPasswordStruct<'a>,
    issue: IssueAuthTicketStruct<'a>,
    encode: EncodeAuthTicketStruct<'a>,
}

impl<'a> ResetPasswordFeature<'a> {
    pub fn action(feature: &'a AuthAppFeature, metadata: &'a MetadataMap) -> ResetPasswordAction<Self> {
        ResetPasswordAction::with_material(Self {
            reset: ResetPasswordStruct::new(&feature.auth, metadata),
            issue: IssueAuthTicketStruct::new(&feature.auth),
            encode: EncodeAuthTicketStruct::new(&feature.auth),
        })
    }
    pub fn request_decoder(request: ResetPasswordRequestPb) -> impl ResetPasswordRequestDecoder {
        PbResetPasswordRequestDecoder::new(request)
    }
}

impl<'a> ResetPasswordMaterial for ResetPasswordFeature<'a> {
    type Reset = ResetPasswordStruct<'a>;
    type Issue = IssueAuthTicketStruct<'a>;
    type Encode = EncodeAuthTicketStruct<'a>;

    fn reset(&self) -> &Self::Reset {
        &self.reset
    }
    fn issue(&self) -> &Self::Issue {
        &self.issue
    }
    fn encode(&self) -> &Self::Encode {
        &self.encode
    }
}

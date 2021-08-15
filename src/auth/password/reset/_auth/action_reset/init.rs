use tonic::metadata::MetadataMap;

use crate::auth::password::reset::_auth::reset::infra::ResetPasswordRequestDecoder;
use crate::auth::password::reset::_auth::reset::init::request_decoder::PbResetPasswordRequestDecoder;
use crate::auth::password::reset::_common::y_protobuf::service::ResetPasswordRequestPb;

use crate::auth::_auth::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::{
    auth_ticket::_auth::{
        encode::init::EncodeAuthTicketStruct, issue::init::IssueAuthTicketStruct,
    },
    password::reset::_auth::reset::init::ResetPasswordStruct,
};

use super::action::{ResetPasswordAction, ResetPasswordMaterial};

pub struct ResetPasswordFeature<'a> {
    reset: ResetPasswordStruct<'a>,
    issue: IssueAuthTicketStruct<'a>,
    encode: EncodeAuthTicketStruct<'a>,
}

impl<'a> ResetPasswordFeature<'a> {
    pub fn action(
        feature: &'a AuthOutsideFeature,
        metadata: &'a MetadataMap,
    ) -> ResetPasswordAction<Self> {
        ResetPasswordAction::with_material(Self {
            reset: ResetPasswordStruct::new(feature, metadata),
            issue: IssueAuthTicketStruct::new(feature),
            encode: EncodeAuthTicketStruct::new(feature),
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

    fn extract(self) -> (Self::Reset, Self::Issue, Self::Encode) {
        (self.reset, self.issue, self.encode)
    }
}

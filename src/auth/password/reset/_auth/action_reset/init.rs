use tonic::metadata::MetadataMap;

use crate::auth::password::reset::_common::y_protobuf::service::ResetPasswordRequestPb;

use crate::auth::_auth::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::{
    auth_ticket::_auth::{
        encode::init::EncodeAuthTicketStruct, issue::init::IssueAuthTicketStruct,
    },
    password::reset::_auth::reset::init::ResetPasswordStruct,
};

use super::action::{ResetPasswordAction, ResetPasswordMaterial};

impl<'a> ResetPasswordAction<ResetPasswordFeature<'a>> {
    pub fn new(
        feature: &'a AuthOutsideFeature,
        metadata: MetadataMap,
        request: ResetPasswordRequestPb,
    ) -> Self {
        Self::with_material(ResetPasswordFeature::new(feature, metadata, request))
    }
}

pub struct ResetPasswordFeature<'a> {
    reset: ResetPasswordStruct<'a>,
    issue: IssueAuthTicketStruct<'a>,
    encode: EncodeAuthTicketStruct<'a>,
}

impl<'a> ResetPasswordFeature<'a> {
    fn new(
        feature: &'a AuthOutsideFeature,
        metadata: MetadataMap,
        request: ResetPasswordRequestPb,
    ) -> Self {
        Self {
            reset: ResetPasswordStruct::new(feature, metadata, request),
            issue: IssueAuthTicketStruct::new(feature),
            encode: EncodeAuthTicketStruct::new(feature),
        }
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

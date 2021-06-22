use actix_web::HttpRequest;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::{
    auth_ticket::_api::{
        encode::init::ResetPasswordEncodeAuthTicketStruct, issue::init::IssueAuthTicketStruct,
    },
    password::reset::_api::reset::init::ResetPasswordStruct,
};

use super::action::{ResetPasswordAction, ResetPasswordMaterial};

impl<'a> ResetPasswordAction<ResetPasswordFeature<'a>> {
    pub fn new(feature: &'a AuthOutsideFeature, request: &'a HttpRequest, body: String) -> Self {
        Self::with_material(ResetPasswordFeature::new(feature, request, body))
    }
}

pub struct ResetPasswordFeature<'a> {
    reset: ResetPasswordStruct<'a>,
    issue: IssueAuthTicketStruct<'a>,
    encode: ResetPasswordEncodeAuthTicketStruct<'a>,
}

impl<'a> ResetPasswordFeature<'a> {
    fn new(feature: &'a AuthOutsideFeature, request: &'a HttpRequest, body: String) -> Self {
        Self {
            reset: ResetPasswordStruct::new(feature, request, body),
            issue: IssueAuthTicketStruct::new(feature),
            encode: ResetPasswordEncodeAuthTicketStruct::new(feature),
        }
    }
}

impl<'a> ResetPasswordMaterial for ResetPasswordFeature<'a> {
    type Reset = ResetPasswordStruct<'a>;
    type Issue = IssueAuthTicketStruct<'a>;
    type Encode = ResetPasswordEncodeAuthTicketStruct<'a>;

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

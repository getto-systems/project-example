use tonic::Request;

use crate::auth::_auth::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::auth_ticket::_auth::{
    encode::init::EncodeAuthTicketStruct, validate::init::TicketValidateAuthTokenStruct,
};

use super::action::{RenewAuthTicketAction, RenewAuthTicketMaterial};

impl<'a> RenewAuthTicketAction<RenewAuthTicketFeature<'a>> {
    pub fn new<T>(feature: &'a AuthOutsideFeature, request: &'a Request<T>) -> Self {
        Self::with_material(RenewAuthTicketFeature::new(feature, request))
    }
}

pub struct RenewAuthTicketFeature<'a> {
    validate: TicketValidateAuthTokenStruct<'a>,
    encode: EncodeAuthTicketStruct<'a>,
}

impl<'a> RenewAuthTicketFeature<'a> {
    fn new<T>(feature: &'a AuthOutsideFeature, request: &'a Request<T>) -> Self {
        Self {
            validate: TicketValidateAuthTokenStruct::new(feature, request),
            encode: EncodeAuthTicketStruct::new(feature),
        }
    }
}

impl<'a> RenewAuthTicketMaterial for RenewAuthTicketFeature<'a> {
    type Validate = TicketValidateAuthTokenStruct<'a>;
    type Encode = EncodeAuthTicketStruct<'a>;

    fn validate(&self) -> &Self::Validate {
        &self.validate
    }
    fn encode(&self) -> &Self::Encode {
        &self.encode
    }
}

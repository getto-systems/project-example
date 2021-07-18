use tonic::Request;

use crate::auth::_auth::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::auth_ticket::_auth::{
    encode::init::EncodeAuthTicketStruct, validate::init::TicketValidateAuthTokenStruct,
};

use super::action::{RenewAuthTicketAction, RenewAuthTicketMaterial};

impl<'a, T> RenewAuthTicketAction<RenewAuthTicketFeature<'a, T>> {
    pub fn new(feature: &'a AuthOutsideFeature, request: &'a Request<T>) -> Self {
        Self::with_material(RenewAuthTicketFeature::new(feature, request))
    }
}

pub struct RenewAuthTicketFeature<'a, T> {
    validate: TicketValidateAuthTokenStruct<'a, T>,
    encode: EncodeAuthTicketStruct<'a>,
}

impl<'a, T> RenewAuthTicketFeature<'a, T> {
    fn new(feature: &'a AuthOutsideFeature, request: &'a Request<T>) -> Self {
        Self {
            validate: TicketValidateAuthTokenStruct::new(feature, request),
            encode: EncodeAuthTicketStruct::new(feature),
        }
    }
}

impl<'a, T> RenewAuthTicketMaterial for RenewAuthTicketFeature<'a, T> {
    type Validate = TicketValidateAuthTokenStruct<'a, T>;
    type Encode = EncodeAuthTicketStruct<'a>;

    fn validate(&self) -> &Self::Validate {
        &self.validate
    }
    fn encode(&self) -> &Self::Encode {
        &self.encode
    }
}

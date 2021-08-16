use tonic::metadata::MetadataMap;

use crate::auth::_auth::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::auth_ticket::_auth::{
    encode::init::EncodeAuthTicketStruct, validate::init::TicketValidateAuthTokenStruct,
};

use super::action::{RenewAuthTicketAction, RenewAuthTicketMaterial};

pub struct RenewAuthTicketFeature<'a> {
    validate: TicketValidateAuthTokenStruct<'a>,
    encode: EncodeAuthTicketStruct<'a>,
}

impl<'a> RenewAuthTicketFeature<'a> {
    pub fn action(
        feature: &'a AuthOutsideFeature,
        metadata: &'a MetadataMap,
    ) -> RenewAuthTicketAction<Self> {
        RenewAuthTicketAction::with_material(Self {
            validate: TicketValidateAuthTokenStruct::new(feature, metadata),
            encode: EncodeAuthTicketStruct::new(feature),
        })
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

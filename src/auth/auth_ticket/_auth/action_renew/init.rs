use tonic::metadata::MetadataMap;

use crate::x_outside_feature::_auth::feature::AuthAppFeature;

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
        feature: &'a AuthAppFeature,
        metadata: &'a MetadataMap,
    ) -> RenewAuthTicketAction<Self> {
        RenewAuthTicketAction::with_material(Self {
            validate: TicketValidateAuthTokenStruct::new(&feature.auth, metadata),
            encode: EncodeAuthTicketStruct::new(&feature.auth),
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

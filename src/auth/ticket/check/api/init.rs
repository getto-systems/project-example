use tonic::metadata::MetadataMap;

use crate::x_outside_feature::auth::feature::AuthAppFeature;

use crate::auth::ticket::{
    encode::init::EncodeAuthTicketStruct, validate::init::TicketValidateAuthTokenStruct,
};

use super::action::{CheckAuthTicketAction, CheckAuthTicketMaterial};

pub struct CheckAuthTicketStruct<'a> {
    validate: TicketValidateAuthTokenStruct<'a>,
    encode: EncodeAuthTicketStruct<'a>,
}

impl<'a> CheckAuthTicketStruct<'a> {
    pub fn action(
        feature: &'a AuthAppFeature,
        metadata: &'a MetadataMap,
    ) -> CheckAuthTicketAction<Self> {
        CheckAuthTicketAction::with_material(Self {
            validate: TicketValidateAuthTokenStruct::new(feature, metadata),
            encode: EncodeAuthTicketStruct::new(feature),
        })
    }
}

impl<'a> CheckAuthTicketMaterial for CheckAuthTicketStruct<'a> {
    type Validate = TicketValidateAuthTokenStruct<'a>;
    type Encode = EncodeAuthTicketStruct<'a>;

    fn validate(&self) -> &Self::Validate {
        &self.validate
    }
    fn encode(&self) -> &Self::Encode {
        &self.encode
    }
}

use tonic::metadata::MetadataMap;

use crate::x_outside_feature::auth::feature::AuthAppFeature;

use crate::auth::ticket::{
    encode::init::EncodeAuthTicketStruct, validate::init::AuthenticateTicketStruct,
};

use super::action::{CheckAuthTicketAction, CheckAuthTicketMaterial};

pub struct CheckAuthTicketStruct<'a> {
    validate: AuthenticateTicketStruct<'a>,
    encode: EncodeAuthTicketStruct<'a>,
}

impl<'a> CheckAuthTicketStruct<'a> {
    pub fn action(
        feature: &'a AuthAppFeature,
        metadata: &'a MetadataMap,
    ) -> CheckAuthTicketAction<Self> {
        CheckAuthTicketAction::with_material(Self {
            validate: AuthenticateTicketStruct::new(feature, metadata),
            encode: EncodeAuthTicketStruct::new(feature),
        })
    }
}

impl<'a> CheckAuthTicketMaterial for CheckAuthTicketStruct<'a> {
    type Authenticate = AuthenticateTicketStruct<'a>;
    type Encode = EncodeAuthTicketStruct<'a>;

    fn authenticate(&self) -> &Self::Authenticate {
        &self.validate
    }
    fn encode(&self) -> &Self::Encode {
        &self.encode
    }
}

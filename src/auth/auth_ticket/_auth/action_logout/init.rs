use tonic::metadata::MetadataMap;

use crate::auth::_auth::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::auth_ticket::_auth::{
    discard::init::DiscardAuthTicketStruct, validate::init::TicketValidateAuthTokenStruct,
};

use super::action::{LogoutAction, LogoutMaterial};

pub struct LogoutFeature<'a> {
    validate: TicketValidateAuthTokenStruct<'a>,
    discard: DiscardAuthTicketStruct<'a>,
}

impl<'a> LogoutFeature<'a> {
    pub fn action(
        feature: &'a AuthOutsideFeature,
        metadata: &'a MetadataMap,
    ) -> LogoutAction<Self> {
        LogoutAction::with_material(Self {
            validate: TicketValidateAuthTokenStruct::new(feature, metadata),
            discard: DiscardAuthTicketStruct::new(feature),
        })
    }
}

impl<'a> LogoutMaterial for LogoutFeature<'a> {
    type Validate = TicketValidateAuthTokenStruct<'a>;
    type Discard = DiscardAuthTicketStruct<'a>;

    fn extract(self) -> (Self::Validate, Self::Discard) {
        (self.validate, self.discard)
    }
}

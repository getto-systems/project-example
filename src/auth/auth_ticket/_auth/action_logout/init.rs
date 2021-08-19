use tonic::metadata::MetadataMap;

use crate::x_outside_feature::_auth::feature::AppFeature;

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
        feature: &'a AppFeature,
        metadata: &'a MetadataMap,
    ) -> LogoutAction<Self> {
        LogoutAction::with_material(Self {
            validate: TicketValidateAuthTokenStruct::new(&feature.auth, metadata),
            discard: DiscardAuthTicketStruct::new(&feature.auth),
        })
    }
}

#[async_trait::async_trait]
impl<'a> LogoutMaterial for LogoutFeature<'a> {
    type Validate = TicketValidateAuthTokenStruct<'a>;
    type Discard = DiscardAuthTicketStruct<'a>;

    fn validate(&self) -> &Self::Validate {
        &self.validate
    }
    fn discard(&self) -> &Self::Discard {
        &self.discard
    }
}

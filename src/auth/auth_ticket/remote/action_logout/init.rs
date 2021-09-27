use tonic::metadata::MetadataMap;

use crate::x_outside_feature::_auth::feature::AuthAppFeature;

use crate::auth::auth_ticket::remote::discard::init::DiscardAuthTicketStruct;

use super::action::{LogoutAction, LogoutMaterial};

pub struct LogoutFeature<'a> {
    discard: DiscardAuthTicketStruct<'a>,
}

impl<'a> LogoutFeature<'a> {
    pub fn action(feature: &'a AuthAppFeature, metadata: &'a MetadataMap) -> LogoutAction<Self> {
        LogoutAction::with_material(Self {
            discard: DiscardAuthTicketStruct::new(&feature.auth, metadata),
        })
    }
}

#[async_trait::async_trait]
impl<'a> LogoutMaterial for LogoutFeature<'a> {
    type Discard = DiscardAuthTicketStruct<'a>;

    fn discard(&self) -> &Self::Discard {
        &self.discard
    }
}

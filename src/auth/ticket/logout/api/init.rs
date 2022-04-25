use tonic::metadata::MetadataMap;

use crate::x_outside_feature::auth::feature::AuthAppFeature;

use crate::auth::ticket::{
    kernel::init::ticket_repository::dynamodb::DynamoDbAuthTicketRepository,
    validate::init::AuthenticateTicketStruct,
};

use super::action::{LogoutAction, LogoutMaterial};

pub struct LogoutStruct<'a> {
    validate: AuthenticateTicketStruct<'a>,
    ticket_repository: DynamoDbAuthTicketRepository<'a>,
}

impl<'a> LogoutStruct<'a> {
    pub fn action(feature: &'a AuthAppFeature, metadata: &'a MetadataMap) -> LogoutAction<Self> {
        LogoutAction::with_material(Self {
            validate: AuthenticateTicketStruct::new(feature, metadata),
            ticket_repository: DynamoDbAuthTicketRepository::new(&feature.store),
        })
    }
}

#[async_trait::async_trait]
impl<'a> LogoutMaterial for LogoutStruct<'a> {
    type AuthenticateInfra = AuthenticateTicketStruct<'a>;
    type TicketRepository = DynamoDbAuthTicketRepository<'a>;

    fn authenticate(&self) -> &Self::AuthenticateInfra {
        &self.validate
    }
    fn ticket_repository(&self) -> &Self::TicketRepository {
        &self.ticket_repository
    }
}

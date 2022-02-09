use tonic::metadata::MetadataMap;

use crate::x_outside_feature::auth::feature::AuthAppFeature;

use crate::auth::ticket::{
    kernel::init::ticket_repository::dynamodb::DynamoDbAuthTicketRepository,
    validate::init::TicketValidateAuthTokenStruct,
};

use super::action::{LogoutAction, LogoutMaterial};

pub struct LogoutStruct<'a> {
    validate: TicketValidateAuthTokenStruct<'a>,
    ticket_repository: DynamoDbAuthTicketRepository<'a>,
}

impl<'a> LogoutStruct<'a> {
    pub fn action(feature: &'a AuthAppFeature, metadata: &'a MetadataMap) -> LogoutAction<Self> {
        LogoutAction::with_material(Self {
            validate: TicketValidateAuthTokenStruct::new(&feature.auth, metadata),
            ticket_repository: DynamoDbAuthTicketRepository::new(&feature.auth.store),
        })
    }
}

#[async_trait::async_trait]
impl<'a> LogoutMaterial for LogoutStruct<'a> {
    type ValidateInfra = TicketValidateAuthTokenStruct<'a>;
    type TicketRepository = DynamoDbAuthTicketRepository<'a>;

    fn validate(&self) -> &Self::ValidateInfra {
        &self.validate
    }
    fn ticket_repository(&self) -> &Self::TicketRepository {
        &self.ticket_repository
    }
}

use tonic::metadata::MetadataMap;

use crate::x_outside_feature::remote::auth::feature::AuthAppFeature;

use crate::auth::ticket::remote::{
    kernel::init::{clock::ChronoAuthClock, ticket_repository::MysqlAuthTicketRepository},
    validate::init::TicketValidateAuthTokenStruct,
};

use super::action::{LogoutAction, LogoutMaterial};

pub struct LogoutStruct<'a> {
    validate: TicketValidateAuthTokenStruct<'a>,
    clock: ChronoAuthClock,
    ticket_repository: MysqlAuthTicketRepository<'a>,
}

impl<'a> LogoutStruct<'a> {
    pub fn action(feature: &'a AuthAppFeature, metadata: &'a MetadataMap) -> LogoutAction<Self> {
        LogoutAction::with_material(Self {
            validate: TicketValidateAuthTokenStruct::new(&feature.auth, metadata),
            clock: ChronoAuthClock::new(),
            ticket_repository: MysqlAuthTicketRepository::new(&feature.auth.store.mysql),
        })
    }
}

#[async_trait::async_trait]
impl<'a> LogoutMaterial for LogoutStruct<'a> {
    type ValidateInfra = TicketValidateAuthTokenStruct<'a>;
    type Clock = ChronoAuthClock;
    type TicketRepository = MysqlAuthTicketRepository<'a>;

    fn validate(&self) -> &Self::ValidateInfra {
        &self.validate
    }
    fn clock(&self) -> &Self::Clock {
        &self.clock
    }
    fn ticket_repository(&self) -> &Self::TicketRepository {
        &self.ticket_repository
    }
}

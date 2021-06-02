use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use super::{super::kernel::infra::{
    clock::ChronoAuthClock, ticket_repository::MemoryAuthTicketRepository,
}, infra::id_generator::UuidAuthTicketIdGenerator};
use super::infra::{IssueAuthTicketConfig, IssueAuthTicketInfra};

pub struct IssueAuthTicketStruct<'a> {
    config: IssueAuthTicketConfig,
    clock: ChronoAuthClock,
    ticket_repository: MemoryAuthTicketRepository<'a>,
    ticket_id_generator: UuidAuthTicketIdGenerator,
}

impl<'a> IssueAuthTicketStruct<'a> {
    pub fn new(feature: &'a AuthOutsideFeature) -> Self {
        Self {
            config: IssueAuthTicketConfig {
                ticket_expansion_limit: feature.config.ticket_expansion_limit,
            },
            clock: ChronoAuthClock::new(),
            ticket_repository: MemoryAuthTicketRepository::new(&feature.store.ticket),
            ticket_id_generator: UuidAuthTicketIdGenerator::new(),
        }
    }
}

impl<'a> IssueAuthTicketInfra for IssueAuthTicketStruct<'a> {
    type Clock = ChronoAuthClock;
    type TicketRepository = MemoryAuthTicketRepository<'a>;
    type TicketIdGenerator = UuidAuthTicketIdGenerator;

    fn config(&self) -> &IssueAuthTicketConfig {
        &self.config
    }
    fn clock(&self) -> &Self::Clock {
        &self.clock
    }
    fn ticket_repository(&self) -> &Self::TicketRepository {
        &self.ticket_repository
    }
    fn ticket_id_generator(&self) -> &Self::TicketIdGenerator {
        &self.ticket_id_generator
    }
}

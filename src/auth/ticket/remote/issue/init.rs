pub(in crate::auth) mod id_generator;

use crate::auth::_auth::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::ticket::remote::kernel::init::{
    clock::ChronoAuthClock, ticket_repository::MysqlAuthTicketRepository,
};
use id_generator::UuidAuthTicketIdGenerator;

use super::infra::{IssueAuthTicketConfig, IssueAuthTicketInfra};

pub struct IssueAuthTicketStruct<'a> {
    clock: ChronoAuthClock,
    ticket_repository: MysqlAuthTicketRepository<'a>,
    ticket_id_generator: UuidAuthTicketIdGenerator,
    config: IssueAuthTicketConfig,
}

impl<'a> IssueAuthTicketStruct<'a> {
    pub fn new(feature: &'a AuthOutsideFeature) -> Self {
        Self {
            clock: ChronoAuthClock::new(),
            ticket_repository: MysqlAuthTicketRepository::new(&feature.store.mysql),
            ticket_id_generator: UuidAuthTicketIdGenerator::new(),
            config: IssueAuthTicketConfig {
                ticket_expansion_limit: feature.config.ticket_expansion_limit,
            },
        }
    }
}

impl<'a> IssueAuthTicketInfra for IssueAuthTicketStruct<'a> {
    type Clock = ChronoAuthClock;
    type TicketRepository = MysqlAuthTicketRepository<'a>;
    type TicketIdGenerator = UuidAuthTicketIdGenerator;

    fn clock(&self) -> &Self::Clock {
        &self.clock
    }
    fn ticket_repository(&self) -> &Self::TicketRepository {
        &self.ticket_repository
    }
    fn ticket_id_generator(&self) -> &Self::TicketIdGenerator {
        &self.ticket_id_generator
    }
    fn config(&self) -> &IssueAuthTicketConfig {
        &self.config
    }
}

#[cfg(test)]
pub mod test {
    use super::id_generator::test::StaticAuthTicketIdGenerator;
    use crate::auth::ticket::remote::kernel::init::{
        clock::test::StaticChronoAuthClock, ticket_repository::test::MemoryAuthTicketRepository,
    };

    use super::super::infra::{IssueAuthTicketConfig, IssueAuthTicketInfra};

    pub struct StaticIssueAuthTicketStruct<'a> {
        pub clock: StaticChronoAuthClock,
        pub ticket_repository: MemoryAuthTicketRepository<'a>,
        pub ticket_id_generator: StaticAuthTicketIdGenerator,
        pub config: IssueAuthTicketConfig,
    }

    impl<'a> IssueAuthTicketInfra for StaticIssueAuthTicketStruct<'a> {
        type Clock = StaticChronoAuthClock;
        type TicketRepository = MemoryAuthTicketRepository<'a>;
        type TicketIdGenerator = StaticAuthTicketIdGenerator;

        fn clock(&self) -> &Self::Clock {
            &self.clock
        }
        fn ticket_repository(&self) -> &Self::TicketRepository {
            &self.ticket_repository
        }
        fn ticket_id_generator(&self) -> &Self::TicketIdGenerator {
            &self.ticket_id_generator
        }
        fn config(&self) -> &IssueAuthTicketConfig {
            &self.config
        }
    }
}

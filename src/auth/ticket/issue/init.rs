pub mod id_generator;

use crate::auth::x_outside_feature::api::auth::feature::AuthOutsideFeature;

use crate::auth::ticket::{
    issue::init::id_generator::UuidAuthTicketIdGenerator,
    kernel::api::init::{clock::ChronoAuthClock, ticket_repository::mysql::MysqlAuthTicketRepository},
};

use super::method::{IssueAuthTicketConfig, IssueAuthTicketInfra};

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
    use crate::auth::ticket::{
        issue::init::id_generator::test::StaticAuthTicketIdGenerator,
        kernel::api::init::{
            clock::test::StaticChronoAuthClock, ticket_repository::memory::MemoryAuthTicketRepository,
        },
    };

    use super::super::method::{IssueAuthTicketConfig, IssueAuthTicketInfra};

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

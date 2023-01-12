mod id_generator;

use crate::x_outside_feature::auth::feature::AuthAppFeature;

use crate::auth::{
    kernel::init::clock::ChronoAuthClock,
    ticket::{
        issue::init::id_generator::UuidAuthTicketIdGenerator,
        kernel::init::ticket_repository::dynamodb::DynamoDbAuthTicketRepository,
    },
};

use crate::auth::ticket::issue::method::IssueAuthTicketInfra;

use crate::auth::ticket::issue::infra::IssueAuthTicketConfig;

pub struct ActiveIssueAuthTicketInfra<'a> {
    clock: ChronoAuthClock,
    ticket_repository: DynamoDbAuthTicketRepository<'a>,
    ticket_id_generator: UuidAuthTicketIdGenerator,
    config: IssueAuthTicketConfig,
}

impl<'a> ActiveIssueAuthTicketInfra<'a> {
    pub fn new(feature: &'a AuthAppFeature) -> Self {
        Self {
            clock: ChronoAuthClock::new(),
            ticket_repository: DynamoDbAuthTicketRepository::new(&feature.store),
            ticket_id_generator: UuidAuthTicketIdGenerator::new(),
            config: IssueAuthTicketConfig {
                authenticate_expansion_limit: feature.config.authenticate_expansion_limit,
            },
        }
    }
}

impl<'a> IssueAuthTicketInfra for ActiveIssueAuthTicketInfra<'a> {
    type Clock = ChronoAuthClock;
    type TicketRepository = DynamoDbAuthTicketRepository<'a>;
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
    pub use crate::auth::ticket::issue::init::id_generator::test::StaticAuthTicketIdGenerator;

    use crate::auth::{
        kernel::init::clock::test::StaticChronoAuthClock,
        ticket::kernel::init::ticket_repository::memory::MemoryAuthTicketRepository,
    };

    use crate::auth::ticket::issue::method::IssueAuthTicketInfra;

    use crate::auth::ticket::issue::infra::IssueAuthTicketConfig;

    pub struct StaticIssueAuthTicketInfra<'a> {
        pub clock: StaticChronoAuthClock,
        pub ticket_repository: MemoryAuthTicketRepository<'a>,
        pub ticket_id_generator: StaticAuthTicketIdGenerator,
        pub config: IssueAuthTicketConfig,
    }

    impl<'a> IssueAuthTicketInfra for StaticIssueAuthTicketInfra<'a> {
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

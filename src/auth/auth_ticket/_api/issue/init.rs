mod id_generator;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::auth_ticket::_api::kernel::init::AuthTicketStruct;
use id_generator::UuidAuthTicketIdGenerator;

use super::infra::{IssueAuthTicketConfig, IssueAuthTicketInfra};

pub struct IssueAuthTicketStruct<'a> {
    ticket_infra: AuthTicketStruct<'a>,
    ticket_id_generator: UuidAuthTicketIdGenerator,
    config: IssueAuthTicketConfig,
}

impl<'a> IssueAuthTicketStruct<'a> {
    pub fn new(feature: &'a AuthOutsideFeature) -> Self {
        Self {
            ticket_infra: AuthTicketStruct::new(feature),
            ticket_id_generator: UuidAuthTicketIdGenerator::new(),
            config: IssueAuthTicketConfig {
                ticket_expansion_limit: feature.config.ticket_expansion_limit,
            },
        }
    }
}

impl<'a> IssueAuthTicketInfra for IssueAuthTicketStruct<'a> {
    type TicketInfra = AuthTicketStruct<'a>;
    type TicketIdGenerator = UuidAuthTicketIdGenerator;

    fn ticket_infra(&self) -> &Self::TicketInfra {
        &self.ticket_infra
    }
    fn config(&self) -> &IssueAuthTicketConfig {
        &self.config
    }
    fn ticket_id_generator(&self) -> &Self::TicketIdGenerator {
        &self.ticket_id_generator
    }
}

#[cfg(test)]
pub mod test {
    pub use super::id_generator::test::StaticAuthTicketIdGenerator;
    use crate::auth::auth_ticket::_api::kernel::init::test::StaticAuthTicketStruct;

    use super::super::infra::{IssueAuthTicketConfig, IssueAuthTicketInfra};

    pub struct StaticIssueAuthTicketStruct<'a> {
        pub ticket_infra: StaticAuthTicketStruct<'a>,
        pub ticket_id_generator: StaticAuthTicketIdGenerator,
        pub config: IssueAuthTicketConfig,
    }

    impl<'a> IssueAuthTicketInfra for StaticIssueAuthTicketStruct<'a> {
        type TicketInfra = StaticAuthTicketStruct<'a>;
        type TicketIdGenerator = StaticAuthTicketIdGenerator;

        fn ticket_infra(&self) -> &Self::TicketInfra {
            &self.ticket_infra
        }
        fn config(&self) -> &IssueAuthTicketConfig {
            &self.config
        }
        fn ticket_id_generator(&self) -> &Self::TicketIdGenerator {
            &self.ticket_id_generator
        }
    }
}

use tonic::metadata::MetadataMap;

use crate::auth::_auth::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::ticket::{
    remote::kernel::init::{clock::ChronoAuthClock, ticket_repository::MysqlAuthTicketRepository},
    remote::validate::init::TicketValidateAuthTokenStruct,
};

use super::infra::DiscardAuthTicketInfra;

pub struct DiscardAuthTicketStruct<'a> {
    validate_infra: TicketValidateAuthTokenStruct<'a>,
    clock: ChronoAuthClock,
    ticket_repository: MysqlAuthTicketRepository<'a>,
}

impl<'a> DiscardAuthTicketStruct<'a> {
    pub fn new(feature: &'a AuthOutsideFeature, metadata: &'a MetadataMap) -> Self {
        Self {
            validate_infra: TicketValidateAuthTokenStruct::new(feature, metadata),
            clock: ChronoAuthClock::new(),
            ticket_repository: MysqlAuthTicketRepository::new(&feature.store.mysql),
        }
    }
}

impl<'a> DiscardAuthTicketInfra for DiscardAuthTicketStruct<'a> {
    type ValidateInfra = TicketValidateAuthTokenStruct<'a>;
    type Clock = ChronoAuthClock;
    type TicketRepository = MysqlAuthTicketRepository<'a>;

    fn validate_infra(&self) -> &Self::ValidateInfra {
        &self.validate_infra
    }
    fn clock(&self) -> &Self::Clock {
        &self.clock
    }
    fn ticket_repository(&self) -> &Self::TicketRepository {
        &self.ticket_repository
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::ticket::remote::{
        kernel::init::{
            clock::test::StaticChronoAuthClock, ticket_repository::test::MemoryAuthTicketRepository,
        },
        validate::init::test::StaticValidateAuthTokenStruct,
    };

    use super::super::infra::DiscardAuthTicketInfra;

    pub struct StaticDiscardAuthTicketStruct<'a> {
        pub validate_infra: StaticValidateAuthTokenStruct<'a>,
        pub clock: StaticChronoAuthClock,
        pub ticket_repository: MemoryAuthTicketRepository<'a>,
    }

    impl<'a> DiscardAuthTicketInfra for StaticDiscardAuthTicketStruct<'a> {
        type ValidateInfra = StaticValidateAuthTokenStruct<'a>;
        type Clock = StaticChronoAuthClock;
        type TicketRepository = MemoryAuthTicketRepository<'a>;

        fn validate_infra(&self) -> &Self::ValidateInfra {
            &self.validate_infra
        }
        fn clock(&self) -> &Self::Clock {
            &self.clock
        }
        fn ticket_repository(&self) -> &Self::TicketRepository {
            &self.ticket_repository
        }
    }
}

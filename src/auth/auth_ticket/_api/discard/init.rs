use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::auth_ticket::_api::kernel::init::{ChronoAuthClock, MemoryAuthTicketRepository};

use super::infra::DiscardAuthTicketInfra;

pub struct DiscardAuthTicketStruct<'a> {
    clock: ChronoAuthClock,
    ticket_repository: MemoryAuthTicketRepository<'a>,
}

impl<'a> DiscardAuthTicketStruct<'a> {
    pub fn new(feature: &'a AuthOutsideFeature) -> Self {
        Self {
            clock: ChronoAuthClock::new(),
            ticket_repository: MemoryAuthTicketRepository::new(&feature.store.ticket),
        }
    }
}

impl<'a> DiscardAuthTicketInfra for DiscardAuthTicketStruct<'a> {
    type Clock = ChronoAuthClock;
    type TicketRepository = MemoryAuthTicketRepository<'a>;

    fn clock(&self) -> &Self::Clock {
        &self.clock
    }
    fn ticket_repository(&self) -> &Self::TicketRepository {
        &self.ticket_repository
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::auth_ticket::_api::kernel::init::test::{
        MemoryAuthTicketRepository, StaticChronoAuthClock,
    };

    use super::super::infra::DiscardAuthTicketInfra;

    pub struct StaticDiscardAuthTicketStruct<'a> {
        pub clock: StaticChronoAuthClock,
        pub ticket_repository: MemoryAuthTicketRepository<'a>,
    }

    impl<'a> DiscardAuthTicketInfra for StaticDiscardAuthTicketStruct<'a> {
        type Clock = StaticChronoAuthClock;
        type TicketRepository = MemoryAuthTicketRepository<'a>;

        fn clock(&self) -> &Self::Clock {
            &self.clock
        }
        fn ticket_repository(&self) -> &Self::TicketRepository {
            &self.ticket_repository
        }
    }
}

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::auth_ticket::_api::kernel::init::AuthTicketStruct;

use super::infra::DiscardAuthTicketInfra;

pub struct DiscardAuthTicketStruct<'a> {
    ticket_infra: AuthTicketStruct<'a>,
}

impl<'a> DiscardAuthTicketStruct<'a> {
    pub fn new(feature: &'a AuthOutsideFeature) -> Self {
        Self {
            ticket_infra: AuthTicketStruct::new(feature),
        }
    }
}

impl<'a> DiscardAuthTicketInfra for DiscardAuthTicketStruct<'a> {
    type TicketInfra = AuthTicketStruct<'a>;

    fn ticket_infra(&self) -> &Self::TicketInfra {
        &self.ticket_infra
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::auth_ticket::_api::kernel::init::test::StaticAuthTicketStruct;

    use super::super::infra::DiscardAuthTicketInfra;

    pub struct StaticDiscardAuthTicketStruct<'a> {
        pub ticket_infra: StaticAuthTicketStruct<'a>,
    }

    impl<'a> DiscardAuthTicketInfra for StaticDiscardAuthTicketStruct<'a> {
        type TicketInfra = StaticAuthTicketStruct<'a>;

        fn ticket_infra(&self) -> &Self::TicketInfra {
            &self.ticket_infra
        }
    }
}

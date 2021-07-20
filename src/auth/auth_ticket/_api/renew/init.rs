mod messenger;
mod renew_service;

use actix_web::HttpRequest;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::auth_ticket::_api::kernel::init::TicketAuthHeaderStruct;
use {messenger::ProstRenewAuthTicketMessenger, renew_service::TonicRenewAuthTicketService};

use super::infra::RenewAuthTicketInfra;

pub struct RenewAuthTicketStruct<'a> {
    header_infra: TicketAuthHeaderStruct<'a>,
    renew_service: TonicRenewAuthTicketService<'a>,
    messenger: ProstRenewAuthTicketMessenger<'a>,
}

impl<'a> RenewAuthTicketStruct<'a> {
    pub fn new(
        feature: &'a AuthOutsideFeature,
        request_id: &'a str,
        request: &'a HttpRequest,
    ) -> Self {
        Self {
            header_infra: TicketAuthHeaderStruct::new(request),
            renew_service: TonicRenewAuthTicketService::new(&feature.service, request_id),
            messenger: ProstRenewAuthTicketMessenger::new(&feature.cookie),
        }
    }
}

impl<'a> RenewAuthTicketInfra for RenewAuthTicketStruct<'a> {
    type HeaderInfra = TicketAuthHeaderStruct<'a>;
    type RenewService = TonicRenewAuthTicketService<'a>;
    type Messenger = ProstRenewAuthTicketMessenger<'a>;

    fn header_infra(&self) -> &Self::HeaderInfra {
        &self.header_infra
    }
    fn renew_service(&self) -> &Self::RenewService {
        &self.renew_service
    }
    fn messenger(&self) -> &Self::Messenger {
        &self.messenger
    }
}

#[cfg(test)]
pub mod test {
    pub use super::{
        messenger::test::StaticRenewAuthTicketMessenger, renew_service::test::StaticRenewAuthTicketService,
    };
    use crate::auth::auth_ticket::_api::kernel::init::test::StaticAuthHeaderStruct;

    use super::super::infra::RenewAuthTicketInfra;

    pub struct StaticRenewAuthTicketStruct {
        pub header_infra: StaticAuthHeaderStruct,
        pub renew_service: StaticRenewAuthTicketService,
        pub messenger: StaticRenewAuthTicketMessenger,
    }

    impl RenewAuthTicketInfra for StaticRenewAuthTicketStruct {
        type HeaderInfra = StaticAuthHeaderStruct;
        type RenewService = StaticRenewAuthTicketService;
        type Messenger = StaticRenewAuthTicketMessenger;

        fn header_infra(&self) -> &Self::HeaderInfra {
            &self.header_infra
        }
        fn renew_service(&self) -> &Self::RenewService {
            &self.renew_service
        }
        fn messenger(&self) -> &Self::Messenger {
            &self.messenger
        }
    }
}

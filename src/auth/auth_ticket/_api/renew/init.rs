mod renew_service;
mod response_encoder;

use actix_web::HttpRequest;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::auth_ticket::_api::kernel::init::{AuthTokenStruct, TicketAuthHeaderStruct};
use crate::auth::auth_ticket::_api::renew::init::response_encoder::ProstRenewAuthTicketResponseEncoder;
use renew_service::TonicRenewAuthTicketService;

use super::infra::RenewAuthTicketInfra;

pub struct RenewAuthTicketStruct<'a> {
    header_infra: TicketAuthHeaderStruct<'a>,
    token_infra: AuthTokenStruct<'a>,
    renew_service: TonicRenewAuthTicketService<'a>,
    response_encoder: ProstRenewAuthTicketResponseEncoder,
}

impl<'a> RenewAuthTicketStruct<'a> {
    pub fn new(
        feature: &'a AuthOutsideFeature,
        request_id: &'a str,
        request: &'a HttpRequest,
    ) -> Self {
        Self {
            header_infra: TicketAuthHeaderStruct::new(request),
            token_infra: AuthTokenStruct::new(feature),
            renew_service: TonicRenewAuthTicketService::new(&feature.service, request_id),
            response_encoder: ProstRenewAuthTicketResponseEncoder,
        }
    }
}

impl<'a> RenewAuthTicketInfra for RenewAuthTicketStruct<'a> {
    type HeaderInfra = TicketAuthHeaderStruct<'a>;
    type TokenInfra = AuthTokenStruct<'a>;
    type RenewService = TonicRenewAuthTicketService<'a>;
    type ResponseEncoder = ProstRenewAuthTicketResponseEncoder;

    fn header_infra(&self) -> &Self::HeaderInfra {
        &self.header_infra
    }
    fn token_infra(&self) -> &Self::TokenInfra {
        &self.token_infra
    }
    fn renew_service(&self) -> &Self::RenewService {
        &self.renew_service
    }
    fn response_encoder(&self) -> &Self::ResponseEncoder {
        &self.response_encoder
    }
}

#[cfg(test)]
pub mod test {
    pub use super::renew_service::test::StaticRenewAuthTicketService;
    pub use super::response_encoder::test::StaticRenewAuthTicketResponseEncoder;
    use crate::auth::auth_ticket::_api::kernel::init::test::{
        StaticAuthHeaderStruct, StaticAuthTokenStruct,
    };

    use super::super::infra::RenewAuthTicketInfra;

    pub struct StaticRenewAuthTicketStruct {
        pub header_infra: StaticAuthHeaderStruct,
        pub token_infra: StaticAuthTokenStruct,
        pub renew_service: StaticRenewAuthTicketService,
        pub response_encoder: StaticRenewAuthTicketResponseEncoder,
    }

    impl RenewAuthTicketInfra for StaticRenewAuthTicketStruct {
        type HeaderInfra = StaticAuthHeaderStruct;
        type TokenInfra = StaticAuthTokenStruct;
        type RenewService = StaticRenewAuthTicketService;
        type ResponseEncoder = StaticRenewAuthTicketResponseEncoder;

        fn header_infra(&self) -> &Self::HeaderInfra {
            &self.header_infra
        }
        fn token_infra(&self) -> &Self::TokenInfra {
            &self.token_infra
        }
        fn renew_service(&self) -> &Self::RenewService {
            &self.renew_service
        }
        fn response_encoder(&self) -> &Self::ResponseEncoder {
            &self.response_encoder
        }
    }
}

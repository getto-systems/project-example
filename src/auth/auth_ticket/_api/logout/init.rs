mod logout_service;

use actix_web::HttpRequest;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::auth_ticket::_api::{
    kernel::init::TicketAuthHeaderStruct, logout::init::logout_service::TonicLogoutService,
};

use super::infra::LogoutInfra;

pub struct LogoutStruct<'a> {
    header_infra: TicketAuthHeaderStruct<'a>,
    logout_service: TonicLogoutService<'a>,
}

impl<'a> LogoutStruct<'a> {
    pub fn new(
        feature: &'a AuthOutsideFeature,
        request_id: &'a str,
        request: &'a HttpRequest,
    ) -> Self {
        Self {
            header_infra: TicketAuthHeaderStruct::new(request),
            logout_service: TonicLogoutService::new(&feature.service, request_id),
        }
    }
}

impl<'a> LogoutInfra for LogoutStruct<'a> {
    type HeaderInfra = TicketAuthHeaderStruct<'a>;
    type LogoutService = TonicLogoutService<'a>;

    fn header_infra(&self) -> &Self::HeaderInfra {
        &self.header_infra
    }
    fn logout_service(&self) -> &Self::LogoutService {
        &self.logout_service
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::auth_ticket::_api::kernel::init::test::StaticAuthHeaderStruct;
    pub use crate::auth::auth_ticket::_api::logout::init::logout_service::test::StaticLogoutService;

    use super::super::infra::LogoutInfra;

    pub struct StaticLogoutStruct {
        pub header_infra: StaticAuthHeaderStruct,
        pub logout_service: StaticLogoutService,
    }

    impl LogoutInfra for StaticLogoutStruct {
        type HeaderInfra = StaticAuthHeaderStruct;
        type LogoutService = StaticLogoutService;

        fn header_infra(&self) -> &Self::HeaderInfra {
            &self.header_infra
        }
        fn logout_service(&self) -> &Self::LogoutService {
            &self.logout_service
        }
    }
}

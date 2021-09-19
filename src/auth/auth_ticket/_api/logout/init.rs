pub(in crate::auth) mod logout_service;

use actix_web::HttpRequest;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::auth_ticket::_api::{
    kernel::init::service_metadata::TicketServiceMetadata,
    logout::init::logout_service::TonicLogoutService,
};

use super::infra::LogoutInfra;

pub struct LogoutStruct<'a> {
    service_metadata: TicketServiceMetadata<'a>,
    logout_service: TonicLogoutService<'a>,
}

impl<'a> LogoutStruct<'a> {
    pub fn new(
        feature: &'a AuthOutsideFeature,
        request_id: &'a str,
        request: &'a HttpRequest,
    ) -> Self {
        Self {
            service_metadata: TicketServiceMetadata::new(request, &feature.key),
            logout_service: TonicLogoutService::new(&feature.service, request_id),
        }
    }
}

impl<'a> LogoutInfra for LogoutStruct<'a> {
    type ServiceMetadata = TicketServiceMetadata<'a>;
    type LogoutService = TonicLogoutService<'a>;

    fn service_metadata(&self) -> &Self::ServiceMetadata {
        &self.service_metadata
    }
    fn logout_service(&self) -> &Self::LogoutService {
        &self.logout_service
    }
}

#[cfg(test)]
pub mod test {
    use super::logout_service::test::StaticLogoutService;
    use crate::auth::auth_ticket::_common::kernel::init::service_metadata::test::StaticAuthServiceMetadata;

    use super::super::infra::LogoutInfra;

    pub struct StaticLogoutStruct {
        pub service_metadata: StaticAuthServiceMetadata,
        pub logout_service: StaticLogoutService,
    }

    impl LogoutInfra for StaticLogoutStruct {
        type ServiceMetadata = StaticAuthServiceMetadata;
        type LogoutService = StaticLogoutService;

        fn service_metadata(&self) -> &Self::ServiceMetadata {
            &self.service_metadata
        }
        fn logout_service(&self) -> &Self::LogoutService {
            &self.logout_service
        }
    }
}

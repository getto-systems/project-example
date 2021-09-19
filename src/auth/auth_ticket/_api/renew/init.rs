pub(in crate::auth) mod renew_service;
pub(in crate::auth) mod response_encoder;

use actix_web::HttpRequest;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::auth_ticket::_api::{
    kernel::init::{
        response_builder::CookieAuthTokenResponseBuilder, service_metadata::TicketServiceMetadata,
    },
    renew::init::response_encoder::ProstRenewAuthTicketResponseEncoder,
};
use renew_service::TonicRenewAuthTicketService;

use super::infra::RenewAuthTicketInfra;

pub struct RenewAuthTicketStruct<'a> {
    service_metadata: TicketServiceMetadata<'a>,
    renew_service: TonicRenewAuthTicketService<'a>,
    response_builder: CookieAuthTokenResponseBuilder<'a>,
    response_encoder: ProstRenewAuthTicketResponseEncoder,
}

impl<'a> RenewAuthTicketStruct<'a> {
    pub fn new(
        feature: &'a AuthOutsideFeature,
        request_id: &'a str,
        request: &'a HttpRequest,
    ) -> Self {
        Self {
            service_metadata: TicketServiceMetadata::new(request, &feature.key),
            renew_service: TonicRenewAuthTicketService::new(&feature.service, request_id),
            response_builder: CookieAuthTokenResponseBuilder::new(&feature.cookie),
            response_encoder: ProstRenewAuthTicketResponseEncoder,
        }
    }
}

impl<'a> RenewAuthTicketInfra for RenewAuthTicketStruct<'a> {
    type ServiceMetadata = TicketServiceMetadata<'a>;
    type RenewService = TonicRenewAuthTicketService<'a>;
    type ResponseBuilder = CookieAuthTokenResponseBuilder<'a>;
    type ResponseEncoder = ProstRenewAuthTicketResponseEncoder;

    fn service_metadata(&self) -> &Self::ServiceMetadata {
        &self.service_metadata
    }
    fn renew_service(&self) -> &Self::RenewService {
        &self.renew_service
    }
    fn response_builder(&self) -> &Self::ResponseBuilder {
        &self.response_builder
    }
    fn response_encoder(&self) -> &Self::ResponseEncoder {
        &self.response_encoder
    }
}

#[cfg(test)]
pub mod test {
    use super::renew_service::test::StaticRenewAuthTicketService;
    use super::response_encoder::test::StaticRenewAuthTicketResponseEncoder;
    use crate::auth::auth_ticket::{
        _api::kernel::init::response_builder::test::StaticAuthTokenResponseBuilder,
        _common::kernel::init::service_metadata::test::StaticAuthServiceMetadata,
    };

    use super::super::infra::RenewAuthTicketInfra;

    pub struct StaticRenewAuthTicketStruct {
        pub service_metadata: StaticAuthServiceMetadata,
        pub response_builder: StaticAuthTokenResponseBuilder,
        pub renew_service: StaticRenewAuthTicketService,
        pub response_encoder: StaticRenewAuthTicketResponseEncoder,
    }

    impl RenewAuthTicketInfra for StaticRenewAuthTicketStruct {
        type ServiceMetadata = StaticAuthServiceMetadata;
        type RenewService = StaticRenewAuthTicketService;
        type ResponseBuilder = StaticAuthTokenResponseBuilder;
        type ResponseEncoder = StaticRenewAuthTicketResponseEncoder;

        fn service_metadata(&self) -> &Self::ServiceMetadata {
            &self.service_metadata
        }
        fn renew_service(&self) -> &Self::RenewService {
            &self.renew_service
        }
        fn response_builder(&self) -> &Self::ResponseBuilder {
            &self.response_builder
        }
        fn response_encoder(&self) -> &Self::ResponseEncoder {
            &self.response_encoder
        }
    }
}

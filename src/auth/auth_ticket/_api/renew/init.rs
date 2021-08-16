pub(in crate::auth) mod renew_service;
pub(in crate::auth) mod response_encoder;

use actix_web::HttpRequest;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::auth_ticket::_api::{
    kernel::init::{
        nonce_header::ActixWebAuthNonceHeader, response_builder::CookieAuthTokenResponseBuilder,
        token_header::TicketAuthTokenHeader,
    },
    renew::init::response_encoder::ProstRenewAuthTicketResponseEncoder,
};
use renew_service::TonicRenewAuthTicketService;

use super::infra::RenewAuthTicketInfra;

pub struct RenewAuthTicketStruct<'a> {
    nonce_header: ActixWebAuthNonceHeader<'a>,
    token_header: TicketAuthTokenHeader<'a>,
    response_builder: CookieAuthTokenResponseBuilder<'a>,
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
            nonce_header: ActixWebAuthNonceHeader::new(request),
            token_header: TicketAuthTokenHeader::new(request),
            response_builder: CookieAuthTokenResponseBuilder::new(&feature.cookie),
            renew_service: TonicRenewAuthTicketService::new(&feature.service, request_id),
            response_encoder: ProstRenewAuthTicketResponseEncoder,
        }
    }
}

impl<'a> RenewAuthTicketInfra for RenewAuthTicketStruct<'a> {
    type NonceHeader = ActixWebAuthNonceHeader<'a>;
    type TokenHeader = TicketAuthTokenHeader<'a>;
    type ResponseBuilder = CookieAuthTokenResponseBuilder<'a>;
    type RenewService = TonicRenewAuthTicketService<'a>;
    type ResponseEncoder = ProstRenewAuthTicketResponseEncoder;

    fn nonce_header(&self) -> &Self::NonceHeader {
        &self.nonce_header
    }
    fn token_header(&self) -> &Self::TokenHeader {
        &self.token_header
    }
    fn response_builder(&self) -> &Self::ResponseBuilder {
        &self.response_builder
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
    use super::renew_service::test::StaticRenewAuthTicketService;
    use super::response_encoder::test::StaticRenewAuthTicketResponseEncoder;
    use crate::auth::auth_ticket::_api::kernel::init::{
        nonce_header::test::StaticAuthNonceHeader,
        response_builder::test::StaticAuthTokenResponseBuilder,
        token_header::test::StaticAuthTokenHeader,
    };

    use super::super::infra::RenewAuthTicketInfra;

    pub struct StaticRenewAuthTicketStruct {
        pub nonce_header: StaticAuthNonceHeader,
        pub token_header: StaticAuthTokenHeader,
        pub response_builder: StaticAuthTokenResponseBuilder,
        pub renew_service: StaticRenewAuthTicketService,
        pub response_encoder: StaticRenewAuthTicketResponseEncoder,
    }

    impl RenewAuthTicketInfra for StaticRenewAuthTicketStruct {
        type NonceHeader = StaticAuthNonceHeader;
        type TokenHeader = StaticAuthTokenHeader;
        type ResponseBuilder = StaticAuthTokenResponseBuilder;
        type RenewService = StaticRenewAuthTicketService;
        type ResponseEncoder = StaticRenewAuthTicketResponseEncoder;

        fn nonce_header(&self) -> &Self::NonceHeader {
            &self.nonce_header
        }
        fn token_header(&self) -> &Self::TokenHeader {
            &self.token_header
        }
        fn response_builder(&self) -> &Self::ResponseBuilder {
            &self.response_builder
        }
        fn renew_service(&self) -> &Self::RenewService {
            &self.renew_service
        }
        fn response_encoder(&self) -> &Self::ResponseEncoder {
            &self.response_encoder
        }
    }
}

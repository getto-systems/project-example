pub(in crate::auth) mod request_decoder;
pub(in crate::auth) mod reset_service;
pub(in crate::auth) mod response_encoder;

use actix_web::HttpRequest;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::auth_ticket::_api::kernel::init::{AuthTokenStruct, TicketAuthHeaderStruct};
use reset_service::TonicResetPasswordService;
use response_encoder::ProstResetPasswordResponseEncoder;

use crate::auth::password::reset::_api::reset::infra::ResetPasswordInfra;

pub struct ResetPasswordStruct<'a> {
    header_infra: TicketAuthHeaderStruct<'a>,
    token_infra: AuthTokenStruct<'a>,
    reset_service: TonicResetPasswordService<'a>,
    response_encoder: ProstResetPasswordResponseEncoder,
}

impl<'a> ResetPasswordStruct<'a> {
    pub fn new(
        feature: &'a AuthOutsideFeature,
        request_id: &'a str,
        request: &'a HttpRequest,
    ) -> Self {
        Self {
            header_infra: TicketAuthHeaderStruct::new(request),
            token_infra: AuthTokenStruct::new(feature),
            reset_service: TonicResetPasswordService::new(&feature.service, request_id),
            response_encoder: ProstResetPasswordResponseEncoder,
        }
    }
}

impl<'a> ResetPasswordInfra for ResetPasswordStruct<'a> {
    type HeaderInfra = TicketAuthHeaderStruct<'a>;
    type TokenInfra = AuthTokenStruct<'a>;
    type ResetService = TonicResetPasswordService<'a>;
    type ResponseEncoder = ProstResetPasswordResponseEncoder;

    fn header_infra(&self) -> &Self::HeaderInfra {
        &self.header_infra
    }
    fn token_infra(&self) -> &Self::TokenInfra {
        &self.token_infra
    }
    fn reset_service(&self) -> &Self::ResetService {
        &self.reset_service
    }
    fn response_encoder(&self) -> &Self::ResponseEncoder {
        &self.response_encoder
    }
}

#[cfg(test)]
pub mod test {
    use super::reset_service::test::StaticResetPasswordService;
    use super::response_encoder::test::StaticResetPasswordResponseEncoder;

    use crate::auth::auth_ticket::_api::kernel::init::test::{
        StaticAuthHeaderStruct, StaticAuthTokenStruct,
    };

    use super::super::infra::ResetPasswordInfra;

    pub struct StaticResetPasswordStruct {
        pub header_infra: StaticAuthHeaderStruct,
        pub token_infra: StaticAuthTokenStruct,
        pub reset_service: StaticResetPasswordService,
        pub response_encoder: StaticResetPasswordResponseEncoder,
    }

    impl ResetPasswordInfra for StaticResetPasswordStruct {
        type HeaderInfra = StaticAuthHeaderStruct;
        type TokenInfra = StaticAuthTokenStruct;
        type ResetService = StaticResetPasswordService;
        type ResponseEncoder = StaticResetPasswordResponseEncoder;

        fn header_infra(&self) -> &Self::HeaderInfra {
            &self.header_infra
        }
        fn token_infra(&self) -> &Self::TokenInfra {
            &self.token_infra
        }
        fn reset_service(&self) -> &Self::ResetService {
            &self.reset_service
        }
        fn response_encoder(&self) -> &Self::ResponseEncoder {
            &self.response_encoder
        }
    }
}

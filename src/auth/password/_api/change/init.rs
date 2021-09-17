pub(in crate::auth) mod change_service;
pub(in crate::auth) mod request_decoder;
pub(in crate::auth) mod response_encoder;

use actix_web::HttpRequest;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::auth_ticket::_api::kernel::init::{
    nonce_header::ActixWebAuthNonceHeader, token_header::TicketAuthTokenHeader,
};
use crate::auth::auth_ticket::_api::validate::init::ValidateApiTokenStruct;
use change_service::TonicChangePasswordService;
use response_encoder::ProstChangePasswordResponseEncoder;

use super::infra::ChangePasswordInfra;

pub struct ChangePasswordStruct<'a> {
    nonce_header: ActixWebAuthNonceHeader<'a>,
    token_header: TicketAuthTokenHeader<'a>,
    validate_infra: ValidateApiTokenStruct<'a>,
    change_service: TonicChangePasswordService<'a>,
    response_encoder: ProstChangePasswordResponseEncoder,
}

impl<'a> ChangePasswordStruct<'a> {
    pub fn new(
        feature: &'a AuthOutsideFeature,
        request_id: &'a str,
        request: &'a HttpRequest,
    ) -> Self {
        Self {
            nonce_header: ActixWebAuthNonceHeader::new(request),
            token_header: TicketAuthTokenHeader::new(request),
            validate_infra: ValidateApiTokenStruct::new(&feature, request_id, request),
            change_service: TonicChangePasswordService::new(&feature.service, request_id),
            response_encoder: ProstChangePasswordResponseEncoder,
        }
    }
}

impl<'a> ChangePasswordInfra for ChangePasswordStruct<'a> {
    type NonceHeader = ActixWebAuthNonceHeader<'a>;
    type TokenHeader = TicketAuthTokenHeader<'a>;
    type ValidateInfra = ValidateApiTokenStruct<'a>;
    type ChangeService = TonicChangePasswordService<'a>;
    type ResponseEncoder = ProstChangePasswordResponseEncoder;

    fn nonce_header(&self) -> &Self::NonceHeader {
        &self.nonce_header
    }
    fn token_header(&self) -> &Self::TokenHeader {
        &self.token_header
    }
    fn validate_infra(&self) -> &Self::ValidateInfra {
        &self.validate_infra
    }
    fn change_service(&self) -> &Self::ChangeService {
        &self.change_service
    }
    fn response_encoder(&self) -> &Self::ResponseEncoder {
        &self.response_encoder
    }
}

#[cfg(test)]
pub mod test {
    use super::change_service::test::StaticChangePasswordService;
    use super::response_encoder::test::StaticChangePasswordResponseEncoder;

    use crate::auth::auth_ticket::_api::{
        kernel::init::{
            nonce_header::test::StaticAuthNonceHeader, token_header::test::StaticAuthTokenHeader,
        },
        validate::init::test::StaticValidateApiTokenStruct,
    };

    use super::super::infra::ChangePasswordInfra;

    pub struct StaticChangePasswordStruct {
        pub nonce_header: StaticAuthNonceHeader,
        pub token_header: StaticAuthTokenHeader,
        pub validate_infra: StaticValidateApiTokenStruct,
        pub change_service: StaticChangePasswordService,
        pub response_encoder: StaticChangePasswordResponseEncoder,
    }

    impl ChangePasswordInfra for StaticChangePasswordStruct {
        type NonceHeader = StaticAuthNonceHeader;
        type TokenHeader = StaticAuthTokenHeader;
        type ValidateInfra = StaticValidateApiTokenStruct;
        type ChangeService = StaticChangePasswordService;
        type ResponseEncoder = StaticChangePasswordResponseEncoder;

        fn nonce_header(&self) -> &Self::NonceHeader {
            &self.nonce_header
        }
        fn token_header(&self) -> &Self::TokenHeader {
            &self.token_header
        }
        fn validate_infra(&self) -> &Self::ValidateInfra {
            &self.validate_infra
        }
        fn change_service(&self) -> &Self::ChangeService {
            &self.change_service
        }
        fn response_encoder(&self) -> &Self::ResponseEncoder {
            &self.response_encoder
        }
    }
}

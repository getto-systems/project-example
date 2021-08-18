pub(in crate::auth) mod validate_service;

use actix_web::HttpRequest;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::auth_ticket::_api::{
    kernel::init::{nonce_header::ActixWebAuthNonceHeader, token_header::ApiAuthTokenHeader},
    validate::init::validate_service::TonicValidateService,
};

use super::infra::ValidateInfra;

pub struct ValidateStruct<'a> {
    nonce_header: ActixWebAuthNonceHeader<'a>,
    token_header: ApiAuthTokenHeader<'a>,
    validate_service: TonicValidateService<'a>,
}

impl<'a> ValidateStruct<'a> {
    pub fn new(
        feature: &'a AuthOutsideFeature,
        request_id: &'a str,
        request: &'a HttpRequest,
    ) -> Self {
        Self {
            nonce_header: ActixWebAuthNonceHeader::new(request),
            token_header: ApiAuthTokenHeader::new(request),
            validate_service: TonicValidateService::new(&feature.service, request_id),
        }
    }
}

impl<'a> ValidateInfra for ValidateStruct<'a> {
    type NonceHeader = ActixWebAuthNonceHeader<'a>;
    type TokenHeader = ApiAuthTokenHeader<'a>;
    type ValidateService = TonicValidateService<'a>;

    fn nonce_header(&self) -> &Self::NonceHeader {
        &self.nonce_header
    }
    fn token_header(&self) -> &Self::TokenHeader {
        &self.token_header
    }
    fn validate_service(&self) -> &Self::ValidateService {
        &self.validate_service
    }
}

#[cfg(test)]
pub mod test {
    use super::validate_service::test::StaticValidateService;
    use crate::auth::auth_ticket::_api::kernel::init::{
        nonce_header::test::StaticAuthNonceHeader, token_header::test::StaticAuthTokenHeader,
    };

    use super::super::infra::ValidateInfra;

    pub struct StaticLogoutStruct {
        pub nonce_header: StaticAuthNonceHeader,
        pub token_header: StaticAuthTokenHeader,
        pub validate_service: StaticValidateService,
    }

    impl ValidateInfra for StaticLogoutStruct {
        type NonceHeader = StaticAuthNonceHeader;
        type TokenHeader = StaticAuthTokenHeader;
        type ValidateService = StaticValidateService;

        fn nonce_header(&self) -> &Self::NonceHeader {
            &self.nonce_header
        }
        fn token_header(&self) -> &Self::TokenHeader {
            &self.token_header
        }
        fn validate_service(&self) -> &Self::ValidateService {
            &self.validate_service
        }
    }
}

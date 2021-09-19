pub(in crate::auth) mod change_service;
pub(in crate::auth) mod request_decoder;
pub(in crate::auth) mod response_encoder;

use actix_web::HttpRequest;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::auth_ticket::_api::kernel::init::{
    nonce_metadata::ActixWebAuthNonceMetadata, token_metadata::ApiAuthTokenMetadata,
};
use change_service::TonicChangePasswordService;
use response_encoder::ProstChangePasswordResponseEncoder;

use super::infra::ChangePasswordInfra;

pub struct ChangePasswordStruct<'a> {
    nonce_metadata: ActixWebAuthNonceMetadata<'a>,
    token_metadata: ApiAuthTokenMetadata<'a>,
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
            nonce_metadata: ActixWebAuthNonceMetadata::new(request),
            token_metadata: ApiAuthTokenMetadata::new(request),
            change_service: TonicChangePasswordService::new(&feature.service, request_id),
            response_encoder: ProstChangePasswordResponseEncoder,
        }
    }
}

impl<'a> ChangePasswordInfra for ChangePasswordStruct<'a> {
    type NonceMetadata = ActixWebAuthNonceMetadata<'a>;
    type TokenMetadata = ApiAuthTokenMetadata<'a>;
    type ChangeService = TonicChangePasswordService<'a>;
    type ResponseEncoder = ProstChangePasswordResponseEncoder;

    fn nonce_metadata(&self) -> &Self::NonceMetadata {
        &self.nonce_metadata
    }
    fn token_metadata(&self) -> &Self::TokenMetadata {
        &self.token_metadata
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

    use crate::auth::auth_ticket::_common::kernel::init::{
        nonce_metadata::test::StaticAuthNonceMetadata,
        token_metadata::test::StaticAuthTokenMetadata,
    };

    use super::super::infra::ChangePasswordInfra;

    pub struct StaticChangePasswordStruct {
        pub nonce_metadata: StaticAuthNonceMetadata,
        pub token_metadata: StaticAuthTokenMetadata,
        pub change_service: StaticChangePasswordService,
        pub response_encoder: StaticChangePasswordResponseEncoder,
    }

    impl ChangePasswordInfra for StaticChangePasswordStruct {
        type NonceMetadata = StaticAuthNonceMetadata;
        type TokenMetadata = StaticAuthTokenMetadata;
        type ChangeService = StaticChangePasswordService;
        type ResponseEncoder = StaticChangePasswordResponseEncoder;

        fn nonce_metadata(&self) -> &Self::NonceMetadata {
            &self.nonce_metadata
        }
        fn token_metadata(&self) -> &Self::TokenMetadata {
            &self.token_metadata
        }
        fn change_service(&self) -> &Self::ChangeService {
            &self.change_service
        }
        fn response_encoder(&self) -> &Self::ResponseEncoder {
            &self.response_encoder
        }
    }
}

#[cfg(test)]
pub mod test {
    pub use crate::auth::auth_ticket::_common::kernel::init::{
        nonce_metadata::test::StaticAuthNonceMetadata,
        token_metadata::test::StaticAuthTokenMetadata,
    };
    pub use crate::auth::auth_ticket::_common::validate::init::{
        test::StaticValidateApiTokenStruct, validate_service::test::StaticValidateService,
    };
}

// TODO api から validate はしないようにしたいのであとで消す
use actix_web::HttpRequest;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::auth_ticket::{
    _api::kernel::init::{
        nonce_metadata::ActixWebAuthNonceMetadata, token_metadata::ApiAuthTokenMetadata,
    },
    _common::validate::init::validate_service::TonicValidateService,
};

use crate::auth::_common::infra::ValidateApiTokenInfra;

pub struct ValidateApiTokenStruct<'a> {
    nonce_metadata: ActixWebAuthNonceMetadata<'a>,
    token_metadata: ApiAuthTokenMetadata<'a>,
    validate_service: TonicValidateService<'a>,
}

impl<'a> ValidateApiTokenStruct<'a> {
    pub fn new(
        feature: &'a AuthOutsideFeature,
        request_id: &'a str,
        request: &'a HttpRequest,
    ) -> Self {
        Self {
            nonce_metadata: ActixWebAuthNonceMetadata::new(request),
            token_metadata: ApiAuthTokenMetadata::new(request),
            validate_service: TonicValidateService::new(&feature.service, request_id),
        }
    }
}

impl<'a> ValidateApiTokenInfra for ValidateApiTokenStruct<'a> {
    type NonceMetadata = ActixWebAuthNonceMetadata<'a>;
    type TokenMetadata = ApiAuthTokenMetadata<'a>;
    type ValidateService = TonicValidateService<'a>;

    fn nonce_metadata(&self) -> &Self::NonceMetadata {
        &self.nonce_metadata
    }
    fn token_metadata(&self) -> &Self::TokenMetadata {
        &self.token_metadata
    }
    fn validate_service(&self) -> &Self::ValidateService {
        &self.validate_service
    }
}

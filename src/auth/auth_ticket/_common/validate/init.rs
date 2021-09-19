pub(in crate::auth) mod validate_service;

#[cfg(test)]
pub mod test {
    use super::validate_service::test::StaticValidateService;
    use crate::auth::auth_ticket::_common::kernel::init::{
        nonce_metadata::test::StaticAuthNonceMetadata,
        token_metadata::test::StaticAuthTokenMetadata,
    };

    use super::super::infra::ValidateApiTokenInfra;

    pub struct StaticValidateApiTokenStruct {
        pub nonce_metadata: StaticAuthNonceMetadata,
        pub token_metadata: StaticAuthTokenMetadata,
        pub validate_service: StaticValidateService,
    }

    impl ValidateApiTokenInfra for StaticValidateApiTokenStruct {
        type NonceMetadata = StaticAuthNonceMetadata;
        type TokenMetadata = StaticAuthTokenMetadata;
        type ValidateService = StaticValidateService;

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
}

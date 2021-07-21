mod token_decoder;
mod token_metadata;

use tonic::Request;

use crate::auth::_auth::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::auth_ticket::_auth::kernel::init::CheckAuthNonceStruct;
use token_decoder::{JwtApiTokenDecoder, JwtAuthTokenDecoder};
use token_metadata::{ApiAuthTokenMetadata, TicketAuthTokenMetadata};

use super::infra::{ValidateAuthTokenConfig, ValidateAuthTokenInfra};

use crate::auth::auth_user::_common::kernel::data::RequireAuthRoles;

pub struct TicketValidateAuthTokenStruct<'a> {
    check_nonce_infra: CheckAuthNonceStruct<'a>,
    token_metadata: TicketAuthTokenMetadata<'a>,
    token_decoder: JwtAuthTokenDecoder<'a>,
    config: ValidateAuthTokenConfig,
}

impl<'a> TicketValidateAuthTokenStruct<'a> {
    pub fn new<T>(feature: &'a AuthOutsideFeature, request: &'a Request<T>) -> Self {
        Self {
            check_nonce_infra: CheckAuthNonceStruct::new(feature, request.metadata().clone()),
            token_metadata: TicketAuthTokenMetadata::new(request.metadata()),
            token_decoder: JwtAuthTokenDecoder::new(&feature.secret.ticket.decoding_key),
            config: ValidateAuthTokenConfig {
                require_roles: RequireAuthRoles::Nothing, // ticket 検証では role は不問
            },
        }
    }
}

impl<'a> ValidateAuthTokenInfra for TicketValidateAuthTokenStruct<'a> {
    type CheckNonceInfra = CheckAuthNonceStruct<'a>;
    type TokenMetadata = TicketAuthTokenMetadata<'a>;
    type TokenDecoder = JwtAuthTokenDecoder<'a>;

    fn extract(
        self,
    ) -> (
        Self::CheckNonceInfra,
        Self::TokenMetadata,
        Self::TokenDecoder,
        ValidateAuthTokenConfig,
    ) {
        (
            self.check_nonce_infra,
            self.token_metadata,
            self.token_decoder,
            self.config,
        )
    }
}

pub struct ApiValidateAuthTokenStruct<'a> {
    check_nonce_infra: CheckAuthNonceStruct<'a>,
    token_metadata: ApiAuthTokenMetadata<'a>,
    token_decoder: JwtApiTokenDecoder<'a>,
    config: ValidateAuthTokenConfig,
}

impl<'a> ApiValidateAuthTokenStruct<'a> {
    pub fn new<T>(
        feature: &'a AuthOutsideFeature,
        request: &'a Request<T>,
        require_roles: RequireAuthRoles,
    ) -> Self {
        Self {
            check_nonce_infra: CheckAuthNonceStruct::new(feature, request.metadata().clone()),
            config: ValidateAuthTokenConfig { require_roles },
            token_metadata: ApiAuthTokenMetadata::new(request.metadata()),
            token_decoder: JwtApiTokenDecoder::new(&feature.secret.api.decoding_key),
        }
    }
}

impl<'a> ValidateAuthTokenInfra for ApiValidateAuthTokenStruct<'a> {
    type CheckNonceInfra = CheckAuthNonceStruct<'a>;
    type TokenMetadata = ApiAuthTokenMetadata<'a>;
    type TokenDecoder = JwtApiTokenDecoder<'a>;

    fn extract(
        self,
    ) -> (
        Self::CheckNonceInfra,
        Self::TokenMetadata,
        Self::TokenDecoder,
        ValidateAuthTokenConfig,
    ) {
        (
            self.check_nonce_infra,
            self.token_metadata,
            self.token_decoder,
            self.config,
        )
    }
}

#[cfg(test)]
pub mod test {
    pub use super::token_decoder::test::StaticAuthTokenDecoder;
    pub use super::token_metadata::test::StaticAuthTokenMetadata;
    use crate::auth::auth_ticket::_auth::kernel::init::test::StaticCheckAuthNonceStruct;

    use super::super::infra::{ValidateAuthTokenConfig, ValidateAuthTokenInfra};

    pub struct StaticValidateAuthTokenStruct<'a> {
        pub check_nonce_infra: StaticCheckAuthNonceStruct<'a>,
        pub token_metadata: StaticAuthTokenMetadata,
        pub token_decoder: StaticAuthTokenDecoder,
        pub config: ValidateAuthTokenConfig,
    }

    impl<'a> ValidateAuthTokenInfra for StaticValidateAuthTokenStruct<'a> {
        type CheckNonceInfra = StaticCheckAuthNonceStruct<'a>;
        type TokenMetadata = StaticAuthTokenMetadata;
        type TokenDecoder = StaticAuthTokenDecoder;

        fn extract(
            self,
        ) -> (
            Self::CheckNonceInfra,
            Self::TokenMetadata,
            Self::TokenDecoder,
            ValidateAuthTokenConfig,
        ) {
            (
                self.check_nonce_infra,
                self.token_metadata,
                self.token_decoder,
                self.config,
            )
        }
    }
}

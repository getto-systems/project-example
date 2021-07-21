mod token_decoder;
mod token_metadata;

use tonic::Request;

use crate::auth::_auth::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::auth_ticket::_auth::kernel::init::{AuthTicketStruct, CheckAuthNonceStruct};
use token_decoder::{JwtApiTokenDecoder, JwtAuthTokenDecoder};
use token_metadata::{ApiAuthTokenMetadata, TicketAuthTokenMetadata};

use super::infra::{ValidateAuthTokenConfig, ValidateAuthTokenInfra};

use crate::auth::auth_user::_common::kernel::data::RequireAuthRoles;

pub struct TicketValidateAuthTokenStruct<'a> {
    check_nonce_infra: CheckAuthNonceStruct<'a>,
    ticket_infra: AuthTicketStruct<'a>,
    token_metadata: TicketAuthTokenMetadata<'a>,
    token_validator: JwtAuthTokenDecoder<'a>,
    config: ValidateAuthTokenConfig,
}

impl<'a> TicketValidateAuthTokenStruct<'a> {
    pub fn new<T>(feature: &'a AuthOutsideFeature, request: &'a Request<T>) -> Self {
        Self {
            check_nonce_infra: CheckAuthNonceStruct::new(feature, request.metadata().clone()),
            ticket_infra: AuthTicketStruct::new(feature),
            token_metadata: TicketAuthTokenMetadata::new(request.metadata()),
            token_validator: JwtAuthTokenDecoder::new(&feature.secret.ticket.decoding_key),
            config: ValidateAuthTokenConfig {
                require_roles: RequireAuthRoles::Nothing, // ticket 検証では role は不問
            },
        }
    }
}

impl<'a> ValidateAuthTokenInfra for TicketValidateAuthTokenStruct<'a> {
    type CheckNonceInfra = CheckAuthNonceStruct<'a>;
    type TicketInfra = AuthTicketStruct<'a>;
    type TokenMetadata = TicketAuthTokenMetadata<'a>;
    type TokenDecoder = JwtAuthTokenDecoder<'a>;

    fn check_nonce_infra(&self) -> &Self::CheckNonceInfra {
        &self.check_nonce_infra
    }
    fn ticket_infra(&self) -> &Self::TicketInfra {
        &self.ticket_infra
    }
    fn token_metadata(&self) -> &Self::TokenMetadata {
        &self.token_metadata
    }
    fn token_decoder(&self) -> &Self::TokenDecoder {
        &self.token_validator
    }
    fn config(&self) -> &ValidateAuthTokenConfig {
        &self.config
    }
}

pub struct ApiValidateAuthTokenStruct<'a> {
    check_nonce_infra: CheckAuthNonceStruct<'a>,
    ticket_infra: AuthTicketStruct<'a>,
    token_header: ApiAuthTokenMetadata<'a>,
    token_validator: JwtApiTokenDecoder<'a>,
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
            ticket_infra: AuthTicketStruct::new(feature),
            config: ValidateAuthTokenConfig { require_roles },
            token_header: ApiAuthTokenMetadata::new(request.metadata()),
            token_validator: JwtApiTokenDecoder::new(&feature.secret.api.decoding_key),
        }
    }
}

impl<'a> ValidateAuthTokenInfra for ApiValidateAuthTokenStruct<'a> {
    type CheckNonceInfra = CheckAuthNonceStruct<'a>;
    type TicketInfra = AuthTicketStruct<'a>;
    type TokenMetadata = ApiAuthTokenMetadata<'a>;
    type TokenDecoder = JwtApiTokenDecoder<'a>;

    fn check_nonce_infra(&self) -> &Self::CheckNonceInfra {
        &self.check_nonce_infra
    }
    fn ticket_infra(&self) -> &Self::TicketInfra {
        &self.ticket_infra
    }
    fn token_metadata(&self) -> &Self::TokenMetadata {
        &self.token_header
    }
    fn token_decoder(&self) -> &Self::TokenDecoder {
        &self.token_validator
    }
    fn config(&self) -> &ValidateAuthTokenConfig {
        &self.config
    }
}

#[cfg(test)]
pub mod test {
    pub use super::token_decoder::test::StaticAuthTokenDecoder;
    pub use super::token_metadata::test::StaticAuthTokenMetadata;
    use crate::auth::auth_ticket::_auth::kernel::init::test::{
        StaticAuthTicketStruct, StaticCheckAuthNonceStruct,
    };

    use super::super::infra::{ValidateAuthTokenConfig, ValidateAuthTokenInfra};

    pub struct StaticValidateAuthTokenStruct<'a> {
        pub check_nonce_infra: StaticCheckAuthNonceStruct<'a>,
        pub ticket_infra: StaticAuthTicketStruct<'a>,
        pub token_metadata: StaticAuthTokenMetadata,
        pub token_decoder: StaticAuthTokenDecoder,
        pub config: ValidateAuthTokenConfig,
    }

    impl<'a> ValidateAuthTokenInfra for StaticValidateAuthTokenStruct<'a> {
        type CheckNonceInfra = StaticCheckAuthNonceStruct<'a>;
        type TicketInfra = StaticAuthTicketStruct<'a>;
        type TokenMetadata = StaticAuthTokenMetadata;
        type TokenDecoder = StaticAuthTokenDecoder;

        fn check_nonce_infra(&self) -> &Self::CheckNonceInfra {
            &self.check_nonce_infra
        }
        fn ticket_infra(&self) -> &Self::TicketInfra {
            &self.ticket_infra
        }
        fn token_metadata(&self) -> &Self::TokenMetadata {
            &self.token_metadata
        }
        fn token_decoder(&self) -> &Self::TokenDecoder {
            &self.token_decoder
        }
        fn config(&self) -> &ValidateAuthTokenConfig {
            &self.config
        }
    }
}

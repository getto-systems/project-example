mod token_decoder;
mod token_header;

use actix_web::HttpRequest;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::auth_ticket::_api::kernel::init::{AuthTicketStruct, CheckAuthNonceStruct};
use token_decoder::{JwtApiTokenDecoder, JwtAuthTokenDecoder};
use token_header::{ApiAuthTokenHeader, TicketAuthTokenHeader};

use super::infra::{ValidateAuthTokenConfig, ValidateAuthTokenInfra};

use crate::auth::auth_user::_api::kernel::data::RequireAuthRoles;

pub struct TicketValidateAuthTokenStruct<'a> {
    check_nonce_infra: CheckAuthNonceStruct<'a>,
    ticket_infra: AuthTicketStruct<'a>,
    token_header: TicketAuthTokenHeader,
    token_validator: JwtAuthTokenDecoder<'a>,
    config: ValidateAuthTokenConfig,
}

impl<'a> TicketValidateAuthTokenStruct<'a> {
    pub fn new(feature: &'a AuthOutsideFeature, request: &'a HttpRequest) -> Self {
        Self {
            check_nonce_infra: CheckAuthNonceStruct::new(feature, request),
            ticket_infra: AuthTicketStruct::new(feature),
            token_header: TicketAuthTokenHeader::new(request.clone()),
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
    type TokenHeader = TicketAuthTokenHeader;
    type TokenDecoder = JwtAuthTokenDecoder<'a>;

    fn check_nonce_infra(&self) -> &Self::CheckNonceInfra {
        &self.check_nonce_infra
    }
    fn ticket_infra(&self) -> &Self::TicketInfra {
        &self.ticket_infra
    }
    fn token_header(&self) -> &Self::TokenHeader {
        &self.token_header
    }
    fn token_validator(&self) -> &Self::TokenDecoder {
        &self.token_validator
    }
    fn config(&self) -> &ValidateAuthTokenConfig {
        &self.config
    }
}

pub struct ApiValidateAuthTokenStruct<'a> {
    check_nonce_infra: CheckAuthNonceStruct<'a>,
    ticket_infra: AuthTicketStruct<'a>,
    token_header: ApiAuthTokenHeader,
    token_validator: JwtApiTokenDecoder<'a>,
    config: ValidateAuthTokenConfig,
}

impl<'a> ApiValidateAuthTokenStruct<'a> {
    pub fn new(
        feature: &'a AuthOutsideFeature,
        request: &'a HttpRequest,
        require_roles: RequireAuthRoles,
    ) -> Self {
        Self {
            check_nonce_infra: CheckAuthNonceStruct::new(feature, request),
            ticket_infra: AuthTicketStruct::new(feature),
            config: ValidateAuthTokenConfig { require_roles },
            token_header: ApiAuthTokenHeader::new(request.clone()),
            token_validator: JwtApiTokenDecoder::new(&feature.secret.api.decoding_key),
        }
    }
}

impl<'a> ValidateAuthTokenInfra for ApiValidateAuthTokenStruct<'a> {
    type CheckNonceInfra = CheckAuthNonceStruct<'a>;
    type TicketInfra = AuthTicketStruct<'a>;
    type TokenHeader = ApiAuthTokenHeader;
    type TokenDecoder = JwtApiTokenDecoder<'a>;

    fn check_nonce_infra(&self) -> &Self::CheckNonceInfra {
        &self.check_nonce_infra
    }
    fn ticket_infra(&self) -> &Self::TicketInfra {
        &self.ticket_infra
    }
    fn token_header(&self) -> &Self::TokenHeader {
        &self.token_header
    }
    fn token_validator(&self) -> &Self::TokenDecoder {
        &self.token_validator
    }
    fn config(&self) -> &ValidateAuthTokenConfig {
        &self.config
    }
}

#[cfg(test)]
pub mod test {
    pub use super::token_decoder::test::StaticAuthTokenDecoder;
    pub use super::token_header::test::StaticAuthTokenHeader;
    use crate::auth::auth_ticket::_api::kernel::init::test::{
        StaticAuthTicketStruct, StaticCheckAuthNonceStruct,
    };

    use super::super::infra::{ValidateAuthTokenConfig, ValidateAuthTokenInfra};

    pub struct StaticValidateAuthTokenStruct<'a> {
        pub check_nonce_infra: StaticCheckAuthNonceStruct<'a>,
        pub ticket_infra: StaticAuthTicketStruct<'a>,
        pub token_header: StaticAuthTokenHeader,
        pub token_validator: StaticAuthTokenDecoder,
        pub config: ValidateAuthTokenConfig,
    }

    impl<'a> ValidateAuthTokenInfra for StaticValidateAuthTokenStruct<'a> {
        type CheckNonceInfra = StaticCheckAuthNonceStruct<'a>;
        type TicketInfra = StaticAuthTicketStruct<'a>;
        type TokenHeader = StaticAuthTokenHeader;
        type TokenDecoder = StaticAuthTokenDecoder;

        fn check_nonce_infra(&self) -> &Self::CheckNonceInfra {
            &self.check_nonce_infra
        }
        fn ticket_infra(&self) -> &Self::TicketInfra {
            &self.ticket_infra
        }
        fn token_header(&self) -> &Self::TokenHeader {
            &self.token_header
        }
        fn token_validator(&self) -> &Self::TokenDecoder {
            &self.token_validator
        }
        fn config(&self) -> &ValidateAuthTokenConfig {
            &self.config
        }
    }
}

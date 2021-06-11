use actix_web::HttpRequest;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use super::super::kernel::infra::{
    clock::ChronoAuthClock, nonce_header::ActixWebAuthNonceHeader,
    nonce_repository::MemoryAuthNonceRepository, ticket_repository::MemoryAuthTicketRepository,
    AuthNonceConfig,
};
use super::infra::{
    token_header::{ApiAuthTokenHeader, TicketAuthTokenHeader},
    token_validator::{JwtApiTokenValidator, JwtAuthTokenValidator},
    ValidateAuthTokenConfig, ValidateAuthTokenInfra,
};

use crate::auth::auth_user::_api::kernel::data::RequireAuthRoles;

pub struct TicketValidateAuthTokenStruct<'a> {
    config: ValidateAuthTokenConfig,
    nonce_config: AuthNonceConfig,
    clock: ChronoAuthClock,
    nonce_header: ActixWebAuthNonceHeader,
    token_header: TicketAuthTokenHeader,
    nonce_repository: MemoryAuthNonceRepository<'a>,
    ticket_repository: MemoryAuthTicketRepository<'a>,
    token_validator: JwtAuthTokenValidator<'a>,
}

impl<'a> TicketValidateAuthTokenStruct<'a> {
    pub fn new(request: HttpRequest, feature: &'a AuthOutsideFeature) -> Self {
        Self {
            config: ValidateAuthTokenConfig {
                require_roles: RequireAuthRoles::Nothing, // ticket 検証では role は不問
            },
            nonce_config: AuthNonceConfig {
                nonce_expires: feature.config.ticket_expires,
            },
            clock: ChronoAuthClock::new(),
            nonce_header: ActixWebAuthNonceHeader::new(request.clone()),
            token_header: TicketAuthTokenHeader::new(request.clone()),
            nonce_repository: MemoryAuthNonceRepository::new(&feature.store.nonce),
            ticket_repository: MemoryAuthTicketRepository::new(&feature.store.ticket),
            token_validator: JwtAuthTokenValidator::new(&feature.secret.ticket.decoding_key),
        }
    }
}

impl<'a> ValidateAuthTokenInfra for TicketValidateAuthTokenStruct<'a> {
    type Clock = ChronoAuthClock;
    type NonceHeader = ActixWebAuthNonceHeader;
    type TokenHeader = TicketAuthTokenHeader;
    type NonceRepository = MemoryAuthNonceRepository<'a>;
    type TicketRepository = MemoryAuthTicketRepository<'a>;
    type TokenValidator = JwtAuthTokenValidator<'a>;

    fn config(&self) -> &ValidateAuthTokenConfig {
        &self.config
    }
    fn nonce_config(&self) -> &AuthNonceConfig {
        &self.nonce_config
    }
    fn clock(&self) -> &Self::Clock {
        &self.clock
    }
    fn nonce_header(&self) -> &Self::NonceHeader {
        &self.nonce_header
    }
    fn token_header(&self) -> &Self::TokenHeader {
        &self.token_header
    }
    fn nonce_repository(&self) -> &Self::NonceRepository {
        &self.nonce_repository
    }
    fn ticket_repository(&self) -> &Self::TicketRepository {
        &self.ticket_repository
    }
    fn token_validator(&self) -> &Self::TokenValidator {
        &self.token_validator
    }
}

pub struct ApiValidateAuthTokenStruct<'a> {
    config: ValidateAuthTokenConfig,
    nonce_config: AuthNonceConfig,
    clock: ChronoAuthClock,
    nonce_header: ActixWebAuthNonceHeader,
    token_header: ApiAuthTokenHeader,
    nonce_repository: MemoryAuthNonceRepository<'a>,
    ticket_repository: MemoryAuthTicketRepository<'a>,
    token_validator: JwtApiTokenValidator<'a>,
}

impl<'a> ApiValidateAuthTokenStruct<'a> {
    pub fn new(
        request: HttpRequest,
        feature: &'a AuthOutsideFeature,
        require_roles: RequireAuthRoles,
    ) -> Self {
        Self {
            config: ValidateAuthTokenConfig { require_roles },
            nonce_config: AuthNonceConfig {
                nonce_expires: feature.config.api_expires,
            },
            clock: ChronoAuthClock::new(),
            nonce_header: ActixWebAuthNonceHeader::new(request.clone()),
            token_header: ApiAuthTokenHeader::new(request.clone()),
            nonce_repository: MemoryAuthNonceRepository::new(&feature.store.nonce),
            ticket_repository: MemoryAuthTicketRepository::new(&feature.store.ticket),
            token_validator: JwtApiTokenValidator::new(&feature.secret.api.decoding_key),
        }
    }
}

impl<'a> ValidateAuthTokenInfra for ApiValidateAuthTokenStruct<'a> {
    type Clock = ChronoAuthClock;
    type NonceHeader = ActixWebAuthNonceHeader;
    type TokenHeader = ApiAuthTokenHeader;
    type NonceRepository = MemoryAuthNonceRepository<'a>;
    type TicketRepository = MemoryAuthTicketRepository<'a>;
    type TokenValidator = JwtApiTokenValidator<'a>;

    fn config(&self) -> &ValidateAuthTokenConfig {
        &self.config
    }
    fn nonce_config(&self) -> &AuthNonceConfig {
        &self.nonce_config
    }
    fn nonce_header(&self) -> &Self::NonceHeader {
        &self.nonce_header
    }
    fn token_header(&self) -> &Self::TokenHeader {
        &self.token_header
    }
    fn clock(&self) -> &Self::Clock {
        &self.clock
    }
    fn nonce_repository(&self) -> &Self::NonceRepository {
        &self.nonce_repository
    }
    fn ticket_repository(&self) -> &Self::TicketRepository {
        &self.ticket_repository
    }
    fn token_validator(&self) -> &Self::TokenValidator {
        &self.token_validator
    }
}

#[cfg(test)]
pub mod test {
    use super::super::infra::{
        token_header::test::StaticAuthTokenHeader, token_validator::test::StaticAuthTokenValidator,
        ValidateAuthTokenConfig, ValidateAuthTokenInfra,
    };
    use crate::auth::auth_ticket::_api::kernel::infra::{
        clock::test::StaticChronoAuthClock, nonce_header::test::StaticAuthNonceHeader,
        nonce_repository::MemoryAuthNonceRepository, ticket_repository::MemoryAuthTicketRepository,
        AuthNonceConfig,
    };

    pub struct StaticValidateAuthTokenStruct<'a> {
        pub config: ValidateAuthTokenConfig,
        pub nonce_config: AuthNonceConfig,
        pub clock: StaticChronoAuthClock,
        pub nonce_header: StaticAuthNonceHeader,
        pub token_header: StaticAuthTokenHeader,
        pub nonce_repository: MemoryAuthNonceRepository<'a>,
        pub ticket_repository: MemoryAuthTicketRepository<'a>,
        pub token_validator: StaticAuthTokenValidator,
    }

    impl<'a> ValidateAuthTokenInfra for StaticValidateAuthTokenStruct<'a> {
        type Clock = StaticChronoAuthClock;
        type NonceHeader = StaticAuthNonceHeader;
        type TokenHeader = StaticAuthTokenHeader;
        type NonceRepository = MemoryAuthNonceRepository<'a>;
        type TicketRepository = MemoryAuthTicketRepository<'a>;
        type TokenValidator = StaticAuthTokenValidator;

        fn config(&self) -> &ValidateAuthTokenConfig {
            &self.config
        }
        fn nonce_config(&self) -> &AuthNonceConfig {
            &self.nonce_config
        }
        fn clock(&self) -> &Self::Clock {
            &self.clock
        }
        fn nonce_header(&self) -> &Self::NonceHeader {
            &self.nonce_header
        }
        fn token_header(&self) -> &Self::TokenHeader {
            &self.token_header
        }
        fn nonce_repository(&self) -> &Self::NonceRepository {
            &self.nonce_repository
        }
        fn ticket_repository(&self) -> &Self::TicketRepository {
            &self.ticket_repository
        }
        fn token_validator(&self) -> &Self::TokenValidator {
            &self.token_validator
        }
    }
}

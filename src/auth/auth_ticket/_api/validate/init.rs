use actix_web::HttpRequest;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use super::super::kernel::infra::{
    clock::ChronoAuthClock, header::ActixWebAuthNonceHeader,
    nonce_repository::MemoryAuthNonceRepository, ticket_repository::MemoryAuthTicketRepository,
    AuthNonceConfig,
};
use super::infra::{
    header::{ApiAuthTokenHeader, TicketAuthTokenHeader},
    token_validator::{ApiJwtTokenValidator, AuthJwtTokenValidator},
    ValidateAuthTokenInfra, ValidateConfig,
};

use crate::auth::auth_user::_api::kernel::data::RequireAuthRoles;

pub struct ValidateTicketTokenStruct<'a> {
    config: ValidateConfig,
    nonce_config: AuthNonceConfig,
    clock: ChronoAuthClock,
    nonce_header: ActixWebAuthNonceHeader,
    token_header: TicketAuthTokenHeader,
    nonce_repository: MemoryAuthNonceRepository<'a>,
    ticket_repository: MemoryAuthTicketRepository<'a>,
    token_validator: AuthJwtTokenValidator<'a>,
}

pub struct ValidateApiTokenStruct<'a> {
    config: ValidateConfig,
    nonce_config: AuthNonceConfig,
    clock: ChronoAuthClock,
    nonce_header: ActixWebAuthNonceHeader,
    token_header: ApiAuthTokenHeader,
    nonce_repository: MemoryAuthNonceRepository<'a>,
    ticket_repository: MemoryAuthTicketRepository<'a>,
    token_validator: ApiJwtTokenValidator<'a>,
}

impl<'a> ValidateTicketTokenStruct<'a> {
    pub fn new(request: HttpRequest, feature: &'a AuthOutsideFeature) -> Self {
        Self {
            config: ValidateConfig {
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
            token_validator: AuthJwtTokenValidator::new(&feature.secret.ticket.decoding_key),
        }
    }
}

impl<'a> ValidateAuthTokenInfra for ValidateTicketTokenStruct<'a> {
    type Clock = ChronoAuthClock;
    type NonceHeader = ActixWebAuthNonceHeader;
    type TokenHeader = TicketAuthTokenHeader;
    type NonceRepository = MemoryAuthNonceRepository<'a>;
    type TicketRepository = MemoryAuthTicketRepository<'a>;
    type TokenValidator = AuthJwtTokenValidator<'a>;

    fn config(&self) -> &ValidateConfig {
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

impl<'a> ValidateApiTokenStruct<'a> {
    pub fn new(
        request: HttpRequest,
        feature: &'a AuthOutsideFeature,
        require_roles: RequireAuthRoles,
    ) -> Self {
        Self {
            config: ValidateConfig { require_roles },
            nonce_config: AuthNonceConfig {
                nonce_expires: feature.config.api_expires,
            },
            clock: ChronoAuthClock::new(),
            nonce_header: ActixWebAuthNonceHeader::new(request.clone()),
            token_header: ApiAuthTokenHeader::new(request.clone()),
            nonce_repository: MemoryAuthNonceRepository::new(&feature.store.nonce),
            ticket_repository: MemoryAuthTicketRepository::new(&feature.store.ticket),
            token_validator: ApiJwtTokenValidator::with_key(&feature.secret.api.decoding_key),
        }
    }
}

impl<'a> ValidateAuthTokenInfra for ValidateApiTokenStruct<'a> {
    type Clock = ChronoAuthClock;
    type NonceHeader = ActixWebAuthNonceHeader;
    type TokenHeader = ApiAuthTokenHeader;
    type NonceRepository = MemoryAuthNonceRepository<'a>;
    type TicketRepository = MemoryAuthTicketRepository<'a>;
    type TokenValidator = ApiJwtTokenValidator<'a>;

    fn config(&self) -> &ValidateConfig {
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

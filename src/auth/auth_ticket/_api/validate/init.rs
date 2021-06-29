mod token_decoder;
mod token_header;

use actix_web::HttpRequest;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::auth_ticket::_api::kernel::init::{
    CheckAuthNonceStruct, ChronoAuthClock, MemoryAuthTicketRepository,
};
use token_decoder::{JwtApiTokenDecoder, JwtAuthTokenDecoder};
use token_header::{ApiAuthTokenHeader, TicketAuthTokenHeader};

use super::infra::{ValidateAuthTokenConfig, ValidateAuthTokenInfra};

use crate::auth::auth_user::_api::kernel::data::RequireAuthRoles;

pub struct TicketValidateAuthTokenStruct<'a> {
    check_nonce_infra: CheckAuthNonceStruct<'a>,
    config: ValidateAuthTokenConfig,
    clock: ChronoAuthClock,
    token_header: TicketAuthTokenHeader,
    ticket_repository: MemoryAuthTicketRepository<'a>,
    token_validator: JwtAuthTokenDecoder<'a>,
}

impl<'a> TicketValidateAuthTokenStruct<'a> {
    pub fn new(feature: &'a AuthOutsideFeature, request: &'a HttpRequest) -> Self {
        Self {
            check_nonce_infra: CheckAuthNonceStruct::new(feature, request),
            config: ValidateAuthTokenConfig {
                require_roles: RequireAuthRoles::Nothing, // ticket 検証では role は不問
            },
            clock: ChronoAuthClock::new(),
            token_header: TicketAuthTokenHeader::new(request.clone()),
            ticket_repository: MemoryAuthTicketRepository::new(&feature.store.ticket),
            token_validator: JwtAuthTokenDecoder::new(&feature.secret.ticket.decoding_key),
        }
    }
}

impl<'a> ValidateAuthTokenInfra for TicketValidateAuthTokenStruct<'a> {
    type CheckNonceInfra = CheckAuthNonceStruct<'a>;
    type Clock = ChronoAuthClock;
    type TokenHeader = TicketAuthTokenHeader;
    type TicketRepository = MemoryAuthTicketRepository<'a>;
    type TokenDecoder = JwtAuthTokenDecoder<'a>;

    fn check_nonce_infra(&self) -> &Self::CheckNonceInfra {
        &self.check_nonce_infra
    }
    fn config(&self) -> &ValidateAuthTokenConfig {
        &self.config
    }
    fn clock(&self) -> &Self::Clock {
        &self.clock
    }
    fn token_header(&self) -> &Self::TokenHeader {
        &self.token_header
    }
    fn ticket_repository(&self) -> &Self::TicketRepository {
        &self.ticket_repository
    }
    fn token_validator(&self) -> &Self::TokenDecoder {
        &self.token_validator
    }
}

pub struct ApiValidateAuthTokenStruct<'a> {
    check_nonce_infra: CheckAuthNonceStruct<'a>,
    config: ValidateAuthTokenConfig,
    clock: ChronoAuthClock,
    token_header: ApiAuthTokenHeader,
    ticket_repository: MemoryAuthTicketRepository<'a>,
    token_validator: JwtApiTokenDecoder<'a>,
}

impl<'a> ApiValidateAuthTokenStruct<'a> {
    pub fn new(
        feature: &'a AuthOutsideFeature,
        request: &'a HttpRequest,
        require_roles: RequireAuthRoles,
    ) -> Self {
        Self {
            check_nonce_infra: CheckAuthNonceStruct::new(feature, request),
            config: ValidateAuthTokenConfig { require_roles },
            clock: ChronoAuthClock::new(),
            token_header: ApiAuthTokenHeader::new(request.clone()),
            ticket_repository: MemoryAuthTicketRepository::new(&feature.store.ticket),
            token_validator: JwtApiTokenDecoder::new(&feature.secret.api.decoding_key),
        }
    }
}

impl<'a> ValidateAuthTokenInfra for ApiValidateAuthTokenStruct<'a> {
    type CheckNonceInfra = CheckAuthNonceStruct<'a>;
    type Clock = ChronoAuthClock;
    type TokenHeader = ApiAuthTokenHeader;
    type TicketRepository = MemoryAuthTicketRepository<'a>;
    type TokenDecoder = JwtApiTokenDecoder<'a>;

    fn check_nonce_infra(&self) -> &Self::CheckNonceInfra {
        &self.check_nonce_infra
    }
    fn config(&self) -> &ValidateAuthTokenConfig {
        &self.config
    }
    fn token_header(&self) -> &Self::TokenHeader {
        &self.token_header
    }
    fn clock(&self) -> &Self::Clock {
        &self.clock
    }
    fn ticket_repository(&self) -> &Self::TicketRepository {
        &self.ticket_repository
    }
    fn token_validator(&self) -> &Self::TokenDecoder {
        &self.token_validator
    }
}

#[cfg(test)]
pub mod test {
    pub use super::token_decoder::test::StaticAuthTokenDecoder;
    pub use super::token_header::test::StaticAuthTokenHeader;
    use crate::auth::auth_ticket::_api::kernel::init::test::{
        MemoryAuthTicketRepository, StaticCheckAuthNonceStruct, StaticChronoAuthClock,
    };

    use super::super::infra::{ValidateAuthTokenConfig, ValidateAuthTokenInfra};

    pub struct StaticValidateAuthTokenStruct<'a> {
        pub check_nonce_infra: StaticCheckAuthNonceStruct<'a>,
        pub config: ValidateAuthTokenConfig,
        pub clock: StaticChronoAuthClock,
        pub token_header: StaticAuthTokenHeader,
        pub ticket_repository: MemoryAuthTicketRepository<'a>,
        pub token_validator: StaticAuthTokenDecoder,
    }

    impl<'a> ValidateAuthTokenInfra for StaticValidateAuthTokenStruct<'a> {
        type CheckNonceInfra = StaticCheckAuthNonceStruct<'a>;
        type Clock = StaticChronoAuthClock;
        type TokenHeader = StaticAuthTokenHeader;
        type TicketRepository = MemoryAuthTicketRepository<'a>;
        type TokenDecoder = StaticAuthTokenDecoder;

        fn check_nonce_infra(&self) -> &Self::CheckNonceInfra {
            &self.check_nonce_infra
        }
        fn config(&self) -> &ValidateAuthTokenConfig {
            &self.config
        }
        fn clock(&self) -> &Self::Clock {
            &self.clock
        }
        fn token_header(&self) -> &Self::TokenHeader {
            &self.token_header
        }
        fn ticket_repository(&self) -> &Self::TicketRepository {
            &self.ticket_repository
        }
        fn token_validator(&self) -> &Self::TokenDecoder {
            &self.token_validator
        }
    }
}

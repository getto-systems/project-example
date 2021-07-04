mod clock;
mod nonce_header;
mod nonce_repository;
mod ticket_repository;

use actix_web::HttpRequest;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use clock::ChronoAuthClock;
use nonce_header::ActixWebAuthNonceHeader;
use nonce_repository::DynamoDbAuthNonceRepository;
use ticket_repository::MysqlAuthTicketRepository;

use crate::auth::auth_ticket::_api::kernel::infra::{
    AuthNonceConfig, AuthTicketInfra, CheckAuthNonceInfra,
};

pub struct AuthTicketStruct<'a> {
    clock: ChronoAuthClock,
    ticket_repository: MysqlAuthTicketRepository<'a>,
}

impl<'a> AuthTicketStruct<'a> {
    pub fn new(feature: &'a AuthOutsideFeature) -> Self {
        Self {
            clock: ChronoAuthClock::new(),
            ticket_repository: MysqlAuthTicketRepository::new(&feature.store.mysql),
        }
    }
}

impl<'a> AuthTicketInfra for AuthTicketStruct<'a> {
    type Clock = ChronoAuthClock;
    type TicketRepository = MysqlAuthTicketRepository<'a>;

    fn clock(&self) -> &Self::Clock {
        &self.clock
    }
    fn ticket_repository(&self) -> &Self::TicketRepository {
        &self.ticket_repository
    }
}

pub struct CheckAuthNonceStruct<'a> {
    config: AuthNonceConfig,
    clock: ChronoAuthClock,
    nonce_header: ActixWebAuthNonceHeader<'a>,
    nonce_repository: DynamoDbAuthNonceRepository<'a>,
}

impl<'a> CheckAuthNonceInfra for CheckAuthNonceStruct<'a> {
    type Clock = ChronoAuthClock;
    type NonceHeader = ActixWebAuthNonceHeader<'a>;
    type NonceRepository = DynamoDbAuthNonceRepository<'a>;

    fn config(&self) -> &AuthNonceConfig {
        &self.config
    }
    fn clock(&self) -> &Self::Clock {
        &self.clock
    }
    fn nonce_header(&self) -> &Self::NonceHeader {
        &self.nonce_header
    }
    fn nonce_repository(&self) -> &Self::NonceRepository {
        &self.nonce_repository
    }
}

impl<'a> CheckAuthNonceStruct<'a> {
    pub fn new(feature: &'a AuthOutsideFeature, request: &'a HttpRequest) -> Self {
        Self {
            config: AuthNonceConfig {
                nonce_expires: feature.config.ticket_expires,
            },
            clock: ChronoAuthClock::new(),
            nonce_header: ActixWebAuthNonceHeader::new(request),
            nonce_repository: DynamoDbAuthNonceRepository::new(
                &feature.store.dynamodb,
                feature.store.nonce_table_name,
            ),
        }
    }
}

#[cfg(test)]
pub mod test {
    pub use super::clock::test::StaticChronoAuthClock;
    pub use super::nonce_header::test::StaticAuthNonceHeader;
    pub use super::nonce_repository::test::{
        MemoryAuthNonceMap, MemoryAuthNonceRepository, MemoryAuthNonceStore,
    };
    pub use super::ticket_repository::test::{
        MemoryAuthTicketMap, MemoryAuthTicketStore, MemoryAuthTicketRepository,
    };

    use crate::auth::auth_ticket::_api::kernel::infra::{
        AuthNonceConfig, AuthTicketInfra, CheckAuthNonceInfra,
    };

    pub struct StaticAuthTicketStruct<'a> {
        pub clock: StaticChronoAuthClock,
        pub ticket_repository: MemoryAuthTicketRepository<'a>,
    }

    impl<'a> AuthTicketInfra for StaticAuthTicketStruct<'a> {
        type Clock = StaticChronoAuthClock;
        type TicketRepository = MemoryAuthTicketRepository<'a>;

        fn clock(&self) -> &Self::Clock {
            &self.clock
        }
        fn ticket_repository(&self) -> &Self::TicketRepository {
            &self.ticket_repository
        }
    }

    pub struct StaticCheckAuthNonceStruct<'a> {
        pub config: AuthNonceConfig,
        pub clock: StaticChronoAuthClock,
        pub nonce_header: StaticAuthNonceHeader,
        pub nonce_repository: MemoryAuthNonceRepository<'a>,
    }

    impl<'a> CheckAuthNonceInfra for StaticCheckAuthNonceStruct<'a> {
        type Clock = StaticChronoAuthClock;
        type NonceHeader = StaticAuthNonceHeader;
        type NonceRepository = MemoryAuthNonceRepository<'a>;

        fn config(&self) -> &AuthNonceConfig {
            &self.config
        }
        fn clock(&self) -> &Self::Clock {
            &self.clock
        }
        fn nonce_header(&self) -> &Self::NonceHeader {
            &self.nonce_header
        }
        fn nonce_repository(&self) -> &Self::NonceRepository {
            &self.nonce_repository
        }
    }
}

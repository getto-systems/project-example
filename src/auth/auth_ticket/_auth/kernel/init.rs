mod clock;
mod nonce_metadata;
mod nonce_repository;
mod ticket_repository;

use tonic::Request;

use crate::auth::_auth::x_outside_feature::feature::AuthOutsideFeature;

use clock::ChronoAuthClock;
use nonce_metadata::TonicAuthNonceMetadata;
use nonce_repository::DynamoDbAuthNonceRepository;
use ticket_repository::MysqlAuthTicketRepository;

use crate::auth::auth_ticket::_auth::kernel::infra::{
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

pub struct CheckAuthNonceStruct<'a, T> {
    config: AuthNonceConfig,
    clock: ChronoAuthClock,
    nonce_metadata: TonicAuthNonceMetadata<'a, T>,
    nonce_repository: DynamoDbAuthNonceRepository<'a>,
}

impl<'a, T> CheckAuthNonceInfra for CheckAuthNonceStruct<'a, T> {
    type Clock = ChronoAuthClock;
    type NonceMetadata = TonicAuthNonceMetadata<'a, T>;
    type NonceRepository = DynamoDbAuthNonceRepository<'a>;

    fn config(&self) -> &AuthNonceConfig {
        &self.config
    }
    fn clock(&self) -> &Self::Clock {
        &self.clock
    }
    fn nonce_metadata(&self) -> &Self::NonceMetadata {
        &self.nonce_metadata
    }
    fn nonce_repository(&self) -> &Self::NonceRepository {
        &self.nonce_repository
    }
}

impl<'a, T> CheckAuthNonceStruct<'a, T> {
    pub fn new(feature: &'a AuthOutsideFeature, request: &'a Request<T>) -> Self {
        Self {
            config: AuthNonceConfig {
                nonce_expires: feature.config.ticket_expires,
            },
            clock: ChronoAuthClock::new(),
            nonce_metadata: TonicAuthNonceMetadata::new(request),
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
    pub use super::nonce_metadata::test::StaticAuthNonceMetadata;
    pub use super::nonce_repository::test::{
        MemoryAuthNonceMap, MemoryAuthNonceRepository, MemoryAuthNonceStore,
    };
    pub use super::ticket_repository::test::{
        MemoryAuthTicketMap, MemoryAuthTicketStore, MemoryAuthTicketRepository,
    };

    use crate::auth::auth_ticket::_auth::kernel::infra::{
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
        pub nonce_metadata: StaticAuthNonceMetadata,
        pub nonce_repository: MemoryAuthNonceRepository<'a>,
    }

    impl<'a> CheckAuthNonceInfra for StaticCheckAuthNonceStruct<'a> {
        type Clock = StaticChronoAuthClock;
        type NonceMetadata = StaticAuthNonceMetadata;
        type NonceRepository = MemoryAuthNonceRepository<'a>;

        fn config(&self) -> &AuthNonceConfig {
            &self.config
        }
        fn clock(&self) -> &Self::Clock {
            &self.clock
        }
        fn nonce_metadata(&self) -> &Self::NonceMetadata {
            &self.nonce_metadata
        }
        fn nonce_repository(&self) -> &Self::NonceRepository {
            &self.nonce_repository
        }
    }
}

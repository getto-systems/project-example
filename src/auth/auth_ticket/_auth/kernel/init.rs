mod clock;
mod nonce_metadata;
mod nonce_repository;
mod ticket_repository;

use tonic::metadata::MetadataMap;

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

    fn extract(self) -> (Self::Clock, Self::TicketRepository) {
        (self.clock, self.ticket_repository)
    }
}

pub struct CheckAuthNonceStruct<'a> {
    config: AuthNonceConfig,
    clock: ChronoAuthClock,
    nonce_metadata: TonicAuthNonceMetadata,
    nonce_repository: DynamoDbAuthNonceRepository<'a>,
}

impl<'a> CheckAuthNonceInfra for CheckAuthNonceStruct<'a> {
    type Clock = ChronoAuthClock;
    type NonceMetadata = TonicAuthNonceMetadata;
    type NonceRepository = DynamoDbAuthNonceRepository<'a>;

    fn extract(
        self,
    ) -> (
        Self::Clock,
        Self::NonceMetadata,
        Self::NonceRepository,
        AuthNonceConfig,
    ) {
        (
            self.clock,
            self.nonce_metadata,
            self.nonce_repository,
            self.config,
        )
    }
}

impl<'a> CheckAuthNonceStruct<'a> {
    pub fn new(feature: &'a AuthOutsideFeature, metadata: MetadataMap) -> Self {
        Self {
            config: AuthNonceConfig {
                nonce_expires: feature.config.ticket_expires,
            },
            clock: ChronoAuthClock::new(),
            nonce_metadata: TonicAuthNonceMetadata::new(metadata),
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
        MemoryAuthTicketMap, MemoryAuthTicketRepository, MemoryAuthTicketStore,
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

        fn extract(self) -> (Self::Clock, Self::TicketRepository) {
            (self.clock, self.ticket_repository)
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

        fn extract(
            self,
        ) -> (
            Self::Clock,
            Self::NonceMetadata,
            Self::NonceRepository,
            AuthNonceConfig,
        ) {
            (
                self.clock,
                self.nonce_metadata,
                self.nonce_repository,
                self.config,
            )
        }
    }
}

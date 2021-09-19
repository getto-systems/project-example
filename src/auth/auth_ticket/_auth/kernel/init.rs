pub(in crate::auth) mod clock;
pub(in crate::auth) mod nonce_metadata;
pub(in crate::auth) mod nonce_repository;
pub(in crate::auth) mod ticket_repository;

use tonic::metadata::MetadataMap;

use crate::auth::_auth::x_outside_feature::feature::AuthOutsideFeature;

use clock::ChronoAuthClock;
use nonce_metadata::TonicAuthNonceMetadata;
use nonce_repository::DynamoDbAuthNonceRepository;

use crate::auth::auth_ticket::_auth::kernel::infra::{AuthNonceConfig, CheckAuthNonceInfra};

pub struct CheckAuthNonceStruct<'a> {
    config: AuthNonceConfig,
    clock: ChronoAuthClock,
    nonce_metadata: TonicAuthNonceMetadata<'a>,
    nonce_repository: DynamoDbAuthNonceRepository<'a>,
}

impl<'a> CheckAuthNonceInfra for CheckAuthNonceStruct<'a> {
    type Clock = ChronoAuthClock;
    type NonceMetadata = TonicAuthNonceMetadata<'a>;
    type NonceRepository = DynamoDbAuthNonceRepository<'a>;

    fn clock(&self) -> &Self::Clock {
        &self.clock
    }
    fn nonce_metadata(&self) -> &Self::NonceMetadata {
        &self.nonce_metadata
    }
    fn nonce_repository(&self) -> &Self::NonceRepository {
        &self.nonce_repository
    }
    fn config(&self) -> &AuthNonceConfig {
        &self.config
    }
}

impl<'a> CheckAuthNonceStruct<'a> {
    pub fn new(feature: &'a AuthOutsideFeature, metadata: &'a MetadataMap) -> Self {
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
    use super::clock::test::StaticChronoAuthClock;
    use super::nonce_repository::test::MemoryAuthNonceRepository;
    use crate::auth::auth_ticket::_common::kernel::init::nonce_metadata::test::StaticAuthNonceMetadata;

    use crate::auth::auth_ticket::_auth::kernel::infra::{AuthNonceConfig, CheckAuthNonceInfra};

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

        fn clock(&self) -> &Self::Clock {
            &self.clock
        }
        fn nonce_metadata(&self) -> &Self::NonceMetadata {
            &self.nonce_metadata
        }
        fn nonce_repository(&self) -> &Self::NonceRepository {
            &self.nonce_repository
        }
        fn config(&self) -> &AuthNonceConfig {
            &self.config
        }
    }
}

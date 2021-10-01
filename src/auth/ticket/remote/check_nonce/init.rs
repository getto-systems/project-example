pub mod nonce_repository;

use tonic::metadata::MetadataMap;

use crate::auth::remote::x_outside_feature::auth::feature::AuthOutsideFeature;

use crate::auth::ticket::remote::kernel::init::{
    clock::ChronoAuthClock, nonce_metadata::TonicAuthNonceMetadata,
};
use nonce_repository::DynamoDbAuthNonceRepository;

use crate::auth::ticket::remote::check_nonce::infra::{AuthNonceConfig, CheckAuthNonceInfra};

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
    use super::nonce_repository::test::MemoryAuthNonceRepository;
    use crate::auth::ticket::remote::kernel::init::{
        clock::test::StaticChronoAuthClock, nonce_metadata::test::StaticAuthNonceMetadata,
    };

    use crate::auth::ticket::remote::check_nonce::infra::{
        AuthNonceConfig, CheckAuthNonceInfra,
    };

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
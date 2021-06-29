mod clock;
mod nonce_header;
mod nonce_repository;
mod ticket_repository;

use actix_web::HttpRequest;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

pub use clock::ChronoAuthClock;
pub use nonce_header::ActixWebAuthNonceHeader;
pub use nonce_repository::{MemoryAuthNonceMap, MemoryAuthNonceRepository, MemoryAuthNonceStore};
pub use ticket_repository::{
    MemoryAuthTicketMap, MemoryAuthTicketRepository, MemoryAuthTicketStore,
};

use super::infra::{AuthNonceConfig, CheckAuthNonceInfra};

pub struct CheckAuthNonceStruct<'a> {
    config: AuthNonceConfig,
    clock: ChronoAuthClock,
    nonce_header: ActixWebAuthNonceHeader<'a>,
    nonce_repository: MemoryAuthNonceRepository<'a>,
}

impl<'a> CheckAuthNonceInfra for CheckAuthNonceStruct<'a> {
    type Clock = ChronoAuthClock;
    type NonceHeader = ActixWebAuthNonceHeader<'a>;
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

impl<'a> CheckAuthNonceStruct<'a> {
    pub fn new(feature: &'a AuthOutsideFeature, request: &'a HttpRequest) -> Self {
        Self {
            config: AuthNonceConfig {
                nonce_expires: feature.config.ticket_expires,
            },
            clock: ChronoAuthClock::new(),
            nonce_header: ActixWebAuthNonceHeader::new(request),
            nonce_repository: MemoryAuthNonceRepository::new(&feature.store.nonce),
        }
    }
}

#[cfg(test)]
pub mod test {
    pub use super::clock::test::StaticChronoAuthClock;
    pub use super::nonce_header::test::StaticAuthNonceHeader;
    pub use super::nonce_repository::{
        MemoryAuthNonceMap, MemoryAuthNonceRepository, MemoryAuthNonceStore,
    };
    pub use super::ticket_repository::{
        MemoryAuthTicketMap, MemoryAuthTicketRepository, MemoryAuthTicketStore,
    };

    use super::super::infra::{AuthNonceConfig, CheckAuthNonceInfra};

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

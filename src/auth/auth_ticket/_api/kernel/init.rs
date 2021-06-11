use actix_web::HttpRequest;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use super::infra::{
    clock::ChronoAuthClock, nonce_header::ActixWebAuthNonceHeader,
    nonce_repository::MemoryAuthNonceRepository, AuthNonceConfig, CheckAuthNonceInfra,
};

pub struct CheckAuthNonceStruct<'a> {
    config: AuthNonceConfig,
    clock: ChronoAuthClock,
    nonce_header: ActixWebAuthNonceHeader,
    nonce_repository: MemoryAuthNonceRepository<'a>,
}

impl<'a> CheckAuthNonceInfra for CheckAuthNonceStruct<'a> {
    type Clock = ChronoAuthClock;
    type NonceHeader = ActixWebAuthNonceHeader;
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
    pub fn new(request: HttpRequest, feature: &'a AuthOutsideFeature) -> Self {
        Self {
            config: AuthNonceConfig {
                nonce_expires: feature.config.ticket_expires,
            },
            clock: ChronoAuthClock::new(),
            nonce_header: ActixWebAuthNonceHeader::new(request.clone()),
            nonce_repository: MemoryAuthNonceRepository::new(&feature.store.nonce),
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::super::infra::{
        clock::test::StaticChronoAuthClock, nonce_header::test::StaticAuthNonceHeader,
        nonce_repository::MemoryAuthNonceRepository, AuthNonceConfig, CheckAuthNonceInfra,
    };

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

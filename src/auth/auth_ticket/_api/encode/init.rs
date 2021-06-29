mod messenger;
mod token_encoder;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use messenger::{
    AuthenticatePasswordEncodeMessenger, RenewEncodeMessenger, ResetPasswordEncodeMessenger,
};
use token_encoder::{ApiJwtAuthTokenEncoder, CloudfrontTokenEncoder, TicketJwtAuthTokenEncoder};

use super::super::kernel::infra::{
    clock::ChronoAuthClock, ticket_repository::MemoryAuthTicketRepository,
};
use super::infra::{EncodeAuthTicketConfig, EncodeAuthTicketInfra, EncodeMessenger};

pub struct EncodeAuthTicketStruct<'a, M: EncodeMessenger> {
    config: EncodeAuthTicketConfig,
    clock: ChronoAuthClock,
    ticket_repository: MemoryAuthTicketRepository<'a>,
    ticket_encoder: TicketJwtAuthTokenEncoder<'a>,
    api_encoder: ApiJwtAuthTokenEncoder<'a>,
    cdn_encoder: CloudfrontTokenEncoder<'a>,
    messenger: M,
}
pub type RenewEncodeAuthTicketStruct<'a> = EncodeAuthTicketStruct<'a, RenewEncodeMessenger>;
pub type AuthenticatePasswordEncodeAuthTicketStruct<'a> =
    EncodeAuthTicketStruct<'a, AuthenticatePasswordEncodeMessenger>;
pub type ResetPasswordEncodeAuthTicketStruct<'a> =
    EncodeAuthTicketStruct<'a, ResetPasswordEncodeMessenger>;

impl<'a, M: EncodeMessenger> EncodeAuthTicketStruct<'a, M> {
    fn with_messenger(feature: &'a AuthOutsideFeature, messenger: M) -> Self {
        Self {
            config: EncodeAuthTicketConfig {
                ticket_expires: feature.config.ticket_expires,
                api_expires: feature.config.api_expires,
                cdn_expires: feature.config.cdn_expires,
            },
            clock: ChronoAuthClock::new(),
            ticket_repository: MemoryAuthTicketRepository::new(&feature.store.ticket),
            ticket_encoder: TicketJwtAuthTokenEncoder::new(
                &feature.cookie,
                &feature.secret.ticket.encoding_key,
            ),
            api_encoder: ApiJwtAuthTokenEncoder::new(
                &feature.cookie,
                &feature.secret.api.encoding_key,
            ),
            cdn_encoder: CloudfrontTokenEncoder::new(&feature.secret.cdn, &feature.cookie),
            messenger,
        }
    }
}
impl<'a> RenewEncodeAuthTicketStruct<'a> {
    pub fn new(feature: &'a AuthOutsideFeature) -> Self {
        Self::with_messenger(feature, RenewEncodeMessenger::new())
    }
}
impl<'a> AuthenticatePasswordEncodeAuthTicketStruct<'a> {
    pub fn new(feature: &'a AuthOutsideFeature) -> Self {
        Self::with_messenger(feature, AuthenticatePasswordEncodeMessenger::new())
    }
}
impl<'a> ResetPasswordEncodeAuthTicketStruct<'a> {
    pub fn new(feature: &'a AuthOutsideFeature) -> Self {
        Self::with_messenger(feature, ResetPasswordEncodeMessenger::new())
    }
}

impl<'a, M: EncodeMessenger> EncodeAuthTicketInfra for EncodeAuthTicketStruct<'a, M> {
    type Clock = ChronoAuthClock;
    type TicketRepository = MemoryAuthTicketRepository<'a>;
    type TicketEncoder = TicketJwtAuthTokenEncoder<'a>;
    type ApiEncoder = ApiJwtAuthTokenEncoder<'a>;
    type CdnEncoder = CloudfrontTokenEncoder<'a>;
    type Messenger = M;

    fn config(&self) -> &EncodeAuthTicketConfig {
        &self.config
    }
    fn clock(&self) -> &Self::Clock {
        &self.clock
    }
    fn ticket_repository(&self) -> &Self::TicketRepository {
        &self.ticket_repository
    }
    fn ticket_encoder(&self) -> &Self::TicketEncoder {
        &self.ticket_encoder
    }
    fn api_encoder(&self) -> &Self::ApiEncoder {
        &self.api_encoder
    }
    fn cdn_encoder(&self) -> &Self::CdnEncoder {
        &self.cdn_encoder
    }
    fn messenger(&self) -> &Self::Messenger {
        &self.messenger
    }
}

#[cfg(test)]
pub mod test {
    pub use super::messenger::test::StaticEncodeMessenger;
    pub use super::token_encoder::test::StaticAuthTokenEncoder;

    use crate::auth::auth_ticket::_api::{
        encode::infra::{EncodeAuthTicketConfig, EncodeAuthTicketInfra},
        kernel::infra::{
            clock::test::StaticChronoAuthClock, ticket_repository::MemoryAuthTicketRepository,
        },
    };

    pub struct StaticEncodeAuthTicketStruct<'a> {
        pub config: EncodeAuthTicketConfig,
        pub clock: StaticChronoAuthClock,
        pub ticket_repository: MemoryAuthTicketRepository<'a>,
        pub ticket_encoder: StaticAuthTokenEncoder,
        pub api_encoder: StaticAuthTokenEncoder,
        pub cdn_encoder: StaticAuthTokenEncoder,
        pub messenger: StaticEncodeMessenger,
    }

    impl<'a> EncodeAuthTicketInfra for StaticEncodeAuthTicketStruct<'a> {
        type Clock = StaticChronoAuthClock;
        type TicketRepository = MemoryAuthTicketRepository<'a>;
        type TicketEncoder = StaticAuthTokenEncoder;
        type ApiEncoder = StaticAuthTokenEncoder;
        type CdnEncoder = StaticAuthTokenEncoder;
        type Messenger = StaticEncodeMessenger;

        fn config(&self) -> &EncodeAuthTicketConfig {
            &self.config
        }
        fn clock(&self) -> &Self::Clock {
            &self.clock
        }
        fn ticket_repository(&self) -> &Self::TicketRepository {
            &self.ticket_repository
        }
        fn ticket_encoder(&self) -> &Self::TicketEncoder {
            &self.ticket_encoder
        }
        fn api_encoder(&self) -> &Self::ApiEncoder {
            &self.api_encoder
        }
        fn cdn_encoder(&self) -> &Self::CdnEncoder {
            &self.cdn_encoder
        }
        fn messenger(&self) -> &Self::Messenger {
            &self.messenger
        }
    }
}

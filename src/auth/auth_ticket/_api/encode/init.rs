use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use super::super::kernel::infra::{
    clock::ChronoAuthClock, ticket_repository::MemoryAuthTicketRepository,
};
use super::infra::{
    messenger::{EncodeAuthenticatePasswordMessenger, EncodeRenewMessenger},
    token_encoder::{ApiJwtTokenEncoder, CloudfrontTokenEncoder, TicketJwtTokenEncoder},
    EncodeAuthTicketConfig, EncodeAuthTicketInfra, EncodeMessenger,
};

pub struct EncodeAuthTicketStruct<'a, M: EncodeMessenger> {
    config: EncodeAuthTicketConfig,
    clock: ChronoAuthClock,
    ticket_repository: MemoryAuthTicketRepository<'a>,
    ticket_encoder: TicketJwtTokenEncoder<'a>,
    api_encoder: ApiJwtTokenEncoder<'a>,
    cdn_encoder: CloudfrontTokenEncoder<'a>,
    messenger: M,
}
pub type EncodeRenewAuthTicketStruct<'a> = EncodeAuthTicketStruct<'a, EncodeRenewMessenger>;
pub type EncodeAuthenticatePasswordAuthTicketStruct<'a> =
    EncodeAuthTicketStruct<'a, EncodeAuthenticatePasswordMessenger>;

impl<'a, M: EncodeMessenger> EncodeAuthTicketStruct<'a, M> {
    pub fn with_messenger(feature: &'a AuthOutsideFeature, messenger: M) -> Self {
        Self {
            config: EncodeAuthTicketConfig {
                ticket_expires: feature.config.ticket_expires,
                api_expires: feature.config.api_expires,
                cdn_expires: feature.config.cdn_expires,
            },
            clock: ChronoAuthClock::new(),
            ticket_repository: MemoryAuthTicketRepository::new(&feature.store.ticket),
            ticket_encoder: TicketJwtTokenEncoder::new(
                &feature.cookie,
                &feature.secret.ticket.encoding_key,
            ),
            api_encoder: ApiJwtTokenEncoder::new(&feature.cookie, &feature.secret.api.encoding_key),
            cdn_encoder: CloudfrontTokenEncoder::new(&feature.secret.cdn, &feature.cookie),
            messenger,
        }
    }
}
impl<'a> EncodeRenewAuthTicketStruct<'a> {
    pub fn new(feature: &'a AuthOutsideFeature) -> Self {
        Self::with_messenger(feature, EncodeRenewMessenger::new())
    }
}
impl<'a> EncodeAuthenticatePasswordAuthTicketStruct<'a> {
    pub fn new(feature: &'a AuthOutsideFeature) -> Self {
        Self::with_messenger(feature, EncodeAuthenticatePasswordMessenger::new())
    }
}

impl<'a, M: EncodeMessenger> EncodeAuthTicketInfra for EncodeAuthTicketStruct<'a, M> {
    type Clock = ChronoAuthClock;
    type TicketRepository = MemoryAuthTicketRepository<'a>;
    type TicketEncoder = TicketJwtTokenEncoder<'a>;
    type ApiEncoder = ApiJwtTokenEncoder<'a>;
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

pub mod token_encoder;

use crate::auth::remote::x_outside_feature::auth::feature::AuthOutsideFeature;

use crate::auth::ticket::remote::kernel::init::{
    clock::ChronoAuthClock, ticket_repository::MysqlAuthTicketRepository,
};
use token_encoder::{
    ApiJwtAuthTokenEncoder, CookieCloudfrontTokenEncoder, TicketJwtAuthTokenEncoder,
};

use super::method::{EncodeAuthTicketConfig, EncodeAuthTicketInfra};

pub struct EncodeAuthTicketStruct<'a> {
    clock: ChronoAuthClock,
    ticket_repository: MysqlAuthTicketRepository<'a>,
    ticket_encoder: TicketJwtAuthTokenEncoder<'a>,
    api_encoder: ApiJwtAuthTokenEncoder<'a>,
    cloudfront_encoder: CookieCloudfrontTokenEncoder<'a>,
    config: EncodeAuthTicketConfig,
}

impl<'a> EncodeAuthTicketStruct<'a> {
    pub fn new(feature: &'a AuthOutsideFeature) -> Self {
        Self {
            clock: ChronoAuthClock::new(),
            ticket_repository: MysqlAuthTicketRepository::new(&feature.store.mysql),
            ticket_encoder: TicketJwtAuthTokenEncoder::new(&feature.encoding_key.ticket),
            api_encoder: ApiJwtAuthTokenEncoder::new(&feature.encoding_key.api),
            cloudfront_encoder: CookieCloudfrontTokenEncoder::new(&feature.cloudfront_key),
            config: EncodeAuthTicketConfig {
                ticket_expires: feature.config.ticket_expires,
                api_expires: feature.config.api_expires,
                cloudfront_expires: feature.config.cloudfront_expires,
            },
        }
    }
}

impl<'a> EncodeAuthTicketInfra for EncodeAuthTicketStruct<'a> {
    type Clock = ChronoAuthClock;
    type TicketRepository = MysqlAuthTicketRepository<'a>;
    type TicketEncoder = TicketJwtAuthTokenEncoder<'a>;
    type ApiEncoder = ApiJwtAuthTokenEncoder<'a>;
    type CloudfrontEncoder = CookieCloudfrontTokenEncoder<'a>;

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
    fn cloudfront_encoder(&self) -> &Self::CloudfrontEncoder {
        &self.cloudfront_encoder
    }
    fn config(&self) -> &EncodeAuthTicketConfig {
        &self.config
    }
}

#[cfg(test)]
pub mod test {
    use super::token_encoder::test::{StaticAuthTokenEncoder, StaticCloudfrontTokenEncoder};
    use crate::auth::ticket::remote::kernel::init::{
        clock::test::StaticChronoAuthClock, ticket_repository::test::MemoryAuthTicketRepository,
    };

    use super::super::method::{EncodeAuthTicketConfig, EncodeAuthTicketInfra};

    pub struct StaticEncodeAuthTicketStruct<'a> {
        pub clock: StaticChronoAuthClock,
        pub ticket_repository: MemoryAuthTicketRepository<'a>,
        pub ticket_encoder: StaticAuthTokenEncoder,
        pub api_encoder: StaticAuthTokenEncoder,
        pub cloudfront_encoder: StaticCloudfrontTokenEncoder,
        pub config: EncodeAuthTicketConfig,
    }

    impl<'a> EncodeAuthTicketInfra for StaticEncodeAuthTicketStruct<'a> {
        type Clock = StaticChronoAuthClock;
        type TicketRepository = MemoryAuthTicketRepository<'a>;
        type TicketEncoder = StaticAuthTokenEncoder;
        type ApiEncoder = StaticAuthTokenEncoder;
        type CloudfrontEncoder = StaticCloudfrontTokenEncoder;

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
        fn cloudfront_encoder(&self) -> &Self::CloudfrontEncoder {
            &self.cloudfront_encoder
        }
        fn config(&self) -> &EncodeAuthTicketConfig {
            &self.config
        }
    }
}

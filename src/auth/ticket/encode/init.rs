use crate::x_outside_feature::auth::feature::AuthAppFeature;

use crate::auth::{
    kernel::init::clock::ChronoAuthClock,
    ticket::kernel::init::{
        ticket_repository::dynamodb::DynamoDbAuthTicketRepository,
        token::{
            authenticate::encoder::JwtAuthenticateTokenEncoder,
            authorize::encoder::JwtAuthorizeTokenEncoder,
            cdn::encoder::AWSCloudfrontCdnTokenEncoder,
        },
    },
};

use crate::auth::ticket::encode::method::EncodeAuthTokenInfra;

use crate::auth::ticket::encode::infra::EncodeAuthTokenConfig;

pub struct ActiveEncodeAuthTokenInfra<'a> {
    clock: ChronoAuthClock,
    ticket_repository: DynamoDbAuthTicketRepository<'a>,
    authenticate_encoder: JwtAuthenticateTokenEncoder<'a>,
    authorize_encoder: JwtAuthorizeTokenEncoder<'a>,
    cdn_encoder: AWSCloudfrontCdnTokenEncoder<'a>,
    config: EncodeAuthTokenConfig,
}

impl<'a> ActiveEncodeAuthTokenInfra<'a> {
    pub fn new(feature: &'a AuthAppFeature) -> Self {
        Self {
            clock: ChronoAuthClock::new(),
            ticket_repository: DynamoDbAuthTicketRepository::new(&feature.store),
            authenticate_encoder: JwtAuthenticateTokenEncoder::new(&feature.encoding_key),
            authorize_encoder: JwtAuthorizeTokenEncoder::new(&feature.encoding_key),
            cdn_encoder: AWSCloudfrontCdnTokenEncoder::new(&feature.cloudfront_key),
            config: EncodeAuthTokenConfig {
                authenticate_expires: feature.config.authenticate_expires,
                authorize_expires: feature.config.authorize_expires,
                cdn_expires: feature.config.cdn_expires,
            },
        }
    }
}

impl<'a> EncodeAuthTokenInfra for ActiveEncodeAuthTokenInfra<'a> {
    type Clock = ChronoAuthClock;
    type TicketRepository = DynamoDbAuthTicketRepository<'a>;
    type AuthenticateEncoder = JwtAuthenticateTokenEncoder<'a>;
    type AuthorizeEncoder = JwtAuthorizeTokenEncoder<'a>;
    type CdnEncoder = AWSCloudfrontCdnTokenEncoder<'a>;

    fn clock(&self) -> &Self::Clock {
        &self.clock
    }
    fn ticket_repository(&self) -> &Self::TicketRepository {
        &self.ticket_repository
    }
    fn authenticate_encoder(&self) -> &Self::AuthenticateEncoder {
        &self.authenticate_encoder
    }
    fn authorize_encoder(&self) -> &Self::AuthorizeEncoder {
        &self.authorize_encoder
    }
    fn cdn_encoder(&self) -> &Self::CdnEncoder {
        &self.cdn_encoder
    }
    fn config(&self) -> &EncodeAuthTokenConfig {
        &self.config
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::{
        kernel::init::clock::test::StaticChronoAuthClock,
        ticket::kernel::init::{
            ticket_repository::memory::MemoryAuthTicketRepository,
            token::{
                authenticate::encoder::test::StaticAuthenticateTokenEncoder,
                authorize::encoder::test::StaticAuthorizeTokenEncoder,
                cdn::encoder::test::StaticCdnTokenEncoder,
            },
        },
    };

    use crate::auth::ticket::encode::method::EncodeAuthTokenInfra;

    use crate::auth::ticket::encode::infra::EncodeAuthTokenConfig;

    pub struct StaticEncodeAuthTokenInfra<'a> {
        clock: StaticChronoAuthClock,
        ticket_repository: MemoryAuthTicketRepository<'a>,
        authenticate_encoder: StaticAuthenticateTokenEncoder,
        authorize_encoder: StaticAuthorizeTokenEncoder,
        cdn_encoder: StaticCdnTokenEncoder,
        config: EncodeAuthTokenConfig,
    }

    impl<'a> StaticEncodeAuthTokenInfra<'a> {
        pub fn standard(
            clock: StaticChronoAuthClock,
            ticket_repository: MemoryAuthTicketRepository<'a>,
            config: EncodeAuthTokenConfig,
        ) -> Self {
            Self {
                clock,
                ticket_repository,
                authenticate_encoder: StaticAuthenticateTokenEncoder,
                authorize_encoder: StaticAuthorizeTokenEncoder,
                cdn_encoder: StaticCdnTokenEncoder,
                config,
            }
        }
    }

    impl<'a> EncodeAuthTokenInfra for StaticEncodeAuthTokenInfra<'a> {
        type Clock = StaticChronoAuthClock;
        type TicketRepository = MemoryAuthTicketRepository<'a>;
        type AuthenticateEncoder = StaticAuthenticateTokenEncoder;
        type AuthorizeEncoder = StaticAuthorizeTokenEncoder;
        type CdnEncoder = StaticCdnTokenEncoder;

        fn clock(&self) -> &Self::Clock {
            &self.clock
        }
        fn ticket_repository(&self) -> &Self::TicketRepository {
            &self.ticket_repository
        }
        fn authenticate_encoder(&self) -> &Self::AuthenticateEncoder {
            &self.authenticate_encoder
        }
        fn authorize_encoder(&self) -> &Self::AuthorizeEncoder {
            &self.authorize_encoder
        }
        fn cdn_encoder(&self) -> &Self::CdnEncoder {
            &self.cdn_encoder
        }
        fn config(&self) -> &EncodeAuthTokenConfig {
            &self.config
        }
    }
}

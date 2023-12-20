use std::sync::Arc;

use crate::{
    auth::{
        kernel::detail::ChronoAuthClock,
        ticket::kernel::detail::{
            repository::dynamodb::ticket::{ConnectionTicket, TableTicket},
            token::{
                authenticate::encoder::JwtAuthenticateTokenEncoder,
                authorize::encoder::JwtAuthorizeTokenEncoder,
                cdn::encoder::AWSCloudfrontCdnTokenEncoder,
            },
        },
    },
    common::api::{feature::AsInfra, logger::detail::StdoutJsonLogger},
    x_outside_feature::auth::feature::AuthAppFeature,
};

use crate::auth::ticket::encode::infra::{
    EncodeAuthTokenConfig, EncodeAuthTokenInfra, EncodeAuthTokenLogger, EncodeAuthTokenRepository,
};

use crate::{
    auth::{
        kernel::data::ExpansionLimitDateTime,
        ticket::{
            encode::data::{
                AuthTokenExpires, EncodeAuthTokenError, EncodeAuthTokenSuccess, EncodeTokenError,
            },
            kernel::data::AuthTicket,
        },
    },
    common::api::repository::data::RepositoryError,
};

pub struct LiveEncodeAuthTokenInfra {
    clock: ChronoAuthClock,
    repository: LiveEncodeAuthTokenRepository,
    authenticate_encoder: JwtAuthenticateTokenEncoder,
    authorize_encoder: JwtAuthorizeTokenEncoder,
    cdn_encoder: AWSCloudfrontCdnTokenEncoder,
    config: EncodeAuthTokenConfig,
}

impl AsInfra<LiveEncodeAuthTokenInfra> for Arc<AuthAppFeature> {
    fn as_infra(&self) -> LiveEncodeAuthTokenInfra {
        LiveEncodeAuthTokenInfra {
            clock: ChronoAuthClock,
            repository: LiveEncodeAuthTokenRepository {
                conn: self.as_infra(),
            },
            authenticate_encoder: self.as_infra(),
            authorize_encoder: self.as_infra(),
            cdn_encoder: self.as_infra(),
            config: EncodeAuthTokenConfig {
                authenticate_expires: self.config.authenticate_expires,
                authorize_expires: self.config.authorize_expires,
                cdn_expires: self.config.cdn_expires,
            },
        }
    }
}

impl EncodeAuthTokenInfra for LiveEncodeAuthTokenInfra {
    type Clock = ChronoAuthClock;
    type Repository = LiveEncodeAuthTokenRepository;
    type AuthenticateEncoder = JwtAuthenticateTokenEncoder;
    type AuthorizeEncoder = JwtAuthorizeTokenEncoder;
    type CdnEncoder = AWSCloudfrontCdnTokenEncoder;

    fn clock(&self) -> &Self::Clock {
        &self.clock
    }
    fn repository(&self) -> &Self::Repository {
        &self.repository
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

pub struct LiveEncodeAuthTokenRepository {
    conn: ConnectionTicket,
}

#[async_trait::async_trait]
impl EncodeAuthTokenRepository for LiveEncodeAuthTokenRepository {
    async fn lookup_expansion_limit(
        &self,
        ticket: &AuthTicket,
    ) -> Result<Option<ExpansionLimitDateTime>, RepositoryError> {
        TableTicket::get_expansion_limit(&self.conn, ticket.clone()).await
    }
}

impl EncodeAuthTokenLogger for StdoutJsonLogger {
    fn try_to_encode_auth_token(&self) {
        self.info(format!("try to encode auth-token"));
    }
    fn calculate_token_expires(&self, expires: AuthTokenExpires) -> AuthTokenExpires {
        self.info(format!("token expires calculated; {}", expires));
        expires
    }
    fn failed_to_lookup_expansion_limit(&self, err: RepositoryError) -> RepositoryError {
        self.fatal(format!("failed to lookup expansion-limit; {}", err));
        err
    }
    fn expansion_limit_not_found(&self, err: EncodeAuthTokenError) -> EncodeAuthTokenError {
        self.fatal(format!("expansion-limit not found; {}", err));
        err
    }
    fn failed_to_encode_token(&self, err: EncodeTokenError) -> EncodeTokenError {
        self.fatal(format!("failed to encode-token; {}", err));
        err
    }
    fn succeed_to_encode_auth_token(&self, auth: EncodeAuthTokenSuccess) -> EncodeAuthTokenSuccess {
        self.info(format!("succeed to encode auth-token; {}", auth));
        auth
    }
}

#[cfg(test)]
pub mod test {
    use std::sync::Arc;

    use crate::{
        auth::{
            kernel::detail::test::MockChronoAuthClock,
            ticket::kernel::detail::{
                repository::memory::{ticket::MapTicket, StoreTicket},
                token::{
                    authenticate::encoder::test::StaticAuthenticateTokenEncoder,
                    authorize::encoder::test::StaticAuthorizeTokenEncoder,
                    cdn::encoder::test::StaticCdnTokenEncoder,
                },
            },
        },
        common::api::feature::AsInfra,
    };

    use crate::auth::ticket::encode::infra::{
        EncodeAuthTokenConfig, EncodeAuthTokenInfra, EncodeAuthTokenRepository,
    };

    use crate::{
        auth::{kernel::data::ExpansionLimitDateTime, ticket::kernel::data::AuthTicket},
        common::api::repository::data::RepositoryError,
    };

    pub struct MockEncodeAuthTokenInfra {
        clock: MockChronoAuthClock,
        repository: MockEncodeAuthTokenRepository,
        authenticate_encoder: StaticAuthenticateTokenEncoder,
        authorize_encoder: StaticAuthorizeTokenEncoder,
        cdn_encoder: StaticCdnTokenEncoder,
        config: EncodeAuthTokenConfig,
    }

    impl AsInfra<MockEncodeAuthTokenInfra>
        for (MockChronoAuthClock, Arc<StoreTicket>, EncodeAuthTokenConfig)
    {
        fn as_infra(&self) -> MockEncodeAuthTokenInfra {
            MockEncodeAuthTokenInfra {
                clock: self.0.clone(),
                repository: MockEncodeAuthTokenRepository {
                    ticket: Arc::clone(&self.1),
                },
                authenticate_encoder: StaticAuthenticateTokenEncoder,
                authorize_encoder: StaticAuthorizeTokenEncoder,
                cdn_encoder: StaticCdnTokenEncoder,
                config: self.2.clone(),
            }
        }
    }

    impl EncodeAuthTokenInfra for MockEncodeAuthTokenInfra {
        type Clock = MockChronoAuthClock;
        type Repository = MockEncodeAuthTokenRepository;
        type AuthenticateEncoder = StaticAuthenticateTokenEncoder;
        type AuthorizeEncoder = StaticAuthorizeTokenEncoder;
        type CdnEncoder = StaticCdnTokenEncoder;

        fn clock(&self) -> &Self::Clock {
            &self.clock
        }
        fn repository(&self) -> &Self::Repository {
            &self.repository
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

    pub struct MockEncodeAuthTokenRepository {
        ticket: Arc<StoreTicket>,
    }

    #[async_trait::async_trait]
    impl EncodeAuthTokenRepository for MockEncodeAuthTokenRepository {
        async fn lookup_expansion_limit(
            &self,
            ticket: &AuthTicket,
        ) -> Result<Option<ExpansionLimitDateTime>, RepositoryError> {
            Ok(MapTicket::get_expansion_limit(
                &self.ticket,
                &ticket.ticket_id,
            ))
        }
    }
}

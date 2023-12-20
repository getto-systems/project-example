use std::sync::Arc;

use actix_web::web::Data;

use crate::x_outside_feature::{
    auth::feature::AuthAppFeature, core::feature::CoreAppFeature, proxy::feature::ProxyAppFeature,
};

use crate::{
    auth::{
        kernel::detail::ChronoAuthClock,
        ticket::kernel::detail::{
            repository::dynamodb::ticket::{ConnectionTicket, TableTicket},
            token::authorize::decoder::JwtAuthorizeTokenDecoder,
        },
        user::kernel::detail::repository::dynamodb::user::{ConnectionUser, TableUser},
    },
    common::api::{feature::AsInfra, logger::detail::StdoutJsonLogger},
};

use crate::auth::ticket::authorize::infra::{
    AuthorizeInfra, AuthorizeLogger, AuthorizeRepository, AuthorizeWithTokenLogger,
    CheckAuthorizeTokenInfra, CheckAuthorizeTokenLogger,
};

use crate::{
    auth::{
        kernel::data::ExpansionLimitDateTime,
        ticket::{
            authorize::data::{AuthorizeError, AuthorizeSuccess, ValidateAuthorizeFieldsError},
            kernel::data::{
                AuthPermissionError, AuthPermissionGranted, AuthTicket, DecodeAuthorizeTokenError,
                ValidateAuthorizeTokenError,
            },
        },
        user::kernel::data::AuthUserId,
    },
    common::api::repository::data::RepositoryError,
};

pub struct LiveCheckAuthorizeTokenInfra {
    token_decoder: JwtAuthorizeTokenDecoder,
}

impl AsInfra<LiveCheckAuthorizeTokenInfra> for Data<ProxyAppFeature> {
    fn as_infra(&self) -> LiveCheckAuthorizeTokenInfra {
        LiveCheckAuthorizeTokenInfra {
            token_decoder: self.as_infra(),
        }
    }
}

impl AsInfra<LiveCheckAuthorizeTokenInfra> for Arc<CoreAppFeature> {
    fn as_infra(&self) -> LiveCheckAuthorizeTokenInfra {
        LiveCheckAuthorizeTokenInfra {
            token_decoder: self.as_infra(),
        }
    }
}

impl CheckAuthorizeTokenInfra for LiveCheckAuthorizeTokenInfra {
    type TokenDecoder = JwtAuthorizeTokenDecoder;

    fn token_decoder(&self) -> &Self::TokenDecoder {
        &self.token_decoder
    }
}

impl CheckAuthorizeTokenLogger for StdoutJsonLogger {
    fn try_to_check_authorize_token(&self) {
        self.debug(format!("try to check authorize-token"));
    }
    fn invalid_request(&self, err: ValidateAuthorizeTokenError) -> ValidateAuthorizeTokenError {
        self.incident(format!("invalid authorize request; {}", &err));
        err
    }
    fn invalid_token(&self, err: DecodeAuthorizeTokenError) -> DecodeAuthorizeTokenError {
        match err {
            DecodeAuthorizeTokenError::Expired => {
                self.debug(format!("authorize-token expired; {}", &err))
            }
            DecodeAuthorizeTokenError::Invalid(_) => {
                self.incident(format!("invalid authorize-token; {}", &err))
            }
        }
        err
    }
    fn forbidden(&self, err: AuthPermissionError) -> AuthPermissionError {
        self.incident(format!("forbidden; {}", &err));
        err
    }
    fn succeed_to_check_authorize_token(
        &self,
        granted: AuthPermissionGranted,
    ) -> AuthPermissionGranted {
        self.debug(format!(
            "succeed to check authorize-token; granted: {}",
            &granted
        ));
        granted
    }
}

pub struct LiveAuthorizeInfra {
    token_decoder: JwtAuthorizeTokenDecoder,
    repository: LiveAuthorizeRepository,
    clock: ChronoAuthClock,
}

impl AsInfra<LiveAuthorizeInfra> for Arc<AuthAppFeature> {
    fn as_infra(&self) -> LiveAuthorizeInfra {
        LiveAuthorizeInfra {
            token_decoder: self.as_infra(),
            repository: LiveAuthorizeRepository {
                ticket: self.as_infra(),
                user: self.as_infra(),
            },
            clock: ChronoAuthClock,
        }
    }
}

impl AuthorizeInfra for LiveAuthorizeInfra {
    type TokenDecoder = JwtAuthorizeTokenDecoder;
    type Repository = LiveAuthorizeRepository;
    type Clock = ChronoAuthClock;

    fn token_decoder(&self) -> &Self::TokenDecoder {
        &self.token_decoder
    }
    fn repository(&self) -> &Self::Repository {
        &self.repository
    }
    fn clock(&self) -> &Self::Clock {
        &self.clock
    }
}

pub struct LiveAuthorizeRepository {
    ticket: ConnectionTicket,
    user: ConnectionUser,
}

#[async_trait::async_trait]
impl AuthorizeRepository for LiveAuthorizeRepository {
    async fn lookup_expansion_limit(
        &self,
        ticket: &AuthTicket,
    ) -> Result<Option<ExpansionLimitDateTime>, RepositoryError> {
        TableTicket::get_expansion_limit(&self.ticket, ticket.clone()).await
    }

    async fn lookup_permission_granted(
        &self,
        user_id: &AuthUserId,
    ) -> Result<Option<AuthPermissionGranted>, RepositoryError> {
        TableUser::get_granted(&self.user, user_id.clone()).await
    }
}

impl AuthorizeLogger for StdoutJsonLogger {
    fn try_to_authorize(&self) {
        self.info(format!("try to authorize"));
    }
    fn invalid_request(&self, err: ValidateAuthorizeFieldsError) -> ValidateAuthorizeFieldsError {
        self.incident(format!("failed to convert fields; {}", err));
        err
    }
    fn invalid_token(&self, err: DecodeAuthorizeTokenError) -> DecodeAuthorizeTokenError {
        self.incident(format!("failed to decode token; {}", err));
        err
    }
    fn forbidden(&self, err: AuthPermissionError) -> AuthPermissionError {
        self.incident(format!("forbidden; {}", err));
        err
    }
    fn failed_to_lookup_expansion_limit(&self, err: RepositoryError) -> RepositoryError {
        self.fatal(format!("failed to lookup expansion-limit; {}", err));
        err
    }
    fn failed_to_lookup_permission_granted(&self, err: RepositoryError) -> RepositoryError {
        self.fatal(format!("failed to lookup permission-granted; {}", err));
        err
    }
    fn expansion_limit_not_found(&self, err: AuthorizeError) -> AuthorizeError {
        self.fatal(format!("expansion-limit not found; {}", err));
        err
    }
    fn ticket_has_expired(&self, err: AuthorizeError) -> AuthorizeError {
        self.info(format!("ticket has expired; {}", err));
        err
    }
    fn permission_granted_not_found(&self, err: AuthorizeError) -> AuthorizeError {
        self.incident(format!("permission-granted not found; {}", err));
        err
    }
    fn authorized(&self, auth: AuthorizeSuccess) -> AuthorizeSuccess {
        self.info(format!("authorized; {}", auth));
        auth
    }
}

impl AuthorizeWithTokenLogger for StdoutJsonLogger {
    fn invalid_request(&self, err: ValidateAuthorizeFieldsError) -> ValidateAuthorizeFieldsError {
        self.incident(format!("invalid authorize request; {}", &err));
        err
    }
    fn invalid_token(&self, err: DecodeAuthorizeTokenError) -> DecodeAuthorizeTokenError {
        match err {
            DecodeAuthorizeTokenError::Expired => {
                self.debug(format!("authorize token expired; {}", &err))
            }
            DecodeAuthorizeTokenError::Invalid(_) => {
                self.incident(format!("invalid authorize token; {}", &err))
            }
        }
        err
    }
    fn forbidden(&self, err: AuthPermissionError) -> AuthPermissionError {
        self.incident(format!("forbidden; {}", &err));
        err
    }
    fn authorized(&self) {
        self.debug(format!("authorized"));
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
                token::authorize::decoder::test::MockAuthorizeTokenDecoder,
            },
            user::kernel::detail::repository::memory::{user::MapUser, StoreUser},
        },
        common::api::feature::AsInfra,
    };

    use crate::auth::ticket::authorize::infra::{
        AuthorizeInfra, AuthorizeRepository, CheckAuthorizeTokenInfra,
    };

    use crate::{
        auth::{
            kernel::data::ExpansionLimitDateTime,
            ticket::kernel::data::{AuthPermissionGranted, AuthTicket},
            user::kernel::data::AuthUserId,
        },
        common::api::repository::data::RepositoryError,
    };

    pub struct MockCheckAuthorizeTokenInfra {
        token_decoder: MockAuthorizeTokenDecoder,
    }

    impl AsInfra<MockCheckAuthorizeTokenInfra> for MockAuthorizeTokenDecoder {
        fn as_infra(&self) -> MockCheckAuthorizeTokenInfra {
            MockCheckAuthorizeTokenInfra {
                token_decoder: self.clone(),
            }
        }
    }

    impl CheckAuthorizeTokenInfra for MockCheckAuthorizeTokenInfra {
        type TokenDecoder = MockAuthorizeTokenDecoder;

        fn token_decoder(&self) -> &Self::TokenDecoder {
            &self.token_decoder
        }
    }

    pub struct MockAuthorizeInfra {
        token_decoder: MockAuthorizeTokenDecoder,
        repository: MockAuthorizeRepository,
        clock: MockChronoAuthClock,
    }

    impl AsInfra<MockAuthorizeInfra>
        for (
            MockAuthorizeTokenDecoder,
            Arc<StoreTicket>,
            Arc<StoreUser>,
            MockChronoAuthClock,
        )
    {
        fn as_infra(&self) -> MockAuthorizeInfra {
            MockAuthorizeInfra {
                token_decoder: self.0.clone(),
                repository: MockAuthorizeRepository {
                    ticket: Arc::clone(&self.1),
                    user: Arc::clone(&self.2),
                },
                clock: self.3.clone(),
            }
        }
    }

    impl AuthorizeInfra for MockAuthorizeInfra {
        type TokenDecoder = MockAuthorizeTokenDecoder;
        type Repository = MockAuthorizeRepository;
        type Clock = MockChronoAuthClock;

        fn token_decoder(&self) -> &Self::TokenDecoder {
            &self.token_decoder
        }
        fn repository(&self) -> &Self::Repository {
            &self.repository
        }
        fn clock(&self) -> &Self::Clock {
            &self.clock
        }
    }

    pub struct MockAuthorizeRepository {
        ticket: Arc<StoreTicket>,
        user: Arc<StoreUser>,
    }

    #[async_trait::async_trait]
    impl AuthorizeRepository for MockAuthorizeRepository {
        async fn lookup_expansion_limit(
            &self,
            ticket: &AuthTicket,
        ) -> Result<Option<ExpansionLimitDateTime>, RepositoryError> {
            Ok(MapTicket::get_expansion_limit(
                &self.ticket,
                &ticket.ticket_id,
            ))
        }

        async fn lookup_permission_granted(
            &self,
            user_id: &AuthUserId,
        ) -> Result<Option<AuthPermissionGranted>, RepositoryError> {
            Ok(MapUser::get_granted(&self.user, user_id))
        }
    }
}

use std::sync::Arc;

use uuid::Uuid;

use crate::x_outside_feature::auth::feature::AuthAppFeature;

use crate::{
    auth::{
        kernel::detail::ChronoAuthClock,
        user::{
            kernel::detail::repository::dynamodb::{
                login_id::{ConnectionLoginId, TableLoginId},
                reset_token::{ConnectionResetToken, TableResetToken},
            },
            password::reset::kernel::detail::{
                email::send_reset_token::EmailSendResetToken,
                token::encoder::JwtResetPasswordTokenEncoder,
            },
        },
    },
    common::api::{feature::AsInfra, logger::detail::StdoutJsonLogger},
};

use crate::auth::user::password::reset::request_token::infra::{
    RequestResetPasswordTokenConfig, RequestResetPasswordTokenInfra,
    RequestResetPasswordTokenLogger, RequestResetPasswordTokenRepository, ResetPasswordIdGenerator,
    ResetPasswordTokenNotifier,
};

use crate::{
    auth::{
        kernel::data::{AuthDateTime, ExpireDateTime},
        user::{
            kernel::data::AuthUserId,
            login_id::kernel::data::{LoginId, ValidateLoginIdError},
            password::reset::{
                kernel::data::{
                    ResetPasswordId, ResetPasswordToken, ResetPasswordTokenDestination,
                },
                request_token::data::{
                    EncodeResetTokenError, NotifyResetTokenError, NotifyResetTokenResponse,
                    RequestResetPasswordTokenError,
                },
            },
        },
    },
    common::api::repository::data::RepositoryError,
};

pub struct LiveRequestResetPasswordTokenInfra {
    clock: ChronoAuthClock,
    repository: LiveRegisterResetPasswordTokenRepository,
    token_generator: UuidResetTokenGenerator,
    token_encoder: JwtResetPasswordTokenEncoder,
    token_notifier: EmailResetPasswordTokenNotifier,
    config: RequestResetPasswordTokenConfig,
}

impl AsInfra<LiveRequestResetPasswordTokenInfra> for Arc<AuthAppFeature> {
    fn as_infra(&self) -> LiveRequestResetPasswordTokenInfra {
        LiveRequestResetPasswordTokenInfra {
            clock: ChronoAuthClock,
            repository: LiveRegisterResetPasswordTokenRepository {
                login_id: self.as_infra(),
                reset_token: self.as_infra(),
            },
            token_generator: UuidResetTokenGenerator,
            token_encoder: self.as_infra(),
            token_notifier: EmailResetPasswordTokenNotifier {
                send_request_token: self.as_infra(),
            },
            config: RequestResetPasswordTokenConfig {
                token_expires: self.config.reset_token_expires,
            },
        }
    }
}

impl RequestResetPasswordTokenInfra for LiveRequestResetPasswordTokenInfra {
    type Clock = ChronoAuthClock;
    type Repository = LiveRegisterResetPasswordTokenRepository;
    type IdGenerator = UuidResetTokenGenerator;
    type TokenEncoder = JwtResetPasswordTokenEncoder;
    type TokenNotifier = EmailResetPasswordTokenNotifier;

    fn clock(&self) -> &Self::Clock {
        &self.clock
    }
    fn repository(&self) -> &Self::Repository {
        &self.repository
    }
    fn id_generator(&self) -> &Self::IdGenerator {
        &self.token_generator
    }
    fn token_encoder(&self) -> &Self::TokenEncoder {
        &self.token_encoder
    }
    fn token_notifier(&self) -> &Self::TokenNotifier {
        &self.token_notifier
    }
    fn config(&self) -> &RequestResetPasswordTokenConfig {
        &self.config
    }
}

pub struct LiveRegisterResetPasswordTokenRepository {
    login_id: ConnectionLoginId,
    reset_token: ConnectionResetToken,
}

#[async_trait::async_trait]
impl RequestResetPasswordTokenRepository for LiveRegisterResetPasswordTokenRepository {
    async fn lookup_user(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<(AuthUserId, ResetPasswordTokenDestination)>, RepositoryError> {
        TableLoginId::get_reset_token_entry(&self.login_id, login_id.clone()).await
    }

    async fn register_reset_token(
        &self,
        reset_token: ResetPasswordId,
        user_id: AuthUserId,
        destination: ResetPasswordTokenDestination,
        expires: ExpireDateTime,
        requested_at: AuthDateTime,
    ) -> Result<(), RepositoryError> {
        TableResetToken::put_reset_token(
            &self.reset_token,
            reset_token,
            user_id,
            destination,
            expires,
            requested_at,
        )
        .await
    }
}

pub struct UuidResetTokenGenerator;

impl ResetPasswordIdGenerator for UuidResetTokenGenerator {
    fn generate(&self) -> ResetPasswordId {
        ResetPasswordId::restore(Uuid::new_v4().to_string())
    }
}

pub struct EmailResetPasswordTokenNotifier {
    send_request_token: EmailSendResetToken,
}

#[async_trait::async_trait]
impl ResetPasswordTokenNotifier for EmailResetPasswordTokenNotifier {
    async fn notify(
        &self,
        destination: ResetPasswordTokenDestination,
        token: ResetPasswordToken,
    ) -> Result<NotifyResetTokenResponse, NotifyResetTokenError> {
        match destination {
            ResetPasswordTokenDestination::None => Err(NotifyResetTokenError::NoDestination),
            ResetPasswordTokenDestination::Email(destination) => {
                let message_id = self.send_request_token.send(destination, token).await?;
                Ok(NotifyResetTokenResponse::new(message_id))
            }
        }
    }
}

impl RequestResetPasswordTokenLogger for StdoutJsonLogger {
    fn try_to_request_reset_password_token(&self) {
        self.info(format!("try to check authenticate-token"));
    }
    fn invalid_request(&self, err: ValidateLoginIdError) -> ValidateLoginIdError {
        self.debug(format!("failed to validate login-id; {}", err));
        err
    }
    fn failed_to_lookup_user(&self, err: RepositoryError) -> RepositoryError {
        self.fatal(format!("failed to lookup user; {}", err));
        err
    }
    fn user_not_found(
        &self,
        err: RequestResetPasswordTokenError,
    ) -> RequestResetPasswordTokenError {
        self.fatal(format!("user not found; {}", err));
        err
    }
    fn calculate_token_expires(&self, expires: ExpireDateTime) -> ExpireDateTime {
        self.debug(format!(
            "calculated reset-password-token expires; {}",
            expires
        ));
        expires
    }
    fn failed_to_register_reset_token(&self, err: RepositoryError) -> RepositoryError {
        self.fatal(format!("failed to register reset-password-token; {}", err));
        err
    }
    fn failed_to_encode_reset_token(&self, err: EncodeResetTokenError) -> EncodeResetTokenError {
        self.fatal(format!("failed to encode reset-password-token; {}", err));
        err
    }
    fn failed_to_notify_reset_token(&self, err: NotifyResetTokenError) -> NotifyResetTokenError {
        self.fatal(format!("failed to notify reset-password-token; {}", err));
        err
    }
    fn succeed_to_request_reset_password_token(
        &self,
        response: NotifyResetTokenResponse,
    ) -> NotifyResetTokenResponse {
        self.info(format!("succeed to request reset-password-token"));
        response
    }
}

#[cfg(test)]
pub mod test {
    use std::sync::Arc;

    use crate::{
        auth::{
            kernel::detail::test::MockChronoAuthClock,
            user::{
                kernel::detail::repository::memory::{
                    login_id::MapLoginId, reset_token::MapResetToken, StoreLoginId, StoreResetToken,
                },
                password::reset::kernel::detail::token::encoder::test::MockResetTokenEncoder,
            },
        },
        common::api::feature::AsInfra,
    };

    use crate::auth::user::password::reset::request_token::infra::{
        RequestResetPasswordTokenConfig, RequestResetPasswordTokenInfra,
        RequestResetPasswordTokenRepository, ResetPasswordIdGenerator, ResetPasswordTokenNotifier,
    };

    use crate::{
        auth::{
            kernel::data::{AuthDateTime, ExpireDateTime},
            user::{
                kernel::data::AuthUserId,
                login_id::kernel::data::LoginId,
                password::reset::{
                    kernel::data::{
                        ResetPasswordId, ResetPasswordToken, ResetPasswordTokenDestination,
                    },
                    request_token::data::{NotifyResetTokenError, NotifyResetTokenResponse},
                },
            },
        },
        common::api::repository::data::RepositoryError,
    };

    pub struct MockRequestResetPasswordTokenInfra {
        clock: MockChronoAuthClock,
        repository: MockRequestResetPasswordTokenRepository,
        token_generator: MockResetTokenGenerator,
        token_encoder: MockResetTokenEncoder,
        token_notifier: MockResetTokenNotifier,
        config: RequestResetPasswordTokenConfig,
    }

    impl AsInfra<MockRequestResetPasswordTokenInfra>
        for (
            MockChronoAuthClock,
            Arc<StoreLoginId>,
            Arc<StoreResetToken>,
            MockResetTokenGenerator,
            RequestResetPasswordTokenConfig,
        )
    {
        fn as_infra(&self) -> MockRequestResetPasswordTokenInfra {
            MockRequestResetPasswordTokenInfra {
                clock: self.0.clone(),
                repository: MockRequestResetPasswordTokenRepository {
                    login_id: Arc::clone(&self.1),
                    reset_token: Arc::clone(&self.2),
                },
                token_generator: self.3.clone(),
                token_encoder: MockResetTokenEncoder,
                token_notifier: MockResetTokenNotifier,
                config: self.4.clone(),
            }
        }
    }

    impl RequestResetPasswordTokenInfra for MockRequestResetPasswordTokenInfra {
        type Clock = MockChronoAuthClock;
        type Repository = MockRequestResetPasswordTokenRepository;
        type IdGenerator = MockResetTokenGenerator;
        type TokenEncoder = MockResetTokenEncoder;
        type TokenNotifier = MockResetTokenNotifier;

        fn clock(&self) -> &Self::Clock {
            &self.clock
        }
        fn repository(&self) -> &Self::Repository {
            &self.repository
        }
        fn id_generator(&self) -> &Self::IdGenerator {
            &self.token_generator
        }
        fn token_encoder(&self) -> &Self::TokenEncoder {
            &self.token_encoder
        }
        fn token_notifier(&self) -> &Self::TokenNotifier {
            &self.token_notifier
        }
        fn config(&self) -> &RequestResetPasswordTokenConfig {
            &self.config
        }
    }

    pub struct MockRequestResetPasswordTokenRepository {
        login_id: Arc<StoreLoginId>,
        reset_token: Arc<StoreResetToken>,
    }

    #[async_trait::async_trait]
    impl RequestResetPasswordTokenRepository for MockRequestResetPasswordTokenRepository {
        async fn lookup_user(
            &self,
            login_id: &LoginId,
        ) -> Result<Option<(AuthUserId, ResetPasswordTokenDestination)>, RepositoryError> {
            Ok(MapLoginId::get_reset_token_entry(&self.login_id, login_id))
        }

        async fn register_reset_token(
            &self,
            reset_token: ResetPasswordId,
            user_id: AuthUserId,
            destination: ResetPasswordTokenDestination,
            expires: ExpireDateTime,
            requested_at: AuthDateTime,
        ) -> Result<(), RepositoryError> {
            Ok(MapResetToken::insert_reset_token(
                &self.reset_token,
                reset_token,
                user_id,
                destination,
                expires,
                requested_at,
            ))
        }
    }

    #[derive(Clone)]
    pub struct MockResetTokenGenerator {
        token: ResetPasswordId,
    }

    impl MockResetTokenGenerator {
        pub const fn new(token: ResetPasswordId) -> Self {
            Self { token }
        }
    }

    impl ResetPasswordIdGenerator for MockResetTokenGenerator {
        fn generate(&self) -> ResetPasswordId {
            self.token.clone()
        }
    }

    pub struct MockResetTokenNotifier;

    #[async_trait::async_trait]
    impl ResetPasswordTokenNotifier for MockResetTokenNotifier {
        async fn notify(
            &self,
            _destination: ResetPasswordTokenDestination,
            _token: ResetPasswordToken,
        ) -> Result<NotifyResetTokenResponse, NotifyResetTokenError> {
            Ok(NotifyResetTokenResponse::new("message-id".into()))
        }
    }
}

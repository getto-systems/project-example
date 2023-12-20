use std::sync::Arc;

use crate::x_outside_feature::auth::feature::AuthAppFeature;

use crate::{
    auth::{
        kernel::detail::ChronoAuthClock,
        user::{
            kernel::detail::repository::dynamodb::{
                reset_token::{ConnectionResetToken, TableResetToken},
                user::{ConnectionUser, TableUser},
            },
            password::{
                kernel::detail::password_hasher::Argon2PasswordHasher,
                reset::kernel::detail::{
                    email::notify_password_reset::EmailNotifyPasswordReset,
                    token::decoder::JwtResetPasswordTokenDecoder,
                },
            },
        },
    },
    common::api::{feature::AsInfra, logger::detail::StdoutJsonLogger},
};

use crate::auth::user::password::{
    kernel::infra::HashedPassword,
    reset::reset::infra::{
        ResetPasswordInfra, ResetPasswordLogger, ResetPasswordNotifier, ResetPasswordRepository,
        ResetPasswordTokenMoment,
    },
};

use crate::{
    auth::{
        kernel::data::AuthDateTime,
        ticket::kernel::data::{AuthPermissionGranted, AuthenticateSuccess},
        user::{
            kernel::data::AuthUserId,
            password::{
                kernel::data::PasswordHashError,
                reset::{
                    kernel::data::{ResetPasswordId, ResetPasswordTokenDestination},
                    reset::data::{
                        DecodeResetTokenError, NotifyResetPasswordError,
                        NotifyResetPasswordResponse, ResetPasswordError,
                        ValidateResetPasswordFieldsError,
                    },
                },
            },
        },
    },
    common::api::repository::data::RepositoryError,
};

pub struct LiveResetPasswordInfra {
    clock: ChronoAuthClock,
    repository: LiveResetPasswordRepository,
    token_decoder: JwtResetPasswordTokenDecoder,
    reset_notifier: EmailResetPasswordNotifier,
}

impl AsInfra<LiveResetPasswordInfra> for Arc<AuthAppFeature> {
    fn as_infra(&self) -> LiveResetPasswordInfra {
        LiveResetPasswordInfra {
            clock: ChronoAuthClock,
            repository: LiveResetPasswordRepository {
                user: self.as_infra(),
                reset_token: self.as_infra(),
            },
            token_decoder: self.as_infra(),
            reset_notifier: EmailResetPasswordNotifier {
                notify_password_reset: self.as_infra(),
            },
        }
    }
}

impl ResetPasswordInfra for LiveResetPasswordInfra {
    type Clock = ChronoAuthClock;
    type Repository = LiveResetPasswordRepository;
    type PasswordHasher = Argon2PasswordHasher;
    type TokenDecoder = JwtResetPasswordTokenDecoder;
    type ResetNotifier = EmailResetPasswordNotifier;

    fn clock(&self) -> &Self::Clock {
        &self.clock
    }
    fn repository(&self) -> &Self::Repository {
        &self.repository
    }
    fn token_decoder(&self) -> &Self::TokenDecoder {
        &self.token_decoder
    }
    fn reset_notifier(&self) -> &Self::ResetNotifier {
        &self.reset_notifier
    }
}

pub struct LiveResetPasswordRepository {
    user: ConnectionUser,
    reset_token: ConnectionResetToken,
}

#[async_trait::async_trait]
impl ResetPasswordRepository for LiveResetPasswordRepository {
    async fn lookup_reset_token_entry(
        &self,
        reset_token: &ResetPasswordId,
    ) -> Result<
        Option<(
            AuthUserId,
            ResetPasswordTokenDestination,
            ResetPasswordTokenMoment,
        )>,
        RepositoryError,
    > {
        TableResetToken::get_reset_token_entry(&self.reset_token, reset_token.clone()).await
    }

    async fn lookup_permission_granted(
        &self,
        user_id: &AuthUserId,
    ) -> Result<Option<AuthPermissionGranted>, RepositoryError> {
        TableUser::get_granted(&self.user, user_id.clone()).await
    }

    async fn consume_reset_id(
        &self,
        reset_token: ResetPasswordId,
        reset_at: AuthDateTime,
    ) -> Result<(), RepositoryError> {
        TableResetToken::update_reset_at(&self.reset_token, reset_token, reset_at).await
    }
    async fn update_password(
        &self,
        user_id: AuthUserId,
        new_password: HashedPassword,
    ) -> Result<(), RepositoryError> {
        TableUser::update_password(&self.user, user_id, new_password).await
    }
}

pub struct EmailResetPasswordNotifier {
    notify_password_reset: EmailNotifyPasswordReset,
}

#[async_trait::async_trait]
impl ResetPasswordNotifier for EmailResetPasswordNotifier {
    async fn notify(
        &self,
        destination: ResetPasswordTokenDestination,
    ) -> Result<NotifyResetPasswordResponse, NotifyResetPasswordError> {
        match destination {
            ResetPasswordTokenDestination::None => Ok(NotifyResetPasswordResponse::NoDestination),
            ResetPasswordTokenDestination::Email(destination) => {
                let message_id = self.notify_password_reset.send(destination).await?;
                Ok(NotifyResetPasswordResponse::Send(message_id))
            }
        }
    }
}

impl ResetPasswordLogger for StdoutJsonLogger {
    fn try_to_reset_password(&self) {
        self.info(format!("try to reset password"));
    }
    fn invalid_request(
        &self,
        err: ValidateResetPasswordFieldsError,
    ) -> ValidateResetPasswordFieldsError {
        self.fatal(format!("failed to validate reset password fields; {}", err));
        err
    }
    fn failed_to_decode_token(&self, err: DecodeResetTokenError) -> DecodeResetTokenError {
        self.incident(format!("failed to decode token; {}", err));
        err
    }
    fn failed_to_lookup_reset_token_entry(&self, err: RepositoryError) -> RepositoryError {
        self.fatal(format!("failed to lookup reset-token entry; {}", err));
        err
    }
    fn reset_token_not_found(&self, err: ResetPasswordError) -> ResetPasswordError {
        self.incident(format!("reset-token not found; {}", err));
        err
    }
    fn failed_to_lookup_permission_granted(&self, err: RepositoryError) -> RepositoryError {
        self.fatal(format!("failed to lookup permission-granted; {}", err));
        err
    }
    fn already_reset(&self, err: ResetPasswordError) -> ResetPasswordError {
        self.incident(format!("already reset; {}", err));
        err
    }
    fn expired(&self, err: ResetPasswordError) -> ResetPasswordError {
        self.incident(format!("reset-id expired; {}", err));
        err
    }
    fn failed_to_hash_password(&self, err: PasswordHashError) -> PasswordHashError {
        self.fatal(format!("failed to hash password; {}", err));
        err
    }
    fn failed_to_consume_reset_id(&self, err: RepositoryError) -> RepositoryError {
        self.fatal(format!("failed to consume reset-id; {}", err));
        err
    }
    fn failed_to_update_password(&self, err: RepositoryError) -> RepositoryError {
        self.fatal(format!("failed to update password; {}", err));
        err
    }
    fn failed_to_notify(&self, err: NotifyResetPasswordError) -> NotifyResetPasswordError {
        self.fatal(format!("failed to notify; {}", err));
        err
    }
    fn succeed_to_notify(
        &self,
        response: NotifyResetPasswordResponse,
    ) -> NotifyResetPasswordResponse {
        self.info(format!("succeed to notify"));
        response
    }
    fn succeed_to_reset_password(&self, auth: AuthenticateSuccess) -> AuthenticateSuccess {
        self.info(format!("succeed to reset password; {}", auth));
        auth
    }
}

#[cfg(test)]
pub mod test {
    use std::sync::Arc;

    use crate::{
        auth::{
            kernel::{data::AuthDateTime, detail::test::MockChronoAuthClock},
            ticket::kernel::data::AuthPermissionGranted,
            user::{
                kernel::{
                    data::AuthUserId,
                    detail::repository::memory::{
                        reset_token::MapResetToken, user::MapUser, StoreResetToken, StoreUser,
                    },
                },
                password::{
                    kernel::{
                        detail::password_hasher::test::PlainPasswordHasher, infra::HashedPassword,
                    },
                    reset::{
                        kernel::{
                            data::ResetPasswordId,
                            detail::token::decoder::test::MockResetTokenDecoder,
                        },
                        reset::infra::{ResetPasswordRepository, ResetPasswordTokenMoment},
                    },
                },
            },
        },
        common::api::{feature::AsInfra, repository::data::RepositoryError},
    };

    use crate::auth::user::password::reset::reset::infra::{
        ResetPasswordInfra, ResetPasswordNotifier,
    };

    use crate::auth::user::password::reset::{
        kernel::data::ResetPasswordTokenDestination,
        reset::data::{NotifyResetPasswordError, NotifyResetPasswordResponse},
    };

    pub struct MockResetPasswordInfra {
        clock: MockChronoAuthClock,
        repository: MockResetPasswordRepository,
        token_decoder: MockResetTokenDecoder,
        reset_notifier: MockResetPasswordNotifier,
    }

    impl AsInfra<MockResetPasswordInfra>
        for (
            MockChronoAuthClock,
            Arc<StoreUser>,
            Arc<StoreResetToken>,
            MockResetTokenDecoder,
        )
    {
        fn as_infra(&self) -> MockResetPasswordInfra {
            MockResetPasswordInfra {
                clock: self.0.clone(),
                repository: MockResetPasswordRepository {
                    user: Arc::clone(&self.1),
                    reset_token: Arc::clone(&self.2),
                },
                token_decoder: self.3.clone(),
                reset_notifier: MockResetPasswordNotifier,
            }
        }
    }

    impl ResetPasswordInfra for MockResetPasswordInfra {
        type Clock = MockChronoAuthClock;
        type Repository = MockResetPasswordRepository;
        type PasswordHasher = PlainPasswordHasher;
        type TokenDecoder = MockResetTokenDecoder;
        type ResetNotifier = MockResetPasswordNotifier;

        fn clock(&self) -> &Self::Clock {
            &self.clock
        }
        fn repository(&self) -> &Self::Repository {
            &self.repository
        }
        fn token_decoder(&self) -> &Self::TokenDecoder {
            &self.token_decoder
        }
        fn reset_notifier(&self) -> &Self::ResetNotifier {
            &self.reset_notifier
        }
    }

    pub struct MockResetPasswordRepository {
        user: Arc<StoreUser>,
        reset_token: Arc<StoreResetToken>,
    }

    #[async_trait::async_trait]
    impl ResetPasswordRepository for MockResetPasswordRepository {
        async fn lookup_reset_token_entry(
            &self,
            reset_token: &ResetPasswordId,
        ) -> Result<
            Option<(
                AuthUserId,
                ResetPasswordTokenDestination,
                ResetPasswordTokenMoment,
            )>,
            RepositoryError,
        > {
            Ok(MapResetToken::get_reset_token_entry(
                &self.reset_token,
                reset_token,
            ))
        }

        async fn lookup_permission_granted(
            &self,
            user_id: &AuthUserId,
        ) -> Result<Option<AuthPermissionGranted>, RepositoryError> {
            Ok(MapUser::get_granted(&self.user, user_id))
        }

        async fn consume_reset_id(
            &self,
            reset_token: ResetPasswordId,
            reset_at: AuthDateTime,
        ) -> Result<(), RepositoryError> {
            Ok(MapResetToken::update_reset_at(
                &self.reset_token,
                reset_token,
                reset_at,
            ))
        }
        async fn update_password(
            &self,
            user_id: AuthUserId,
            new_password: HashedPassword,
        ) -> Result<(), RepositoryError> {
            Ok(MapUser::update_password(&self.user, user_id, new_password))
        }
    }

    pub struct MockResetPasswordNotifier;

    #[async_trait::async_trait]
    impl ResetPasswordNotifier for MockResetPasswordNotifier {
        async fn notify(
            &self,
            _destination: ResetPasswordTokenDestination,
        ) -> Result<NotifyResetPasswordResponse, NotifyResetPasswordError> {
            Ok(NotifyResetPasswordResponse::Send("message-id".into()))
        }
    }
}

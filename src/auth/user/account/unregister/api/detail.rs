use std::sync::Arc;

use crate::x_outside_feature::auth::feature::AuthAppFeature;

use crate::{
    auth::{
        feature::AsAuthorizedInfra,
        ticket::kernel::detail::repository::dynamodb::ticket::{ConnectionTicket, TableTicket},
        user::kernel::detail::repository::dynamodb::{
            login_id::{ConnectionLoginId, TableLoginId},
            user::{ConnectionUser, TableUser},
        },
    },
    common::api::{feature::AsInfra, logger::detail::StdoutJsonLogger},
};

use crate::auth::user::account::unregister::action::UnregisterAuthUserAccountInfo;

use crate::auth::user::account::unregister::infra::{
    UnregisterAuthUserAccountInfra, UnregisterAuthUserAccountLogger,
    UnregisterAuthUserAccountRepository,
};

use crate::{
    auth::{
        ticket::{authorize::data::AuthorizeSuccess, kernel::data::AuthPermissionRequired},
        user::{
            account::unregister::data::{
                UnregisterAuthUserAccountError, UnregisterAuthUserAccountSuccess,
            },
            kernel::data::AuthUserId,
            login_id::kernel::data::{LoginId, ValidateLoginIdError},
        },
    },
    common::api::repository::data::RepositoryError,
};

pub struct LiveUnregisterAuthUserAccountInfra {
    repository: LiveUnregisterAuthUserAccountRepository,
}

impl AsAuthorizedInfra<LiveUnregisterAuthUserAccountInfra> for Arc<AuthAppFeature> {
    fn required(&self) -> AuthPermissionRequired {
        UnregisterAuthUserAccountInfo::required()
    }
    fn as_authorized_infra(&self, _: &AuthorizeSuccess) -> LiveUnregisterAuthUserAccountInfra {
        LiveUnregisterAuthUserAccountInfra {
            repository: LiveUnregisterAuthUserAccountRepository {
                ticket: self.as_infra(),
                login_id: self.as_infra(),
                user: self.as_infra(),
            },
        }
    }
}

impl UnregisterAuthUserAccountInfra for LiveUnregisterAuthUserAccountInfra {
    type Repository = LiveUnregisterAuthUserAccountRepository;

    fn repository(&self) -> &Self::Repository {
        &self.repository
    }
}

pub struct LiveUnregisterAuthUserAccountRepository {
    ticket: ConnectionTicket,
    login_id: ConnectionLoginId,
    user: ConnectionUser,
}

#[async_trait::async_trait]
impl UnregisterAuthUserAccountRepository for LiveUnregisterAuthUserAccountRepository {
    async fn lookup_user_id(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<AuthUserId>, RepositoryError> {
        TableLoginId::get_user_id(&self.login_id, login_id.clone()).await
    }

    async fn unregister_user(
        &self,
        user_id: &AuthUserId,
        login_id: &LoginId,
    ) -> Result<(), RepositoryError> {
        let entry = TableUser::get_entry(&self.user, user_id.clone()).await?;
        TableUser::delete_entry(&self.user, user_id.clone()).await?;

        {
            let result = TableLoginId::delete_entry(&self.login_id, login_id.clone()).await;
            if result.is_err() {
                if let Some(entry) = entry {
                    TableUser::put_entry(&self.user, user_id.clone(), entry).await?;
                }
            }
            result?;
        }

        Ok(())
    }

    async fn discard_all_ticket(&self, user_id: &AuthUserId) -> Result<(), RepositoryError> {
        for ticket_id in TableTicket::query_ticket_id(&self.ticket, user_id.clone()).await? {
            TableTicket::delete_ticket(&self.ticket, ticket_id.clone(), user_id.clone()).await?
        }
        Ok(())
    }
}

impl UnregisterAuthUserAccountLogger for StdoutJsonLogger {
    fn try_to_unregister_auth_user_account(&self) {
        self.info(format!("try to unregister auth-user-account"));
    }
    fn invalid_request(&self, err: ValidateLoginIdError) -> ValidateLoginIdError {
        self.fatal(format!(
            "failed to validate unregister auth-user-account fields; {}",
            err
        ));
        err
    }
    fn failed_to_lookup_user_id(&self, err: RepositoryError) -> RepositoryError {
        self.fatal(format!("failed to lookup user-id; {}", err));
        err
    }
    fn user_id_not_found(
        &self,
        err: UnregisterAuthUserAccountError,
    ) -> UnregisterAuthUserAccountError {
        self.fatal(format!("user-id not found; {}", err));
        err
    }
    fn failed_to_unregister_user(&self, err: RepositoryError) -> RepositoryError {
        self.fatal(format!("failed to unregister user; {}", err));
        err
    }
    fn failed_to_discard_all_ticket(&self, err: RepositoryError) -> RepositoryError {
        self.fatal(format!("failed to discard all ticket; {}", err));
        err
    }
    fn succeed_to_unregister_auth_user_account(
        &self,
        success: UnregisterAuthUserAccountSuccess,
    ) -> UnregisterAuthUserAccountSuccess {
        self.info(format!("succeed to unregister auth-user-account"));
        success
    }
}

#[cfg(test)]
pub mod test {
    use std::sync::Arc;

    use crate::{
        auth::{
            ticket::kernel::detail::repository::memory::{ticket::MapTicket, StoreTicket},
            user::kernel::detail::repository::memory::{
                login_id::MapLoginId, user::MapUser, StoreLoginId, StoreUser,
            },
        },
        common::api::feature::AsInfra,
    };

    use crate::auth::user::account::unregister::infra::{
        UnregisterAuthUserAccountInfra, UnregisterAuthUserAccountRepository,
    };

    use crate::{
        auth::user::{kernel::data::AuthUserId, login_id::kernel::data::LoginId},
        common::api::repository::data::RepositoryError,
    };

    pub struct MockUnregisterAuthUserAccountInfra {
        repository: MockUnregisterAuthUserAccountRepository,
    }

    impl AsInfra<MockUnregisterAuthUserAccountInfra>
        for (Arc<StoreTicket>, Arc<StoreLoginId>, Arc<StoreUser>)
    {
        fn as_infra(&self) -> MockUnregisterAuthUserAccountInfra {
            MockUnregisterAuthUserAccountInfra {
                repository: MockUnregisterAuthUserAccountRepository {
                    ticket: Arc::clone(&self.0),
                    login_id: Arc::clone(&self.1),
                    user: Arc::clone(&self.2),
                },
            }
        }
    }

    impl UnregisterAuthUserAccountInfra for MockUnregisterAuthUserAccountInfra {
        type Repository = MockUnregisterAuthUserAccountRepository;

        fn repository(&self) -> &Self::Repository {
            &self.repository
        }
    }

    pub struct MockUnregisterAuthUserAccountRepository {
        ticket: Arc<StoreTicket>,
        login_id: Arc<StoreLoginId>,
        user: Arc<StoreUser>,
    }

    #[async_trait::async_trait]
    impl<'a> UnregisterAuthUserAccountRepository for MockUnregisterAuthUserAccountRepository {
        async fn lookup_user_id(
            &self,
            login_id: &LoginId,
        ) -> Result<Option<AuthUserId>, RepositoryError> {
            Ok(MapLoginId::get_user_id(&self.login_id, login_id))
        }

        async fn unregister_user(
            &self,
            user_id: &AuthUserId,
            login_id: &LoginId,
        ) -> Result<(), RepositoryError> {
            MapUser::remove_entry(&self.user, user_id);
            MapLoginId::remove_entry(&self.login_id, login_id);
            Ok(())
        }

        async fn discard_all_ticket(&self, user_id: &AuthUserId) -> Result<(), RepositoryError> {
            for ticket_id in MapTicket::get_all_ticket_id(&self.ticket, user_id) {
                MapTicket::remove_ticket(&self.ticket, &ticket_id)
            }
            Ok(())
        }
    }
}

use sqlx::{query, MySqlPool};

use crate::z_details::_common::repository::mysql::helper::mysql_error;

use crate::auth::password::reset::_api::request_token::infra::ResetTokenDestinationRepository;

use crate::auth::{
    login_id::_api::data::LoginId,
    password::reset::_api::request_token::data::{
        ResetTokenDestination, ResetTokenDestinationExtract,
    },
};
use crate::z_details::_common::repository::data::RepositoryError;

pub struct MysqlResetTokenDestinationRepository<'a> {
    pool: &'a MySqlPool,
}

impl<'a> MysqlResetTokenDestinationRepository<'a> {
    pub const fn new(pool: &'a MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl<'a> ResetTokenDestinationRepository for MysqlResetTokenDestinationRepository<'a> {
    async fn get(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<ResetTokenDestination>, RepositoryError> {
        let conn = self.pool;

        let found = query!(
            r"#####
            select
                email
            from user_password_reset_token_destination
            inner join user on user.user_id = user_password_reset_token_destination.user_id
            where user.login_id = ?
            #####",
            login_id.as_str(),
        )
        .fetch_optional(conn)
        .await
        .map_err(mysql_error)?;

        Ok(found.map(|entry| ResetTokenDestinationExtract { email: entry.email }.into()))
    }
}

#[cfg(test)]
pub mod test {
    use std::{collections::HashMap, sync::Mutex};

    use crate::auth::password::reset::_api::request_token::infra::ResetTokenDestinationRepository;

    use crate::{
        auth::{
            login_id::_api::data::LoginId,
            password::reset::_api::request_token::data::ResetTokenDestination,
        },
        z_details::_common::repository::data::RepositoryError,
    };

    pub type MemoryResetTokenDestinationStore = Mutex<MemoryResetTokenDestinationMap>;
    pub struct MemoryResetTokenDestinationMap(HashMap<String, ResetTokenDestination>);

    impl MemoryResetTokenDestinationMap {
        pub fn new() -> Self {
            Self(HashMap::new())
        }

        pub fn with_destination(login_id: LoginId, destination: ResetTokenDestination) -> Self {
            let mut store = Self::new();
            store.insert(login_id, destination);
            store
        }

        pub fn to_store(self) -> MemoryResetTokenDestinationStore {
            Mutex::new(self)
        }

        fn insert(&mut self, login_id: LoginId, destination: ResetTokenDestination) {
            self.0.insert(login_id.extract(), destination);
        }
        fn get(&self, login_id: &LoginId) -> Option<&ResetTokenDestination> {
            self.0.get(login_id.as_str())
        }
    }

    pub struct MemoryResetTokenDestinationRepository<'a> {
        store: &'a MemoryResetTokenDestinationStore,
    }

    impl<'a> MemoryResetTokenDestinationRepository<'a> {
        pub const fn new(store: &'a MemoryResetTokenDestinationStore) -> Self {
            Self { store }
        }
    }

    #[async_trait::async_trait]
    impl<'a> ResetTokenDestinationRepository for MemoryResetTokenDestinationRepository<'a> {
        async fn get(
            &self,
            login_id: &LoginId,
        ) -> Result<Option<ResetTokenDestination>, RepositoryError> {
            let store = self.store.lock().unwrap();
            Ok(store.get(login_id).map(|destination| destination.clone()))
        }
    }
}

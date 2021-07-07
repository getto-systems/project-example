use std::{collections::HashSet, iter::FromIterator};

use sqlx::{query, MySqlPool};

use crate::z_details::_api::mysql::helper::mysql_error;

use crate::auth::auth_user::_api::kernel::infra::AuthUserRepository;

use crate::auth::auth_user::_api::kernel::data::{AuthUser, AuthUserExtract, AuthUserId};
use crate::z_details::_api::repository::data::RepositoryError;

pub struct MysqlAuthUserRepository<'a> {
    pool: &'a MySqlPool,
}

impl<'a> MysqlAuthUserRepository<'a> {
    pub const fn new(pool: &'a MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl<'a> AuthUserRepository for MysqlAuthUserRepository<'a> {
    async fn get(&self, user_id: &AuthUserId) -> Result<Option<AuthUser>, RepositoryError> {
        // granted roles だけの検索だと、未登録だった場合に不足
        // user の存在を確認して、問題なければ granted roles を合わせて返す
        // group concat を使えば一度に取れるが、データの構築をしないといけない
        // ここではそこまで効率を重視しないので、クエリを２回投げることにする

        let mut conn = self.pool.begin().await.map_err(mysql_error)?;

        let found = query!(
            r"#####
            select
                count(*) as count
            from user
            where user_id = ?
            #####",
            user_id.as_str(),
        )
        .fetch_one(&mut conn)
        .await
        .map_err(mysql_error)?;

        if found.count == 0 {
            return Ok(None);
        }

        let roles = query!(
            r"#####
            select
                role
            from user_granted_role
            where user_id = ?
            #####",
            user_id.as_str(),
        )
        .fetch_all(&mut conn)
        .await
        .map_err(mysql_error)?;

        Ok(Some(
            AuthUserExtract {
                user_id: user_id.as_str().into(),
                granted_roles: HashSet::from_iter(roles.into_iter().map(|entry| entry.role)),
            }
            .into(),
        ))
    }
}

#[cfg(test)]
pub mod test {
    use std::{
        collections::{HashMap, HashSet},
        sync::Mutex,
    };

    use crate::auth::auth_user::_api::kernel::infra::AuthUserRepository;

    use crate::auth::auth_user::_api::kernel::data::{AuthUser, AuthUserExtract, AuthUserId};
    use crate::z_details::_api::repository::data::RepositoryError;

    pub type MemoryAuthUserStore = Mutex<MemoryAuthUserMap>;
    pub struct MemoryAuthUserMap(HashMap<String, HashSet<String>>);

    impl MemoryAuthUserMap {
        pub fn new() -> Self {
            Self(HashMap::new())
        }

        pub fn with_user(user: AuthUser) -> Self {
            let mut store = Self::new();
            store.insert(user);
            store
        }

        pub fn to_store(self) -> MemoryAuthUserStore {
            Mutex::new(self)
        }

        fn insert(&mut self, user: AuthUser) {
            let user = user.extract();
            self.0.insert(user.user_id, user.granted_roles);
        }
        fn get(&self, user_id: &AuthUserId) -> Option<&HashSet<String>> {
            self.0.get(user_id.as_str())
        }
    }

    pub struct MemoryAuthUserRepository<'a> {
        store: &'a MemoryAuthUserStore,
    }

    impl<'a> MemoryAuthUserRepository<'a> {
        pub const fn new(store: &'a MemoryAuthUserStore) -> Self {
            Self { store }
        }
    }

    #[async_trait::async_trait]
    impl<'a> AuthUserRepository for MemoryAuthUserRepository<'a> {
        async fn get(&self, user_id: &AuthUserId) -> Result<Option<AuthUser>, RepositoryError> {
            let store = self.store.lock().unwrap();
            Ok(store.get(user_id).map(|granted_roles| {
                AuthUserExtract {
                    user_id: user_id.as_str().into(),
                    granted_roles: granted_roles.clone(),
                }
                .into()
            }))
        }
    }
}

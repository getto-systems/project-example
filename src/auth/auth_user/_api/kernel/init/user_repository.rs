use std::{collections::HashSet, iter::FromIterator};

use mysql::{params, prelude::Queryable, Pool};

use crate::z_details::_api::{
    mysql::helper::{mysql_error, read_only_transaction},
    repository::helper::infra_error,
};

use crate::auth::auth_user::_api::kernel::infra::AuthUserRepository;

use crate::auth::auth_user::_api::kernel::data::{AuthUser, AuthUserExtract, AuthUserId};
use crate::z_details::_api::repository::data::RepositoryError;

pub struct MysqlAuthUserRepository<'a> {
    pool: &'a Pool,
}

impl<'a> MysqlAuthUserRepository<'a> {
    pub const fn new(pool: &'a Pool) -> Self {
        Self { pool }
    }
}

impl<'a> AuthUserRepository for MysqlAuthUserRepository<'a> {
    fn get(&self, user_id: &AuthUserId) -> Result<Option<AuthUser>, RepositoryError> {
        let mut conn = self.pool.get_conn().map_err(mysql_error)?;
        let mut conn = conn
            .start_transaction(read_only_transaction())
            .map_err(mysql_error)?;

        // granted roles だけの検索だと、未登録だった場合に不足
        // user の存在を確認して、問題なければ granted roles を合わせて返す
        // group concat を使えば一度に取れるが、データの構築をしないといけない
        // ここではそこまで効率を重視しないので、クエリを２回投げることにする

        let count: u8 = conn
            .exec_first(
                r"#####
                select count(*)
                from user
                where user_id = :user_id
                #####",
                params! {
                    "user_id" => user_id.as_str(),
                },
            )
            .map_err(mysql_error)?
            .ok_or(infra_error("failed to count user"))?;

        if count == 0 {
            return Ok(None);
        }

        let found: Vec<String> = conn
            .exec(
                r"#####
                select role from user_granted_role
                where user_id = :user_id
                #####",
                params! {
                    "user_id" => user_id.as_str(),
                },
            )
            .map_err(mysql_error)?;

        Ok(Some(
            AuthUserExtract {
                user_id: user_id.as_str().into(),
                granted_roles: HashSet::from_iter(found),
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

    impl<'a> AuthUserRepository for MemoryAuthUserRepository<'a> {
        fn get(&self, user_id: &AuthUserId) -> Result<Option<AuthUser>, RepositoryError> {
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

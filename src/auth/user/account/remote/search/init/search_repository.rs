use sea_query::{Expr, Func, Iden, MysqlQueryBuilder, Order, Query};
use sqlx::{query_as, MySqlPool};

sea_query::sea_query_driver_mysql!();
use sea_query_driver_mysql::bind_query_as;

use crate::z_lib::remote::repository::mysql::helper::mysql_error;

use crate::auth::user::account::remote::search::infra::{
    SearchUserAccountFields, SearchUserAccountRepository,
};

use crate::{
    auth::user::{
        account::remote::search::data::{SearchUserAccountBasket, UserAccountBasket},
        login_id::remote::data::LoginIdBasket,
        remote::kernel::data::GrantedAuthRolesBasket,
    },
    z_lib::remote::{
        repository::data::RepositoryError,
        search::data::{SearchOffsetDetecter, SearchPage, SearchSortOrderMap},
    },
};

pub struct MysqlSearchUserAccountRepository<'a> {
    pool: &'a MySqlPool,
}

impl<'a> MysqlSearchUserAccountRepository<'a> {
    pub const fn new(pool: &'a MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl<'a> SearchUserAccountRepository for MysqlSearchUserAccountRepository<'a> {
    async fn search(
        &self,
        fields: &SearchUserAccountFields,
    ) -> Result<SearchUserAccountBasket, RepositoryError> {
        search(&self, fields).await
    }
}

#[derive(Iden)]
enum User {
    Table,
    UserId,
    LoginId,
}

#[derive(Iden)]
enum UserGrantedRole {
    Table,
    UserId,
}

#[derive(sqlx::FromRow)]
struct Count {
    count: u32,
}

#[derive(sqlx::FromRow)]
struct UserAccount {
    login_id: String,
    granted_roles: String,
}

async fn search<'a>(
    repository: &MysqlSearchUserAccountRepository<'a>,
    fields: &SearchUserAccountFields,
) -> Result<SearchUserAccountBasket, RepositoryError> {
    let mut conn = repository.pool.begin().await.map_err(mysql_error)?;

    let (sort_col, sort_order) = fields
        .sort()
        .detect(vec![(
            "login-id",
            User::LoginId,
            SearchSortOrderMap {
                normal: Order::Asc,
                reverse: Order::Desc,
            },
        )])
        .unwrap_or((User::LoginId, Order::Asc));

    let (sql, values) = Query::select()
        .from(User::Table)
        .expr(Func::count(Expr::col(User::UserId)))
        .conditions(
            fields.login_id().is_empty(),
            |_q| {},
            |q| {
                q.and_where(Expr::col(User::LoginId).eq(fields.login_id().to_owned()));
            },
        )
        .build(MysqlQueryBuilder);

    let all = bind_query_as(query_as::<_, Count>(&sql), &values)
        .fetch_one(&mut conn)
        .await
        .map_err(mysql_error)?
        .count;

    let limit = 1000;
    let offset = SearchOffsetDetecter { all, limit }.detect(fields.offset());

    let (sql, values) = Query::select()
        .column(User::LoginId)
        .expr(Expr::cust(
            "group_concat(user_granted_role.role) as granted_roles",
        ))
        .from(User::Table)
        .inner_join(
            UserGrantedRole::Table,
            Expr::tbl(User::Table, User::UserId)
                .equals(UserGrantedRole::Table, UserGrantedRole::UserId),
        )
        .conditions(
            fields.login_id().is_empty(),
            |_q| {},
            |q| {
                q.and_where(Expr::col(User::LoginId).eq(fields.login_id().to_owned()));
            },
        )
        .offset(offset.into())
        .limit(limit.into())
        .order_by(sort_col, sort_order)
        .build(MysqlQueryBuilder);

    let rows = bind_query_as(query_as::<_, UserAccount>(&sql), &values)
        .fetch_all(&mut conn)
        .await
        .map_err(mysql_error)?;

    Ok(SearchUserAccountBasket {
        page: SearchPage { all, limit, offset },
        users: rows
            .into_iter()
            .map(|row| UserAccountBasket {
                login_id: LoginIdBasket::new(row.login_id),
                granted_roles: GrantedAuthRolesBasket::new(
                    row.granted_roles
                        .split(",")
                        .into_iter()
                        .map(|str| str.to_string())
                        .collect(),
                ),
            })
            .collect(),
    })
}

#[cfg(test)]
pub mod test {
    use std::sync::Mutex;

    use crate::auth::user::{
        password::remote::kernel::init::password_repository::test::MemoryAuthUserPasswordMap,
        remote::kernel::init::user_repository::test::MemoryAuthUserMap,
    };

    use crate::auth::user::account::remote::search::infra::{
        SearchUserAccountFields, SearchUserAccountRepository,
    };

    use crate::{
        auth::user::{
            account::remote::search::data::{SearchUserAccountBasket, UserAccountBasket},
            login_id::remote::data::LoginIdBasket,
            remote::kernel::data::GrantedAuthRolesBasket,
        },
        z_lib::remote::{repository::data::RepositoryError, search::data::SearchPage},
    };

    pub type MemorySearchUserAccountStore = Mutex<MemorySearchUserAccountMap>;
    pub struct MemorySearchUserAccountMap {
        pub user: MemoryAuthUserMap,
        pub password: MemoryAuthUserPasswordMap,
    }

    impl MemorySearchUserAccountMap {
        pub fn to_store(self) -> MemorySearchUserAccountStore {
            Mutex::new(self)
        }
    }

    pub struct MemorySearchUserAccountRepository<'a> {
        store: &'a MemorySearchUserAccountStore,
    }

    impl<'a> MemorySearchUserAccountRepository<'a> {
        pub const fn new(store: &'a MemorySearchUserAccountStore) -> Self {
            Self { store }
        }
    }

    #[async_trait::async_trait]
    impl<'a> SearchUserAccountRepository for MemorySearchUserAccountRepository<'a> {
        async fn search(
            &self,
            fields: &SearchUserAccountFields,
        ) -> Result<SearchUserAccountBasket, RepositoryError> {
            search(&self, fields).await
        }
    }

    async fn search<'a>(
        repository: &MemorySearchUserAccountRepository<'a>,
        _fields: &SearchUserAccountFields,
    ) -> Result<SearchUserAccountBasket, RepositoryError> {
        let store = repository.store.lock().unwrap();
        let users = store
            .user
            .all()
            .into_iter()
            // 実際のデータベースでは fields を使用して検索を行う
            .filter_map(|user| {
                store
                    .password
                    .get_login_id(&user.user_id)
                    .map(|login_id| UserAccountBasket {
                        login_id: LoginIdBasket::new(login_id.clone().extract()),
                        granted_roles: GrantedAuthRolesBasket::new(
                            user.granted_roles.into_iter().collect(),
                        ),
                    })
            })
            .collect();

        Ok(SearchUserAccountBasket {
            page: SearchPage {
                offset: 0,
                limit: 0,
                all: 0,
            },
            users: users,
        })
    }
}

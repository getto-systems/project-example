use std::convert::TryInto;

use sea_query::{Expr, Iden, MysqlQueryBuilder, Order, Query, SelectStatement};
use sqlx::{query_as, MySqlPool};

sea_query::sea_query_driver_mysql!();
use sea_query_driver_mysql::bind_query_as;

use crate::z_lib::api::repository::mysql::helper::mysql_error;

use crate::auth::user::account::search::api::infra::{
    SearchAuthUserAccountFields, SearchAuthUserAccountRepository,
};

use crate::{
    auth::user::{
        account::search::api::data::{AuthUserAccountBasket, SearchAuthUserAccountBasket},
        kernel::data::GrantedAuthRolesBasket,
        login_id::kernel::data::LoginIdBasket,
    },
    z_lib::api::{
        repository::data::RepositoryError,
        search::data::{SearchOffsetDetecter, SearchPage, SearchSortOrderMap},
    },
};

pub struct MysqlSearchAuthUserAccountRepository<'a> {
    pool: &'a MySqlPool,
}

impl<'a> MysqlSearchAuthUserAccountRepository<'a> {
    pub const fn new(pool: &'a MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl<'a> SearchAuthUserAccountRepository for MysqlSearchAuthUserAccountRepository<'a> {
    async fn search(
        &self,
        fields: &SearchAuthUserAccountFields,
    ) -> Result<SearchAuthUserAccountBasket, RepositoryError> {
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
    count: i64,
}

#[derive(sqlx::FromRow)]
struct AuthUserAccount {
    login_id: String,
    granted_roles: String,
}

async fn search<'a>(
    repository: &MysqlSearchAuthUserAccountRepository<'a>,
    fields: &SearchAuthUserAccountFields,
) -> Result<SearchAuthUserAccountBasket, RepositoryError> {
    let mut conn = repository.pool.begin().await.map_err(mysql_error)?;

    let (sql, values) = search_conditions(
        Query::select()
            .from(User::Table)
            .expr(Expr::cust("count(user.user_id) as count")),
        fields,
    )
    .build(MysqlQueryBuilder);

    let all = bind_query_as(query_as::<_, Count>(&sql), &values)
        .fetch_one(&mut conn)
        .await
        .map_err(mysql_error)?
        .count;

    let limit = 1000;

    if all == 0 {
        return Ok(SearchAuthUserAccountBasket {
            page: SearchPage {
                // i64 -> i32 に変換; それほど大きな値にならないはず
                all: all.try_into().unwrap(),
                limit: limit.try_into().unwrap(),
                offset: 0,
            },
            users: vec![],
        });
    }

    let offset = SearchOffsetDetecter {
        // i64 -> u64; count() はマイナスにならないので、unwrap に失敗しない
        all: all.try_into().unwrap(),
        limit,
    }
    .detect(fields.offset());

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

    let (sql, values) = search_conditions(
        Query::select()
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
            .offset(offset)
            .limit(limit)
            .order_by(sort_col, sort_order),
        fields,
    )
    .build(MysqlQueryBuilder);

    let rows = bind_query_as(query_as::<_, AuthUserAccount>(&sql), &values)
        .fetch_all(&mut conn)
        .await
        .map_err(mysql_error)?;

    Ok(SearchAuthUserAccountBasket {
        page: SearchPage {
            // i64 -> i32 に変換; それほど大きな値にならないはず
            all: all.try_into().unwrap(),
            limit: limit.try_into().unwrap(),
            offset: offset.try_into().unwrap(),
        },
        users: rows
            .into_iter()
            .map(|row| AuthUserAccountBasket {
                login_id: LoginIdBasket::new(row.login_id),
                granted_roles: GrantedAuthRolesBasket::new(
                    row.granted_roles
                        .split(",")
                        .into_iter()
                        .map(|str| str.to_owned())
                        .collect(),
                ),
            })
            .collect(),
    })
}

fn search_conditions<'a>(
    query: &'a mut SelectStatement,
    fields: &SearchAuthUserAccountFields,
) -> &'a mut SelectStatement {
    query.conditions(
        fields.login_id().is_empty(),
        |_q| {},
        |q| {
            q.and_where(Expr::col(User::LoginId).eq(fields.login_id().to_owned()));
        },
    )
}

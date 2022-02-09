use std::{collections::HashSet, convert::TryInto, iter::FromIterator};

use chrono::{TimeZone, Utc};
use sqlx::{query, query_as, MySql, MySqlPool, Transaction};

use sea_query::{Expr, Iden, MysqlQueryBuilder, Order, Query, SelectStatement};
sea_query::sea_query_driver_mysql!();
use sea_query_driver_mysql::bind_query_as;

use crate::z_lib::repository::{helper::infra_error, mysql::helper::mysql_error};

use crate::auth::user::{
    account::search::infra::{SearchAuthUserAccountFields, SearchAuthUserAccountRepository},
    kernel::infra::AuthUserRepository,
    password::{
        authenticate::infra::VerifyPasswordRepository,
        change::infra::ChangePasswordRepository,
        kernel::infra::{AuthUserPasswordHasher, AuthUserPasswordMatcher, HashedPassword},
        reset::{
            kernel::infra::{ResetTokenEntry, ResetTokenEntryExtract},
            request_token::infra::{RegisterResetTokenRepository, ResetTokenDestinationRepository},
            reset::infra::ResetPasswordRepository,
        },
    },
};

use crate::{
    auth::{
        ticket::kernel::data::{AuthDateTime, ExpireDateTime},
        user::{
            account::search::data::{AuthUserAccountBasket, SearchAuthUserAccountBasket},
            kernel::data::{AuthUser, AuthUserExtract, AuthUserId, GrantedAuthRolesBasket},
            login_id::kernel::data::{LoginId, LoginIdBasket},
            password::{
                authenticate::data::VerifyPasswordRepositoryError,
                change::data::ChangePasswordRepositoryError,
                reset::{
                    kernel::data::{
                        ResetToken, ResetTokenDestination, ResetTokenDestinationExtract,
                    },
                    request_token::data::RegisterResetTokenRepositoryError,
                    reset::data::ResetPasswordRepositoryError,
                },
            },
        },
    },
    z_lib::{
        repository::data::RepositoryError,
        search::data::{SearchOffset, SearchPage, SearchSortOrderMap},
    },
};

pub struct MysqlAuthUserRepository<'pool> {
    pool: &'pool MySqlPool,
}

impl<'pool> MysqlAuthUserRepository<'pool> {
    pub const fn new(pool: &'pool MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl<'pool> AuthUserRepository for MysqlAuthUserRepository<'pool> {
    async fn get(&self, user_id: &AuthUserId) -> Result<Option<AuthUser>, RepositoryError> {
        get_user(self, user_id).await
    }
}
async fn get_user<'pool>(
    repository: &MysqlAuthUserRepository<'pool>,
    user_id: &AuthUserId,
) -> Result<Option<AuthUser>, RepositoryError> {
    // granted roles だけの検索だと、未登録だった場合に不足
    // user の存在を確認して、問題なければ granted roles を合わせて返す
    // group concat を使えば一度に取れるが、データの構築をしないといけない
    // ここでは効率を重視せずに、クエリを 2回投げることにする

    let mut conn = repository.pool.begin().await.map_err(mysql_error)?;

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
        .restore(),
    ))
}

#[async_trait::async_trait]
impl<'pool> VerifyPasswordRepository for MysqlAuthUserRepository<'pool> {
    async fn verify_password<'a>(
        &self,
        login_id: &'a LoginId,
        matcher: impl 'a + AuthUserPasswordMatcher,
    ) -> Result<AuthUserId, VerifyPasswordRepositoryError> {
        verify_password(self, login_id, matcher).await
    }
}
async fn verify_password<'pool, 'a>(
    repository: &MysqlAuthUserRepository<'pool>,
    login_id: &'a LoginId,
    matcher: impl 'a + AuthUserPasswordMatcher,
) -> Result<AuthUserId, VerifyPasswordRepositoryError> {
    let conn = repository.pool;

    let found = query!(
        r"#####
        select
            user.user_id,
            hashed_password
        from user_password
        inner join user on user_password.user_id = user.user_id
        where user.login_id = ?
        #####",
        login_id.as_str(),
    )
    .fetch_optional(conn)
    .await
    .map_err(verify_password_error)?
    .ok_or(VerifyPasswordRepositoryError::PasswordNotFound)?;

    let matched = matcher
        .match_password(&HashedPassword::restore(found.hashed_password))
        .map_err(VerifyPasswordRepositoryError::PasswordHashError)?;

    if !matched {
        return Err(VerifyPasswordRepositoryError::PasswordNotMatched);
    }

    Ok(AuthUserId::restore(found.user_id))
}
fn verify_password_error(err: sqlx::Error) -> VerifyPasswordRepositoryError {
    VerifyPasswordRepositoryError::RepositoryError(mysql_error(err))
}

#[async_trait::async_trait]
impl<'pool> ChangePasswordRepository for MysqlAuthUserRepository<'pool> {
    async fn change_password<'a>(
        &self,
        user_id: &'a AuthUserId,
        matcher: impl 'a + AuthUserPasswordMatcher,
        hasher: impl 'a + AuthUserPasswordHasher,
    ) -> Result<(), ChangePasswordRepositoryError> {
        change_password(self, user_id, matcher, hasher).await
    }
}
async fn change_password<'pool, 'a>(
    repository: &MysqlAuthUserRepository<'pool>,
    user_id: &'a AuthUserId,
    matcher: impl 'a + AuthUserPasswordMatcher,
    hasher: impl 'a + AuthUserPasswordHasher,
) -> Result<(), ChangePasswordRepositoryError> {
    let mut conn = repository
        .pool
        .begin()
        .await
        .map_err(change_password_error)?;

    let found = query!(
        r"#####
        select
            hashed_password
        from user_password
        where user_id = ?
        #####",
        user_id.as_str(),
    )
    .fetch_optional(&mut conn)
    .await
    .map_err(change_password_error)?
    .ok_or(ChangePasswordRepositoryError::PasswordNotFound)?;

    let matched = matcher
        .match_password(&HashedPassword::restore(found.hashed_password))
        .map_err(ChangePasswordRepositoryError::PasswordHashError)?;

    if !matched {
        return Err(ChangePasswordRepositoryError::PasswordNotMatched);
    }

    let hashed_password = hasher
        .hash_password()
        .map_err(ChangePasswordRepositoryError::PasswordHashError)?;

    let conn = update_password(conn, user_id.as_str(), &hashed_password.extract())
        .await
        .map_err(change_password_error)?;

    conn.commit().await.map_err(change_password_error)?;

    Ok(())
}
fn change_password_error(err: sqlx::Error) -> ChangePasswordRepositoryError {
    ChangePasswordRepositoryError::RepositoryError(mysql_error(err))
}

#[async_trait::async_trait]
impl<'pool> RegisterResetTokenRepository for MysqlAuthUserRepository<'pool> {
    async fn register_reset_token(
        &self,
        reset_token: ResetToken,
        login_id: LoginId,
        destination: ResetTokenDestination,
        expires: ExpireDateTime,
        requested_at: AuthDateTime,
    ) -> Result<(), RegisterResetTokenRepositoryError> {
        register_reset_token(
            self,
            login_id,
            reset_token,
            destination,
            expires,
            requested_at,
        )
        .await
    }
}
async fn register_reset_token<'pool>(
    repository: &MysqlAuthUserRepository<'pool>,
    login_id: LoginId,
    reset_token: ResetToken,
    destination: ResetTokenDestination,
    expires: ExpireDateTime,
    requested_at: AuthDateTime,
) -> Result<(), RegisterResetTokenRepositoryError> {
    let mut conn = repository
        .pool
        .begin()
        .await
        .map_err(request_reset_token_error)?;

    let found = query!(
        r"#####
        select
            user_id
        from user
        where login_id = ?
        #####",
        login_id.as_str(),
    )
    .fetch_optional(&mut conn)
    .await
    .map_err(request_reset_token_error)?
    .ok_or(RegisterResetTokenRepositoryError::UserNotFound)?;

    query!(
        r"#####
        insert into user_password_reset_token
            (user_id, reset_token, login_id, expires, requested_at)
        values
            (?, ?, ?, ?, ?)
        #####",
        found.user_id,
        reset_token.as_str(),
        login_id.extract(),
        expires.extract().naive_utc(),
        requested_at.extract().naive_utc(),
    )
    .execute(&mut conn)
    .await
    .map_err(request_reset_token_error)?;

    query!(
        r"#####
        insert into user_password_reset_token_registered_destination
            (user_id, reset_token, email)
        values
            (?, ?, ?)
        #####",
        found.user_id,
        reset_token.as_str(),
        destination.into_email(),
    )
    .execute(&mut conn)
    .await
    .map_err(request_reset_token_error)?;

    conn.commit().await.map_err(request_reset_token_error)?;

    Ok(())
}
fn request_reset_token_error(err: sqlx::Error) -> RegisterResetTokenRepositoryError {
    RegisterResetTokenRepositoryError::RepositoryError(mysql_error(err))
}

#[async_trait::async_trait]
impl<'pool> ResetPasswordRepository for MysqlAuthUserRepository<'pool> {
    async fn reset_token_entry(
        &self,
        reset_token: &ResetToken,
    ) -> Result<Option<ResetTokenEntry>, RepositoryError> {
        reset_token_entry(self, reset_token).await
    }
    async fn reset_password<'a>(
        &self,
        reset_token: &'a ResetToken,
        hasher: impl 'a + AuthUserPasswordHasher,
        reset_at: AuthDateTime,
    ) -> Result<AuthUserId, ResetPasswordRepositoryError> {
        reset_password(self, reset_token, hasher, reset_at).await
    }
}
async fn reset_token_entry<'pool>(
    repository: &MysqlAuthUserRepository<'pool>,
    reset_token: &ResetToken,
) -> Result<Option<ResetTokenEntry>, RepositoryError> {
    let conn = repository.pool;

    let found = query!(
        r"#####
        select
            login_id,
            expires,
            reset_at
        from user_password_reset_token
        where reset_token = ?
        #####",
        reset_token.as_str(),
    )
    .fetch_optional(conn)
    .await
    .map_err(mysql_error)?;

    let destination = query!(
        r"#####
        select
            email
        from user_password_reset_token_registered_destination
        where reset_token = ?
        #####",
        reset_token.as_str(),
    )
    .fetch_optional(conn)
    .await
    .map_err(mysql_error)?;

    match (found, destination) {
        (Some(entry), Some(destination)) => Ok(Some(
            ResetTokenEntryExtract {
                login_id: entry.login_id,
                destination: ResetTokenDestinationExtract {
                    email: destination.email,
                },
                expires: Utc.from_utc_datetime(&entry.expires),
                reset_at: entry
                    .reset_at
                    .map(|reset_at| Utc.from_utc_datetime(&reset_at)),
            }
            .restore(),
        )),
        _ => Ok(None),
    }
}
async fn reset_password<'pool, 'a>(
    repository: &MysqlAuthUserRepository<'pool>,
    reset_token: &'a ResetToken,
    hasher: impl 'a + AuthUserPasswordHasher,
    reset_at: AuthDateTime,
) -> Result<AuthUserId, ResetPasswordRepositoryError> {
    // reset_token が正しいことが前提; reset_token_entry() で事前に確認する

    let mut conn = repository
        .pool
        .begin()
        .await
        .map_err(reset_password_error)?;

    let found = query!(
        r"#####
        select
            user_id
        from user_password_reset_token
        where reset_token = ?
        #####",
        reset_token.as_str(),
    )
    .fetch_optional(&mut conn)
    .await
    .map_err(reset_password_error)?
    .ok_or(reset_password_infra_error("reset token not found"))?;

    query!(
        r"#####
        update user_password_reset_token
        set reset_at = ?
        where user_id = ?
        #####",
        reset_at.extract().naive_utc(),
        &found.user_id,
    )
    .execute(&mut conn)
    .await
    .map_err(reset_password_error)?;

    let hashed_password = hasher
        .hash_password()
        .map_err(ResetPasswordRepositoryError::PasswordHashError)?;

    let conn = update_password(conn, &found.user_id, &hashed_password.extract())
        .await
        .map_err(reset_password_error)?;

    conn.commit().await.map_err(reset_password_error)?;

    Ok(AuthUserId::restore(found.user_id))
}
fn reset_password_error(err: sqlx::Error) -> ResetPasswordRepositoryError {
    ResetPasswordRepositoryError::RepositoryError(mysql_error(err))
}
fn reset_password_infra_error(err: impl std::fmt::Display) -> ResetPasswordRepositoryError {
    ResetPasswordRepositoryError::RepositoryError(infra_error(err))
}

async fn update_password<'pool>(
    mut conn: Transaction<'pool, MySql>,
    user_id: &str,
    hashed_password: &str,
) -> Result<Transaction<'pool, MySql>, sqlx::Error> {
    query!(
        r"#####
        delete from user_password
        where user_id = ?
        #####",
        user_id,
    )
    .execute(&mut conn)
    .await?;

    query!(
        r"#####
        insert into user_password
            (user_id, hashed_password)
        values
            (?, ?)
        #####",
        user_id,
        hashed_password,
    )
    .execute(&mut conn)
    .await?;

    Ok(conn)
}

#[async_trait::async_trait]
impl<'a> ResetTokenDestinationRepository for MysqlAuthUserRepository<'a> {
    async fn get(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<ResetTokenDestination>, RepositoryError> {
        get_destination(self, login_id).await
    }
}
async fn get_destination<'pool>(
    repository: &MysqlAuthUserRepository<'pool>,
    login_id: &LoginId,
) -> Result<Option<ResetTokenDestination>, RepositoryError> {
    let conn = repository.pool;

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

    Ok(found.map(|entry| ResetTokenDestinationExtract { email: entry.email }.restore()))
}

#[async_trait::async_trait]
impl<'a> SearchAuthUserAccountRepository for MysqlAuthUserRepository<'a> {
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

async fn search<'pool>(
    repository: &MysqlAuthUserRepository<'pool>,
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
    let all: i32 = all.try_into().map_err(infra_error)?;

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

    let offset = SearchOffset { all, limit }.detect(fields.offset());

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
            .offset(offset.try_into().map_err(infra_error)?)
            .limit(limit.try_into().map_err(infra_error)?)
            .order_by(sort_col, sort_order),
        fields,
    )
    .build(MysqlQueryBuilder);

    let rows = bind_query_as(query_as::<_, AuthUserAccount>(&sql), &values)
        .fetch_all(&mut conn)
        .await
        .map_err(mysql_error)?;

    Ok(SearchAuthUserAccountBasket {
        page: SearchPage { all, limit, offset },
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

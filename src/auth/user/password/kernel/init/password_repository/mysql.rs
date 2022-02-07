use chrono::{TimeZone, Utc};
use sqlx::{query, MySql, MySqlPool, Transaction};

use crate::z_lib::api::repository::{helper::infra_error, mysql::helper::mysql_error};

use crate::auth::user::password::{
    authenticate::api::infra::VerifyPasswordRepository,
    change::api::infra::ChangePasswordRepository,
    kernel::infra::{AuthUserPasswordHasher, AuthUserPasswordMatcher, HashedPassword},
    reset::{
        kernel::infra::{ResetTokenEntry, ResetTokenEntryExtract},
        request_token::api::infra::RegisterResetTokenRepository,
        reset::api::infra::ResetPasswordRepository,
    },
};

use crate::{
    auth::{
        ticket::kernel::api::data::{AuthDateTime, ExpireDateTime},
        user::{
            kernel::data::AuthUserId,
            login_id::kernel::data::LoginId,
            password::{
                authenticate::api::data::VerifyPasswordRepositoryError,
                change::api::data::ChangePasswordRepositoryError,
                reset::{
                    kernel::data::{
                        ResetToken, ResetTokenDestination, ResetTokenDestinationExtract,
                    },
                    request_token::api::data::RegisterResetTokenRepositoryError,
                    reset::api::data::ResetPasswordRepositoryError,
                },
            },
        },
    },
    z_lib::api::repository::data::RepositoryError,
};

pub struct MysqlAuthUserPasswordRepository<'a> {
    pool: &'a MySqlPool,
}

impl<'a> MysqlAuthUserPasswordRepository<'a> {
    pub const fn new(pool: &'a MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl<'pool> VerifyPasswordRepository for MysqlAuthUserPasswordRepository<'pool> {
    async fn verify_password<'a>(
        &self,
        login_id: &'a LoginId,
        matcher: impl 'a + AuthUserPasswordMatcher,
    ) -> Result<AuthUserId, VerifyPasswordRepositoryError> {
        let conn = self.pool;

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
}
fn verify_password_error(err: sqlx::Error) -> VerifyPasswordRepositoryError {
    VerifyPasswordRepositoryError::RepositoryError(mysql_error(err))
}

#[async_trait::async_trait]
impl<'pool> ChangePasswordRepository for MysqlAuthUserPasswordRepository<'pool> {
    async fn change_password<'a>(
        &self,
        user_id: &'a AuthUserId,
        matcher: impl 'a + AuthUserPasswordMatcher,
        hasher: impl 'a + AuthUserPasswordHasher,
    ) -> Result<(), ChangePasswordRepositoryError> {
        let mut conn = self.pool.begin().await.map_err(change_password_error)?;

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
}
fn change_password_error(err: sqlx::Error) -> ChangePasswordRepositoryError {
    ChangePasswordRepositoryError::RepositoryError(mysql_error(err))
}

#[async_trait::async_trait]
impl<'pool> RegisterResetTokenRepository for MysqlAuthUserPasswordRepository<'pool> {
    async fn register_reset_token(
        &self,
        login_id: LoginId,
        reset_token: ResetToken,
        destination: ResetTokenDestination,
        expires: ExpireDateTime,
        requested_at: AuthDateTime,
    ) -> Result<(), RegisterResetTokenRepositoryError> {
        let mut conn = self.pool.begin().await.map_err(request_reset_token_error)?;

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
}
fn request_reset_token_error(err: sqlx::Error) -> RegisterResetTokenRepositoryError {
    RegisterResetTokenRepositoryError::RepositoryError(mysql_error(err))
}

#[async_trait::async_trait]
impl<'pool> ResetPasswordRepository for MysqlAuthUserPasswordRepository<'pool> {
    async fn reset_token_entry(
        &self,
        reset_token: &ResetToken,
    ) -> Result<Option<ResetTokenEntry>, RepositoryError> {
        let conn = self.pool;

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

    async fn reset_password<'a>(
        &self,
        reset_token: &'a ResetToken,
        hasher: impl 'a + AuthUserPasswordHasher,
        reset_at: AuthDateTime,
    ) -> Result<AuthUserId, ResetPasswordRepositoryError> {
        // reset_token が正しいことが前提; reset_token_entry() で事前に確認する

        let mut conn = self.pool.begin().await.map_err(reset_password_error)?;

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
}
fn reset_password_error(err: sqlx::Error) -> ResetPasswordRepositoryError {
    ResetPasswordRepositoryError::RepositoryError(mysql_error(err))
}
fn reset_password_infra_error(err: impl std::fmt::Display) -> ResetPasswordRepositoryError {
    ResetPasswordRepositoryError::RepositoryError(infra_error(err))
}

async fn update_password<'a>(
    mut conn: Transaction<'a, MySql>,
    user_id: &str,
    hashed_password: &str,
) -> Result<Transaction<'a, MySql>, sqlx::Error> {
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

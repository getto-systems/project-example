use std::fmt::Display;

use chrono::{TimeZone, Utc};
use sqlx::{query, MySqlPool};

use crate::z_details::_api::mysql::helper::mysql_error;

use crate::auth::password::_api::kernel::infra::{
    AuthUserPasswordHasher, AuthUserPasswordMatcher, AuthUserPasswordRepository, HashedPassword,
    RequestResetTokenError, ResetPasswordError, ResetTokenEntry, ResetTokenEntryExtract,
    VerifyPasswordError,
};

use crate::auth::{
    auth_ticket::_api::kernel::data::{AuthDateTime, ExpireDateTime},
    auth_user::_api::kernel::data::AuthUserId,
    login_id::_api::data::LoginId,
    password::_api::kernel::data::ResetToken,
};
use crate::z_details::_api::repository::data::RepositoryError;
use crate::z_details::_api::repository::helper::infra_error;

pub struct MysqlAuthUserPasswordRepository<'a> {
    pool: &'a MySqlPool,
}

impl<'a> MysqlAuthUserPasswordRepository<'a> {
    pub const fn new(pool: &'a MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl<'pool> AuthUserPasswordRepository for MysqlAuthUserPasswordRepository<'pool> {
    async fn verify_password<'a>(
        &self,
        login_id: &'a LoginId,
        matcher: impl AuthUserPasswordMatcher + 'a,
    ) -> Result<AuthUserId, VerifyPasswordError> {
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
        .map_err(verify_mysql_error)?
        .ok_or(VerifyPasswordError::PasswordNotFound)?;

        let is_matched = matcher
            .match_password(&HashedPassword::restore(found.hashed_password))
            .map_err(VerifyPasswordError::PasswordHashError)?;

        if is_matched {
            Ok(AuthUserId::restore(found.user_id))
        } else {
            Err(VerifyPasswordError::PasswordNotMatched)
        }
    }

    async fn request_reset_token(
        &self,
        reset_token: ResetToken,
        login_id: LoginId,
        expires: ExpireDateTime,
        requested_at: AuthDateTime,
    ) -> Result<(), RequestResetTokenError> {
        let mut conn = self.pool.begin().await.map_err(request_mysql_error)?;

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
        .map_err(request_mysql_error)?
        .ok_or(RequestResetTokenError::NotFound)?;

        query!(
            r"#####
            insert into user_password_reset_token
                (user_id, reset_token, login_id, expires, requested_at)
            values
                (?, ?, ?, ?, ?)
            #####",
            found.user_id,
            reset_token.extract(),
            login_id.extract(),
            expires.extract(),
            requested_at.extract(),
        )
        .execute(&mut conn)
        .await
        .map_err(request_mysql_error)?;

        conn.commit().await.map_err(request_mysql_error)?;

        Ok(())
    }

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

        Ok(found.map(|entry| {
            ResetTokenEntryExtract {
                login_id: entry.login_id,
                expires: Utc.from_utc_datetime(&entry.expires),
                reset_at: entry
                    .reset_at
                    .map(|reset_at| Utc.from_utc_datetime(&reset_at)),
            }
            .into()
        }))
    }

    async fn reset_password<'a>(
        &self,
        reset_token: &'a ResetToken,
        hasher: impl AuthUserPasswordHasher + 'a,
        reset_at: AuthDateTime,
    ) -> Result<AuthUserId, ResetPasswordError> {
        // reset_token が正しいことが前提; reset_token_entry() で事前に確認する

        let mut conn = self.pool.begin().await.map_err(reset_mysql_error)?;

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
        .map_err(reset_mysql_error)?
        .ok_or(reset_infra_error("reset token not found"))?;

        query!(
            r"#####
            update user_password_reset_token
            set reset_at = ?
            where user_id = ?
            #####",
            reset_at.extract(),
            &found.user_id,
        )
        .execute(&mut conn)
        .await
        .map_err(reset_mysql_error)?;

        let hashed_password = hasher
            .hash_password()
            .map_err(ResetPasswordError::PasswordHashError)?;

        query!(
            r"#####
            delete from user_password
            where user_id = ?
            #####",
            &found.user_id,
        )
        .execute(&mut conn)
        .await
        .map_err(reset_mysql_error)?;

        query!(
            r"#####
            insert into user_password
                (user_id, hashed_password)
            values
                (?, ?)
            #####",
            &found.user_id,
            hashed_password.extract(),
        )
        .execute(&mut conn)
        .await
        .map_err(reset_mysql_error)?;

        conn.commit().await.map_err(reset_mysql_error)?;

        Ok(AuthUserId::restore(found.user_id))
    }
}

fn verify_mysql_error(err: sqlx::Error) -> VerifyPasswordError {
    VerifyPasswordError::RepositoryError(mysql_error(err))
}
fn request_mysql_error(err: sqlx::Error) -> RequestResetTokenError {
    RequestResetTokenError::RepositoryError(mysql_error(err))
}
fn reset_mysql_error(err: sqlx::Error) -> ResetPasswordError {
    ResetPasswordError::RepositoryError(mysql_error(err))
}
fn reset_infra_error(err: impl Display) -> ResetPasswordError {
    ResetPasswordError::RepositoryError(infra_error(err))
}

#[cfg(test)]
pub mod test {
    use std::{collections::HashMap, sync::Mutex};

    use chrono::{DateTime, Utc};

    use crate::z_details::_api::repository::helper::infra_error;

    use crate::auth::password::_api::kernel::infra::{
        AuthUserPasswordHasher, AuthUserPasswordMatcher, AuthUserPasswordRepository,
        HashedPassword, RequestResetTokenError, ResetPasswordError, ResetTokenEntry,
        ResetTokenEntryExtract, VerifyPasswordError,
    };

    use crate::auth::{
        auth_ticket::_api::kernel::data::{AuthDateTime, ExpireDateTime},
        auth_user::_api::kernel::data::{AuthUser, AuthUserId},
        login_id::_api::data::LoginId,
        password::_api::kernel::data::ResetToken,
    };
    use crate::z_details::_api::repository::data::RepositoryError;

    pub type MemoryAuthUserPasswordStore = Mutex<MemoryAuthUserPasswordMap>;
    pub struct MemoryAuthUserPasswordMap {
        login_id: HashMap<String, AuthUserId>, // login-id => user-id
        password: HashMap<String, HashedPassword>, // user-id => hashed-password
        reset_token: HashMap<String, ResetEntry>, // reset-token => reset entry
    }

    #[derive(Clone)]
    struct ResetEntry {
        user_id: AuthUserId,
        login_id: String,
        expires: DateTime<Utc>,
        reset_at: Option<DateTime<Utc>>,
    }

    impl ResetEntry {
        fn discard(self, reset_at: AuthDateTime) -> Self {
            Self {
                reset_at: Some(reset_at.extract()),
                ..self
            }
        }
    }

    impl MemoryAuthUserPasswordMap {
        pub fn new() -> Self {
            Self {
                login_id: HashMap::new(),
                password: HashMap::new(),
                reset_token: HashMap::new(),
            }
        }

        pub fn with_user_id(login_id: LoginId, user_id: AuthUserId) -> Self {
            let mut store = Self::new();
            store.insert_login_id(login_id, user_id);
            store
        }
        pub fn with_password(login_id: LoginId, user: AuthUser, password: HashedPassword) -> Self {
            let mut store = Self::new();
            store
                .insert_login_id(login_id.clone(), user.into_user_id())
                .insert_password(login_id, password);
            store
        }
        pub fn with_reset_token(
            login_id: LoginId,
            user_id: AuthUserId,
            reset_token: ResetToken,
            expires: ExpireDateTime,
            discard_at: Option<AuthDateTime>,
        ) -> Self {
            let mut store = Self::new();
            store
                .insert_login_id(login_id.clone(), user_id.clone())
                .insert_reset_token(
                    reset_token,
                    ResetEntry {
                        user_id,
                        login_id: login_id.extract(),
                        expires: expires.extract(),
                        reset_at: discard_at.map(|discard_at| discard_at.extract()),
                    },
                );
            store
        }

        pub fn to_store(self) -> MemoryAuthUserPasswordStore {
            Mutex::new(self)
        }

        fn insert_login_id(&mut self, login_id: LoginId, user_id: AuthUserId) -> &mut Self {
            self.login_id.insert(login_id.extract(), user_id);
            self
        }
        fn get_user_id(&self, login_id: &LoginId) -> Option<&AuthUserId> {
            self.login_id.get(login_id.as_str())
        }

        fn insert_password(
            &mut self,
            login_id: LoginId,
            hashed_password: HashedPassword,
        ) -> &mut Self {
            self.password.insert(login_id.extract(), hashed_password);
            self
        }
        fn get_password(&self, login_id: &LoginId) -> Option<&HashedPassword> {
            self.password.get(login_id.as_str())
        }

        fn insert_reset_token(&mut self, token: ResetToken, entry: ResetEntry) -> &mut Self {
            self.reset_token.insert(token.extract(), entry);
            self
        }
        fn get_reset_entry(&self, token: &ResetToken) -> Option<&ResetEntry> {
            self.reset_token.get(token.as_str())
        }
    }

    pub struct MemoryAuthUserPasswordRepository<'a> {
        store: &'a MemoryAuthUserPasswordStore,
    }

    impl<'a> MemoryAuthUserPasswordRepository<'a> {
        pub const fn new(store: &'a MemoryAuthUserPasswordStore) -> Self {
            Self { store }
        }
    }

    #[async_trait::async_trait]
    impl<'store> AuthUserPasswordRepository for MemoryAuthUserPasswordRepository<'store> {
        async fn verify_password<'a>(
            &self,
            login_id: &'a LoginId,
            matcher: impl AuthUserPasswordMatcher + 'a,
        ) -> Result<AuthUserId, VerifyPasswordError> {
            let store = self.store.lock().unwrap();

            let user_id = store
                .get_user_id(login_id)
                .ok_or(VerifyPasswordError::PasswordNotFound)?;

            let password = store
                .get_password(login_id)
                .ok_or(VerifyPasswordError::PasswordNotFound)?;

            let is_matched = matcher
                .match_password(password)
                .map_err(VerifyPasswordError::PasswordHashError)?;

            if is_matched {
                Ok(user_id.clone())
            } else {
                Err(VerifyPasswordError::PasswordNotMatched)
            }
        }

        async fn request_reset_token(
            &self,
            reset_token: ResetToken,
            login_id: LoginId,
            expires: ExpireDateTime,
            _requested_at: AuthDateTime,
        ) -> Result<(), RequestResetTokenError> {
            let target_user_id: AuthUserId;

            {
                let store = self.store.lock().unwrap();

                let user_id = store
                    .get_user_id(&login_id)
                    .ok_or(RequestResetTokenError::NotFound)?;

                // 有効期限が切れていても削除しない
                // もし衝突したら token generator の桁数を増やす
                if store.get_reset_entry(&reset_token).is_some() {
                    return Err(RequestResetTokenError::RepositoryError(infra_error(
                        "reset token conflict",
                    )));
                }

                target_user_id = user_id.clone();
            }

            {
                let mut store = self.store.lock().unwrap();

                // 実際のデータベースには registered_at も保存する必要がある
                store.insert_reset_token(
                    reset_token.clone(),
                    ResetEntry {
                        user_id: target_user_id,
                        login_id: login_id.extract(),
                        expires: expires.extract(),
                        reset_at: None,
                    },
                );
            }

            Ok(())
        }

        async fn reset_token_entry(
            &self,
            reset_token: &ResetToken,
        ) -> Result<Option<ResetTokenEntry>, RepositoryError> {
            let store = self.store.lock().unwrap();

            Ok(store.get_reset_entry(&reset_token).map(|entry| {
                let entry = entry.clone();
                ResetTokenEntryExtract {
                    login_id: entry.login_id,
                    expires: entry.expires,
                    reset_at: entry.reset_at,
                }
                .into()
            }))
        }

        async fn reset_password<'a>(
            &self,
            reset_token: &'a ResetToken,
            hasher: impl AuthUserPasswordHasher + 'a,
            reset_at: AuthDateTime,
        ) -> Result<AuthUserId, ResetPasswordError> {
            let target_entry: ResetEntry;

            {
                let store = self.store.lock().unwrap();

                let entry = store.get_reset_entry(&reset_token).ok_or(
                    ResetPasswordError::RepositoryError(infra_error("reset token not found")),
                )?;

                target_entry = entry.clone().discard(reset_at);
            }

            {
                let hashed_password = hasher
                    .hash_password()
                    .map_err(ResetPasswordError::PasswordHashError)?;

                let mut store = self.store.lock().unwrap();

                // 実際のデータベースには registered_at も保存する必要がある
                store
                    .insert_password(
                        LoginId::restore(target_entry.login_id.clone()),
                        hashed_password,
                    )
                    .insert_reset_token(reset_token.clone(), target_entry.clone());
            }

            Ok(target_entry.user_id)
        }
    }
}

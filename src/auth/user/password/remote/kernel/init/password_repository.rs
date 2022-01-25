use chrono::{TimeZone, Utc};
use sqlx::{query, MySql, MySqlPool, Transaction};

use crate::z_lib::remote::repository::{helper::infra_error, mysql::helper::mysql_error};

use crate::auth::user::password::{
    remote::{
        authenticate::infra::VerifyPasswordRepository,
        change::infra::ChangePasswordRepository,
        kernel::infra::{AuthUserPasswordHasher, AuthUserPasswordMatcher, HashedPassword},
    },
    reset::remote::{
        kernel::infra::{ResetTokenEntry, ResetTokenEntryExtract},
        request_token::infra::RegisterResetTokenRepository,
        reset::infra::ResetPasswordRepository,
    },
};

use crate::{
    auth::{
        ticket::remote::kernel::data::{AuthDateTime, ExpireDateTime},
        user::{
            login_id::remote::data::LoginId,
            password::{
                remote::{
                    authenticate::data::VerifyPasswordRepositoryError,
                    change::data::ChangePasswordRepositoryError,
                },
                reset::remote::{
                    kernel::data::{
                        ResetToken, ResetTokenDestination, ResetTokenDestinationExtract,
                    },
                    request_token::data::RegisterResetTokenRepositoryError,
                    reset::data::ResetPasswordRepositoryError,
                },
            },
            remote::kernel::data::AuthUserId,
        },
    },
    z_lib::remote::repository::data::RepositoryError,
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

#[cfg(test)]
pub mod test {
    use std::{collections::HashMap, sync::Mutex};

    use chrono::{DateTime, Utc};

    use crate::z_lib::remote::repository::helper::infra_error;

    use crate::auth::user::password::{
        remote::{
            authenticate::infra::VerifyPasswordRepository,
            change::infra::ChangePasswordRepository,
            kernel::infra::{AuthUserPasswordHasher, AuthUserPasswordMatcher, HashedPassword},
        },
        reset::remote::{
            kernel::infra::{ResetTokenEntry, ResetTokenEntryExtract},
            request_token::infra::RegisterResetTokenRepository,
            reset::infra::ResetPasswordRepository,
        },
    };

    use crate::{
        auth::{
            ticket::remote::kernel::data::{AuthDateTime, ExpireDateTime},
            user::{
                login_id::remote::data::LoginId,
                password::{
                    remote::{
                        authenticate::data::VerifyPasswordRepositoryError,
                        change::data::ChangePasswordRepositoryError,
                    },
                    reset::remote::{
                        kernel::data::{
                            ResetToken, ResetTokenDestination, ResetTokenDestinationExtract,
                        },
                        request_token::data::RegisterResetTokenRepositoryError,
                        reset::data::ResetPasswordRepositoryError,
                    },
                },
                remote::kernel::data::{AuthUser, AuthUserId},
            },
        },
        z_lib::remote::repository::data::RepositoryError,
    };

    pub type MemoryAuthUserPasswordStore = Mutex<MemoryAuthUserPasswordMap>;
    pub struct MemoryAuthUserPasswordMap {
        login_id: HashMap<String, AuthUserId>, // login-id => user-id
        user_id: HashMap<String, LoginId>,     // login-id => user-id
        password: HashMap<String, HashedPassword>, // user-id => hashed-password
        reset_token: HashMap<String, ResetEntry>, // reset-token => reset entry
    }

    #[derive(Clone)]
    struct ResetEntry {
        user_id: AuthUserId,
        login_id: String,
        destination: ResetTokenDestinationExtract,
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
                user_id: HashMap::new(),
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
            let user_id = user.into_user_id();
            store
                .insert_login_id(login_id.clone(), user_id.clone())
                .insert_password(user_id, password);
            store
        }
        pub fn with_reset_token(
            login_id: LoginId,
            user_id: AuthUserId,
            reset_token: ResetToken,
            destination: ResetTokenDestination,
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
                        destination: destination.extract(),
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
            self.login_id
                .insert(login_id.clone().extract(), user_id.clone());
            self.user_id.insert(user_id.extract(), login_id);
            self
        }
        fn get_user_id(&self, login_id: &LoginId) -> Option<&AuthUserId> {
            self.login_id.get(login_id.as_str())
        }
        pub fn get_login_id(&self, user_id: &str) -> Option<&LoginId> {
            self.user_id.get(user_id)
        }

        fn insert_password(
            &mut self,
            user_id: AuthUserId,
            hashed_password: HashedPassword,
        ) -> &mut Self {
            self.password.insert(user_id.extract(), hashed_password);
            self
        }
        fn get_password(&self, user_id: &AuthUserId) -> Option<&HashedPassword> {
            self.password.get(user_id.as_str())
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
    impl<'store> VerifyPasswordRepository for MemoryAuthUserPasswordRepository<'store> {
        async fn verify_password<'a>(
            &self,
            login_id: &'a LoginId,
            matcher: impl AuthUserPasswordMatcher + 'a,
        ) -> Result<AuthUserId, VerifyPasswordRepositoryError> {
            let store = self.store.lock().unwrap();

            let user_id = store
                .get_user_id(login_id)
                .ok_or(VerifyPasswordRepositoryError::PasswordNotFound)?;

            let password = store
                .get_password(user_id)
                .ok_or(VerifyPasswordRepositoryError::PasswordNotFound)?;

            let matched = matcher
                .match_password(password)
                .map_err(VerifyPasswordRepositoryError::PasswordHashError)?;

            if !matched {
                return Err(VerifyPasswordRepositoryError::PasswordNotMatched);
            }

            Ok(user_id.clone())
        }
    }

    #[async_trait::async_trait]
    impl<'store> ChangePasswordRepository for MemoryAuthUserPasswordRepository<'store> {
        async fn change_password<'a>(
            &self,
            user_id: &'a AuthUserId,
            matcher: impl 'a + AuthUserPasswordMatcher,
            hasher: impl 'a + AuthUserPasswordHasher,
        ) -> Result<(), ChangePasswordRepositoryError> {
            {
                let store = self.store.lock().unwrap();

                let password = store
                    .get_password(user_id)
                    .ok_or(ChangePasswordRepositoryError::PasswordNotFound)?;

                let matched = matcher
                    .match_password(password)
                    .map_err(ChangePasswordRepositoryError::PasswordHashError)?;

                if !matched {
                    return Err(ChangePasswordRepositoryError::PasswordNotMatched);
                }
            }

            {
                let hashed_password = hasher
                    .hash_password()
                    .map_err(ChangePasswordRepositoryError::PasswordHashError)?;

                let mut store = self.store.lock().unwrap();

                // 実際のデータベースには registered_at も保存する必要がある
                store.insert_password(user_id.clone(), hashed_password);
            }

            Ok(())
        }
    }

    #[async_trait::async_trait]
    impl<'store> RegisterResetTokenRepository for MemoryAuthUserPasswordRepository<'store> {
        async fn register_reset_token(
            &self,
            login_id: LoginId,
            reset_token: ResetToken,
            destination: ResetTokenDestination,
            expires: ExpireDateTime,
            _requested_at: AuthDateTime,
        ) -> Result<(), RegisterResetTokenRepositoryError> {
            let target_user_id: AuthUserId;

            {
                let store = self.store.lock().unwrap();

                let user_id = store
                    .get_user_id(&login_id)
                    .ok_or(RegisterResetTokenRepositoryError::UserNotFound)?;

                // 有効期限が切れていても削除しない
                // もし衝突したら token generator の桁数を増やす
                if store.get_reset_entry(&reset_token).is_some() {
                    return Err(RegisterResetTokenRepositoryError::RepositoryError(
                        infra_error("reset token conflict"),
                    ));
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
                        destination: destination.extract(),
                        expires: expires.extract(),
                        reset_at: None,
                    },
                );
            }

            Ok(())
        }
    }

    #[async_trait::async_trait]
    impl<'store> ResetPasswordRepository for MemoryAuthUserPasswordRepository<'store> {
        async fn reset_token_entry(
            &self,
            reset_token: &ResetToken,
        ) -> Result<Option<ResetTokenEntry>, RepositoryError> {
            let store = self.store.lock().unwrap();

            Ok(store.get_reset_entry(&reset_token).map(|entry| {
                let entry = entry.clone();
                ResetTokenEntryExtract {
                    login_id: entry.login_id,
                    destination: entry.destination,
                    expires: entry.expires,
                    reset_at: entry.reset_at,
                }
                .restore()
            }))
        }

        async fn reset_password<'a>(
            &self,
            reset_token: &'a ResetToken,
            hasher: impl AuthUserPasswordHasher + 'a,
            reset_at: AuthDateTime,
        ) -> Result<AuthUserId, ResetPasswordRepositoryError> {
            let target_entry: ResetEntry;

            {
                let store = self.store.lock().unwrap();

                let entry = store.get_reset_entry(&reset_token).ok_or(
                    ResetPasswordRepositoryError::RepositoryError(infra_error(
                        "reset token not found",
                    )),
                )?;

                target_entry = entry.clone().discard(reset_at);
            }

            {
                let hashed_password = hasher
                    .hash_password()
                    .map_err(ResetPasswordRepositoryError::PasswordHashError)?;

                let mut store = self.store.lock().unwrap();

                // 実際のデータベースには registered_at も保存する必要がある
                store
                    .insert_password(target_entry.user_id.clone(), hashed_password)
                    .insert_reset_token(reset_token.clone(), target_entry.clone());
            }

            Ok(target_entry.user_id)
        }
    }
}

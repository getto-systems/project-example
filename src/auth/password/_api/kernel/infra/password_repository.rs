use std::{collections::HashMap, sync::Mutex};

use super::{
    AuthUserPasswordMatcher, AuthUserPasswordRepository, HashedPassword, VerifyPasswordError,
};

use crate::auth::{
    auth_ticket::_api::kernel::data::{AuthDateTime, ExpireDateTime},
    auth_user::_api::kernel::data::{AuthUser, AuthUserId},
    login_id::_api::data::LoginId,
    password::_api::kernel::data::ResetToken,
};
use crate::z_details::_api::repository::data::{RegisterAttemptResult, RepositoryError};

pub type MemoryAuthUserPasswordStore = Mutex<MemoryAuthUserPasswordMap>;
pub struct MemoryAuthUserPasswordMap {
    password: HashMap<String, PasswordEntry>,
    reset_token: HashMap<String, ResetTokenEntry>,
}

struct PasswordEntry(AuthUserId, HashedPassword);
struct ResetTokenEntry {
    user_id: AuthUserId,
    login_id: LoginId,
    expires: ExpireDateTime,
    discard_at: Option<AuthDateTime>,
}

impl MemoryAuthUserPasswordMap {
    pub fn new() -> Self {
        Self {
            password: HashMap::new(),
            reset_token: HashMap::new(),
        }
    }

    pub fn with_password(login_id: LoginId, user: AuthUser, password: HashedPassword) -> Self {
        let mut store = Self::new();
        store.insert_password(login_id, PasswordEntry(user.into_user_id(), password));
        store
    }

    pub fn to_store(self) -> MemoryAuthUserPasswordStore {
        Mutex::new(self)
    }

    fn insert_password(&mut self, login_id: LoginId, entry: PasswordEntry) {
        self.password.insert(login_id.extract(), entry);
    }
    fn get_password(&self, login_id: &LoginId) -> Option<&PasswordEntry> {
        self.password.get(login_id.as_str())
    }

    fn insert_reset_token(&mut self, token: ResetToken, entry: ResetTokenEntry) {
        self.reset_token.insert(token.extract(), entry);
    }
    fn get_reset_token(&self, token: &ResetToken) -> Option<&ResetTokenEntry> {
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

impl<'a> AuthUserPasswordRepository for MemoryAuthUserPasswordRepository<'a> {
    fn verify_password(
        &self,
        login_id: &LoginId,
        matcher: impl AuthUserPasswordMatcher,
    ) -> Result<Option<AuthUserId>, VerifyPasswordError> {
        let store = self.store.lock().unwrap();
        Ok(match store.get_password(login_id) {
            None => None,
            Some(PasswordEntry(user_id, password)) => {
                if matcher
                    .match_password(password)
                    .map_err(VerifyPasswordError::PasswordMatchError)?
                {
                    Some(user_id.clone())
                } else {
                    None
                }
            }
        })
    }

    fn register_reset_token(
        &self,
        user_id: AuthUserId,
        login_id: LoginId,
        token: ResetToken,
        expires: ExpireDateTime,
        _registered_at: AuthDateTime,
    ) -> Result<RegisterAttemptResult<ResetToken>, RepositoryError> {
        let mut store = self.store.lock().unwrap();

        // 有効期限が切れていても削除しない
        // もし衝突したら token generator の桁数を増やす
        if store.get_reset_token(&token).is_some() {
            return Ok(RegisterAttemptResult::Conflict);
        }

        // 実際のデータベースには registered_at も保存する必要がある
        store.insert_reset_token(
            token.clone(),
            ResetTokenEntry {
                user_id,
                login_id,
                expires,
                discard_at: None,
            },
        );

        return Ok(RegisterAttemptResult::Success(token));
    }
}

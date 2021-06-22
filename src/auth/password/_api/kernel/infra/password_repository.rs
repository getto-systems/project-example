use std::{collections::HashMap, sync::Mutex};

use crate::auth::password::_api::kernel::infra::{
    AuthUserPasswordHasher, AuthUserPasswordMatcher, AuthUserPasswordRepository, HashedPassword,
    RegisterResetTokenError, ResetPasswordError, VerifyPasswordError,
};

use crate::auth::{
    auth_ticket::_api::kernel::data::{AuthDateTime, ExpireDateTime},
    auth_user::_api::kernel::data::{AuthUser, AuthUserId},
    login_id::_api::data::LoginId,
    password::_api::kernel::data::ResetToken,
};
use crate::z_details::_api::repository::data::RegisterAttemptResult;

pub type MemoryAuthUserPasswordStore = Mutex<MemoryAuthUserPasswordMap>;
pub struct MemoryAuthUserPasswordMap {
    login_id: HashMap<String, AuthUserId>, // login-id => user-id
    password: HashMap<String, HashedPassword>, // user-id => hashed-password
    reset_token: HashMap<String, ResetEntry>, // reset-token => reset entry
}

#[derive(Clone)]
struct ResetEntry {
    user_id: AuthUserId,
    login_id: LoginId,
    expires: ExpireDateTime,
    discard_at: Option<AuthDateTime>,
}

impl ResetEntry {
    fn discard(self, discard_at: AuthDateTime) -> Self {
        Self {
            discard_at: Some(discard_at),
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
        store.insert_login_id(login_id.clone(), user_id);
        store
    }
    pub fn with_password(login_id: LoginId, user: AuthUser, password: HashedPassword) -> Self {
        let mut store = Self::new();
        store
            .insert_login_id(login_id.clone(), user.into_user_id())
            .insert_password(login_id, password);
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

    fn insert_password(&mut self, login_id: LoginId, hashed_password: HashedPassword) -> &mut Self {
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
    fn get_reset_token(&self, token: &ResetToken) -> Option<&ResetEntry> {
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
        matcher: &impl AuthUserPasswordMatcher,
    ) -> Result<AuthUserId, VerifyPasswordError> {
        let store = self.store.lock().unwrap();

        let user_id = store
            .get_user_id(login_id)
            .ok_or(VerifyPasswordError::NotFound)?;

        let password = store
            .get_password(login_id)
            .ok_or(VerifyPasswordError::NotFound)?;

        let is_matched = matcher
            .match_password(password)
            .map_err(VerifyPasswordError::PasswordMatchError)?;

        if is_matched {
            Ok(user_id.clone())
        } else {
            Err(VerifyPasswordError::NotFound)
        }
    }

    fn register_reset_token(
        &self,
        reset_token: ResetToken,
        login_id: &LoginId,
        expires: &ExpireDateTime,
        _registered_at: &AuthDateTime,
    ) -> Result<RegisterAttemptResult<ResetToken>, RegisterResetTokenError> {
        let target_user_id: AuthUserId;

        {
            let store = self.store.lock().unwrap();

            let user_id = store
                .get_user_id(login_id)
                .ok_or(RegisterResetTokenError::NotFound)?;

            // 有効期限が切れていても削除しない
            // もし衝突したら token generator の桁数を増やす
            if store.get_reset_token(&reset_token).is_some() {
                return Ok(RegisterAttemptResult::Conflict);
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
                    login_id: login_id.clone(),
                    expires: expires.clone(),
                    discard_at: None,
                },
            );
        }

        return Ok(RegisterAttemptResult::Success(reset_token));
    }

    fn reset_password(
        &self,
        reset_token: &ResetToken,
        login_id: &LoginId,
        hasher: &impl AuthUserPasswordHasher,
        reset_at: &AuthDateTime,
    ) -> Result<AuthUserId, ResetPasswordError> {
        let target_entry: ResetEntry;

        {
            let store = self.store.lock().unwrap();

            let entry = store
                .get_reset_token(&reset_token)
                .ok_or(ResetPasswordError::NotFound)?;

            if entry.discard_at.is_some() {
                return Err(ResetPasswordError::AlreadyReset);
            }
            if entry.expires.has_elapsed(reset_at) {
                return Err(ResetPasswordError::NotFound);
            }

            target_entry = entry.clone().discard(reset_at.clone());
        }

        {
            let hashed_password = hasher
                .hash_password()
                .map_err(ResetPasswordError::PasswordHashError)?;

            let mut store = self.store.lock().unwrap();

            // 実際のデータベースには registered_at も保存する必要がある
            store
                .insert_password(login_id.clone(), hashed_password)
                .insert_reset_token(reset_token.clone(), target_entry.clone());
        }

        return Ok(target_entry.user_id);
    }
}

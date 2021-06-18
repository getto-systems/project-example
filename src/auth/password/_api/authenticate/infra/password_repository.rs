use std::{collections::HashMap, sync::Mutex};

use super::{
    AuthUserPasswordMatcher, AuthUserPasswordRepository, HashedPassword, VerifyPasswordError,
};

use crate::auth::{
    auth_user::_api::kernel::data::{AuthUser, AuthUserId},
    login_id::_api::data::LoginId,
};

pub type MemoryAuthUserPasswordStore = Mutex<MemoryAuthUserPasswordMap>;
pub struct MemoryAuthUserPasswordMap(HashMap<String, Entry>);

struct Entry(AuthUserId, HashedPassword);

impl MemoryAuthUserPasswordMap {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn with_password(login_id: LoginId, user: AuthUser, password: HashedPassword) -> Self {
        let mut store = Self::new();
        store.insert(login_id, Entry(user.into_user_id(), password));
        store
    }

    pub fn to_store(self) -> MemoryAuthUserPasswordStore {
        Mutex::new(self)
    }

    fn insert(&mut self, login_id: LoginId, entry: Entry) {
        self.0.insert(login_id.extract(), entry);
    }
    fn get(&self, login_id: &LoginId) -> Option<&Entry> {
        self.0.get(login_id.as_str())
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
        Ok(match store.get(login_id) {
            None => None,
            Some(Entry(user_id, password)) => {
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
}

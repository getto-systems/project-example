use std::{collections::HashMap, sync::Mutex};

use super::{AuthUserPasswordRepository, HashedPassword, MatchPasswordError};

use crate::auth::{
    auth_user::_api::kernel::data::AuthUserId, login_id::_api::data::LoginId,
    password::_api::authenticate::data::PasswordHashError,
};

pub type MemoryAuthUserPasswordStore = Mutex<HashMap<String, (AuthUserId, HashedPassword)>>;

pub struct MemoryAuthUserPasswordRepository<'a> {
    store: &'a MemoryAuthUserPasswordStore,
}

impl<'a> MemoryAuthUserPasswordRepository<'a> {
    pub fn new_store() -> MemoryAuthUserPasswordStore {
        let mut store = HashMap::new();
        // admin/password
        let entry = (AuthUserId::new("admin".into()), HashedPassword::new("$argon2id$v=19$m=4096,t=3,p=1$wL7bldJ+qUCSNYyrgm6OUA$BW+HlZoe6tYaO4yZ3PwQ+F/hj640LiKtfuM8B6YZ+bk".into()));
        store.insert("admin".to_string(), entry);
        Mutex::new(store)
    }

    pub const fn new(store: &'a MemoryAuthUserPasswordStore) -> Self {
        Self { store }
    }
}

impl<'a> AuthUserPasswordRepository for MemoryAuthUserPasswordRepository<'a> {
    fn match_password(
        &self,
        login_id: &LoginId,
        matcher: impl Fn(&HashedPassword) -> Result<bool, PasswordHashError>,
    ) -> Result<Option<AuthUserId>, MatchPasswordError> {
        let store = self.store.lock().unwrap();
        Ok(match store.get(login_id.as_str()) {
            None => None,
            Some((user_id, password)) => {
                if matcher(password).map_err(MatchPasswordError::PasswordHashError)? {
                    Some(user_id.clone())
                } else {
                    None
                }
            }
        })
    }
}

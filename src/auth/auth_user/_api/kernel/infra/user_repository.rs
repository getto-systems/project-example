use std::{
    collections::{HashMap, HashSet},
    sync::Mutex,
};

use super::AuthUserRepository;

use super::super::data::{AuthUser, AuthUserExtract, AuthUserId};
use crate::z_details::_api::repository::data::RepositoryError;

pub type MemoryAuthUserStore = Mutex<MemoryAuthUserMap>;
pub struct MemoryAuthUserMap(HashMap<String, HashSet<String>>);

impl MemoryAuthUserMap {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn with_user(user: AuthUser) -> Self {
        let mut store = Self::new();
        store.insert(user);
        store
    }

    pub fn to_store(self) -> MemoryAuthUserStore {
        Mutex::new(self)
    }

    fn insert(&mut self, user: AuthUser) {
        let user = user.extract();
        self.0.insert(user.user_id, user.granted_roles);
    }
    fn get(&self, user_id: &AuthUserId) -> Option<&HashSet<String>> {
        self.0.get(user_id.as_str())
    }
}

pub struct MemoryAuthUserRepository<'a> {
    store: &'a MemoryAuthUserStore,
}

impl<'a> MemoryAuthUserRepository<'a> {
    pub const fn new(store: &'a MemoryAuthUserStore) -> Self {
        Self { store }
    }
}

impl<'a> AuthUserRepository for MemoryAuthUserRepository<'a> {
    fn get(&self, user_id: &AuthUserId) -> Result<Option<AuthUser>, RepositoryError> {
        let store = self.store.lock().unwrap();
        Ok(store.get(user_id).map(|granted_roles| {
            AuthUserExtract {
                user_id: user_id.as_str().into(),
                granted_roles: granted_roles.clone(),
            }
            .into()
        }))
    }
}

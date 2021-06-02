use std::{
    collections::{HashMap, HashSet},
    sync::Mutex,
};

use super::AuthUserRepository;

use super::super::data::{AuthUser, AuthUserExtract, AuthUserId};
use crate::z_details::_api::repository::data::RepositoryError;

pub type MemoryAuthUserStore = Mutex<HashMap<String, HashSet<String>>>;

pub struct MemoryAuthUserRepository<'a> {
    store: &'a MemoryAuthUserStore,
}

impl<'a> MemoryAuthUserRepository<'a> {
    pub fn new_store() -> MemoryAuthUserStore {
        let mut store = HashMap::new();
        let mut roles = HashSet::new();
        roles.insert("admin".to_string());
        roles.insert("dev-docs".to_string());
        store.insert("admin".to_string(), roles);
        Mutex::new(store)
    }

    pub const fn new(store: &'a MemoryAuthUserStore) -> Self {
        Self { store }
    }
}

impl<'a> AuthUserRepository for MemoryAuthUserRepository<'a> {
    fn get(&self, id: &AuthUserId) -> Result<Option<AuthUser>, RepositoryError> {
        let store = self.store.lock().unwrap();
        Ok(store.get(id.as_str()).map(|granted_roles| {
            AuthUser::from_extract(AuthUserExtract {
                id: id.as_str().into(),
                granted_roles: granted_roles.clone(),
            })
        }))
    }
}

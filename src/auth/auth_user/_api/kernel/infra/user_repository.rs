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
        let extract = user.extract();

        let mut store = HashMap::new();
        store.insert(extract.id, extract.granted_roles);

        Self(store)
    }

    pub fn to_store(self) -> MemoryAuthUserStore {
        Mutex::new(self)
    }

    fn get(&self, user_id: &str) -> Option<&HashSet<String>> {
        self.0.get(user_id)
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

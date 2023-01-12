use std::{collections::HashMap, sync::Mutex};

use crate::auth::user::password::kernel::infra::HashedPassword;

use crate::auth::{
    ticket::kernel::data::AuthPermissionGranted,
    user::{
        account::kernel::data::{AuthUserAccountAttrs, AuthUserMemo},
        kernel::data::AuthUserId,
        login_id::kernel::data::LoginId,
    },
};

pub struct MapUser<'a> {
    store: &'a StoreUser,
}
pub type StoreUser = Mutex<HashMap<AuthUserId, EntryUser>>;

#[derive(Clone)]
pub struct EntryUser {
    pub login_id: LoginId,
    pub granted: Option<AuthPermissionGranted>,
    pub hashed_password: Option<HashedPassword>,
    pub memo: Option<AuthUserMemo>,
}

impl<'a> MapUser<'a> {
    pub fn new_store() -> StoreUser {
        Mutex::new(HashMap::new())
    }
    pub fn new(store: &'a StoreUser) -> Self {
        Self { store }
    }

    pub fn get_password_and_granted(
        &self,
        user_id: &AuthUserId,
    ) -> Option<(HashedPassword, Option<AuthPermissionGranted>)> {
        let store = self.store.lock().unwrap();
        store.get(user_id).and_then(|entry| {
            let password = entry.hashed_password.clone();
            let granted = entry.granted.clone();

            password.map(|password| (password, granted))
        })
    }
    pub fn get_password(&self, user_id: &AuthUserId) -> Option<HashedPassword> {
        let store = self.store.lock().unwrap();
        store
            .get(user_id)
            .and_then(|entry| entry.hashed_password.clone())
    }
    pub fn get_granted(&self, user_id: &AuthUserId) -> Option<AuthPermissionGranted> {
        let store = self.store.lock().unwrap();
        store
            .get(user_id)
            .map(|entry| entry.granted.clone().unwrap_or_default())
    }
    pub fn get_attrs(&self, user_id: &AuthUserId) -> Option<AuthUserAccountAttrs> {
        let store = self.store.lock().unwrap();
        store.get(user_id).map(|entry| AuthUserAccountAttrs {
            granted: entry.granted.clone().unwrap_or_default(),
            memo: entry.memo.clone().unwrap_or(AuthUserMemo::empty()),
        })
    }

    pub fn insert_entry(&self, user_id: AuthUserId, entry: EntryUser) {
        // 本当のデータベースでは user_id がすでに存在したらエラーにする
        let mut store = self.store.lock().unwrap();
        store.insert(user_id, entry);
    }
    pub fn remove_entry(&self, user_id: &AuthUserId) {
        let mut store = self.store.lock().unwrap();
        store.remove(user_id);
    }

    pub fn update_login_id(&self, user_id: AuthUserId, login_id: LoginId) {
        let mut store = self.store.lock().unwrap();
        if let Some(entry) = store.remove(&user_id) {
            store.insert(user_id, EntryUser { login_id, ..entry });
        }
    }
    pub fn update_password(&self, user_id: AuthUserId, new_password: HashedPassword) {
        let mut store = self.store.lock().unwrap();
        if let Some(entry) = store.remove(&user_id) {
            store.insert(
                user_id,
                EntryUser {
                    hashed_password: Some(new_password),
                    ..entry
                },
            );
        }
    }
    pub fn update_user(&self, user_id: AuthUserId, attrs: AuthUserAccountAttrs) {
        let mut store = self.store.lock().unwrap();
        if let Some(entry) = store.remove(&user_id) {
            store.insert(
                user_id,
                EntryUser {
                    granted: Some(attrs.granted),
                    memo: Some(attrs.memo),
                    ..entry
                },
            );
        }
    }

    pub fn all(&self) -> Vec<(AuthUserId, EntryUser)> {
        let store = self.store.lock().unwrap();
        store
            .iter()
            .map(|(user_id, entry)| (user_id.clone(), entry.clone()))
            .collect()
    }
}

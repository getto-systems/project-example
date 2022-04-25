use std::{collections::HashMap, sync::Mutex};

use crate::auth::user::account::modify::infra::ModifyAuthUserAccountChanges;

use crate::auth::user::{
    account::kernel::data::AuthUserAttributes,
    kernel::data::{AuthUserId, GrantedAuthRoles},
    login_id::kernel::data::LoginId,
    password::kernel::infra::HashedPassword,
};

pub struct MapUser<'a> {
    store: &'a StoreUser,
}
pub type StoreUser = Mutex<HashMap<AuthUserId, EntryUser>>;

#[derive(Clone)]
pub struct EntryUser {
    pub login_id: LoginId,
    pub granted_roles: Option<GrantedAuthRoles>,
    pub password: Option<HashedPassword>,
    pub attrs: AuthUserAttributes,
}

impl<'a> MapUser<'a> {
    pub fn new_store() -> StoreUser {
        Mutex::new(HashMap::new())
    }
    pub fn new(store: &'a StoreUser) -> Self {
        Self { store }
    }

    pub fn get_password_and_granted_roles(
        &self,
        user_id: &AuthUserId,
    ) -> Option<(HashedPassword, Option<GrantedAuthRoles>)> {
        let store = self.store.lock().unwrap();
        store.get(user_id).and_then(|entry| {
            let password = entry.password.clone();
            let granted_roles = entry.granted_roles.clone();

            password.map(|password| (password, granted_roles))
        })
    }
    pub fn get_password(&self, user_id: &AuthUserId) -> Option<HashedPassword> {
        let store = self.store.lock().unwrap();
        store.get(user_id).and_then(|entry| entry.password.clone())
    }
    pub fn get_granted_roles(&self, user_id: &AuthUserId) -> Option<Option<GrantedAuthRoles>> {
        let store = self.store.lock().unwrap();
        store.get(user_id).map(|entry| entry.granted_roles.clone())
    }
    pub fn get_modify_changes(&self, user_id: &AuthUserId) -> Option<ModifyAuthUserAccountChanges> {
        let store = self.store.lock().unwrap();
        store
            .get(user_id)
            .map(|entry| ModifyAuthUserAccountChanges {
                granted_roles: entry
                    .granted_roles
                    .clone()
                    .unwrap_or(GrantedAuthRoles::empty()),
                attrs: entry.attrs.clone(),
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
                    password: Some(new_password),
                    ..entry
                },
            );
        }
    }
    pub fn update_user(&self, user_id: AuthUserId, changes: ModifyAuthUserAccountChanges) {
        let mut store = self.store.lock().unwrap();
        if let Some(entry) = store.remove(&user_id) {
            store.insert(
                user_id,
                EntryUser {
                    granted_roles: Some(changes.granted_roles),
                    attrs: changes.attrs,
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

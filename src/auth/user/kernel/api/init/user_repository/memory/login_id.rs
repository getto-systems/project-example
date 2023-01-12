use std::{collections::HashMap, sync::Mutex};

use crate::auth::user::{
    kernel::data::AuthUserId,
    login_id::{change::infra::OverwriteLoginIdEntry, kernel::data::LoginId},
    password::reset::kernel::data::ResetPasswordTokenDestination,
};

pub struct MapLoginId<'a> {
    store: &'a StoreLoginId,
}
pub type StoreLoginId = Mutex<HashMap<LoginId, EntryLoginId>>;

#[derive(Clone)]
pub struct EntryLoginId {
    pub user_id: AuthUserId,
    pub reset_token_destination: Option<ResetPasswordTokenDestination>,
}

impl<'a> MapLoginId<'a> {
    pub fn new_store() -> StoreLoginId {
        Mutex::new(HashMap::new())
    }
    pub fn new(store: &'a StoreLoginId) -> Self {
        Self { store }
    }

    pub fn get_user_id(&self, login_id: &LoginId) -> Option<AuthUserId> {
        let store = self.store.lock().unwrap();
        store.get(login_id).map(|entry| entry.user_id.clone())
    }
    pub fn get_overwrite_entry(&self, login_id: &LoginId) -> Option<OverwriteLoginIdEntry> {
        let store = self.store.lock().unwrap();
        store.get(login_id).map(|entry| OverwriteLoginIdEntry {
            user_id: entry.user_id.clone(),
            login_id: login_id.clone(),
            reset_token_destination: entry.reset_token_destination.clone(),
        })
    }
    pub fn get_reset_token_entry(
        &self,
        login_id: &LoginId,
    ) -> Option<(AuthUserId, Option<ResetPasswordTokenDestination>)> {
        let store = self.store.lock().unwrap();
        store
            .get(login_id)
            .map(|entry| (entry.user_id.clone(), entry.reset_token_destination.clone()))
    }
    pub fn get_reset_token_destination(
        &self,
        login_id: &LoginId,
    ) -> Option<ResetPasswordTokenDestination> {
        let store = self.store.lock().unwrap();
        store
            .get(login_id)
            .and_then(|entry| entry.reset_token_destination.clone())
    }

    pub fn insert_entry(&self, login_id: LoginId, entry: EntryLoginId) {
        // 本当のデータベースでは login_id がすでに存在したらエラーにする
        let mut store = self.store.lock().unwrap();
        store.insert(login_id, entry);
    }
    pub fn insert_overwrite_entry(&self, login_id: LoginId, user: OverwriteLoginIdEntry) {
        let mut store = self.store.lock().unwrap();
        store.insert(
            login_id,
            EntryLoginId {
                user_id: user.user_id,
                reset_token_destination: user.reset_token_destination,
            },
        );
    }
    pub fn remove_entry(&self, login_id: &LoginId) {
        let mut store = self.store.lock().unwrap();
        store.remove(login_id);
    }

    pub fn update_reset_token_destination(
        &self,
        login_id: LoginId,
        new_destination: ResetPasswordTokenDestination,
    ) {
        let mut store = self.store.lock().unwrap();
        if let Some(entry) = store.remove(&login_id) {
            store.insert(
                login_id,
                EntryLoginId {
                    reset_token_destination: Some(new_destination),
                    ..entry
                },
            );
        }
    }

    pub fn all(&self) -> Vec<(LoginId, EntryLoginId)> {
        let store = self.store.lock().unwrap();
        store
            .iter()
            .map(|(login_id, entry)| (login_id.clone(), entry.clone()))
            .collect()
    }
}

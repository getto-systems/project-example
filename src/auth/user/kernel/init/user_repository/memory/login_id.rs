use std::{collections::HashMap, sync::Mutex};

use crate::auth::user::{
    kernel::data::AuthUserId,
    login_id::{change::infra::OverrideLoginIdEntry, kernel::data::LoginId},
    password::reset::kernel::data::ResetTokenDestination,
};

pub struct MapLoginId {
    store: Mutex<HashMap<LoginId, EntryLoginId>>,
}

#[derive(Clone)]
pub struct EntryLoginId {
    pub user_id: AuthUserId,
    pub reset_token_destination: Option<ResetTokenDestination>,
}

impl MapLoginId {
    pub fn new() -> Self {
        Self {
            store: Mutex::new(HashMap::new()),
        }
    }

    pub fn lookup_user_id(&self, login_id: &LoginId) -> Option<AuthUserId> {
        let store = self.store.lock().unwrap();
        store.get(login_id).map(|entry| entry.user_id.clone())
    }
    pub fn lookup_override_entry(&self, login_id: &LoginId) -> Option<OverrideLoginIdEntry> {
        let store = self.store.lock().unwrap();
        store.get(login_id).map(|entry| OverrideLoginIdEntry {
            user_id: entry.user_id.clone(),
            login_id: login_id.clone(),
            reset_token_destination: entry.reset_token_destination.clone(),
        })
    }
    pub fn lookup_reset_token_entry(
        &self,
        login_id: &LoginId,
    ) -> Option<(AuthUserId, Option<ResetTokenDestination>)> {
        let store = self.store.lock().unwrap();
        store
            .get(login_id)
            .map(|entry| (entry.user_id.clone(), entry.reset_token_destination.clone()))
    }
    pub fn lookup_reset_token_destination(
        &self,
        login_id: &LoginId,
    ) -> Option<ResetTokenDestination> {
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
    pub fn insert_override_entry(&self, login_id: LoginId, user: OverrideLoginIdEntry) {
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
        new_destination: ResetTokenDestination,
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

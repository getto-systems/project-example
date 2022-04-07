use std::{collections::HashMap, sync::Mutex};

use crate::auth::{
    ticket::kernel::data::{AuthDateTime, ExpireDateTime},
    user::{
        kernel::data::AuthUserId,
        login_id::kernel::data::LoginId,
        password::reset::{
            kernel::data::{ResetToken, ResetTokenDestination},
            reset::infra::ResetTokenMoment,
        },
    },
};

pub struct MapResetToken {
    store: Mutex<HashMap<ResetToken, EntryResetToken>>,
}

pub struct EntryResetToken {
    pub user_id: AuthUserId,
    pub login_id: LoginId,
    pub destination: ResetTokenDestination,
    pub expires: ExpireDateTime,
    pub requested_at: AuthDateTime,
    pub reset_at: Option<AuthDateTime>,
}

impl MapResetToken {
    pub fn new() -> Self {
        Self {
            store: Mutex::new(HashMap::new()),
        }
    }

    pub fn get_reset_token_entry(
        &self,
        reset_token: &ResetToken,
    ) -> Option<(AuthUserId, LoginId, ResetTokenDestination, ResetTokenMoment)> {
        let store = self.store.lock().unwrap();
        store.get(reset_token).map(|entry| {
            (
                entry.user_id.clone(),
                entry.login_id.clone(),
                entry.destination.clone(),
                ResetTokenMoment::restore(entry.expires.clone(), entry.reset_at.clone()),
            )
        })
    }

    pub fn insert_entry(&self, reset_token: ResetToken, entry: EntryResetToken) {
        let mut store = self.store.lock().unwrap();
        store.insert(reset_token, entry);
    }
    pub fn insert_reset_token(
        &self,
        reset_token: ResetToken,
        user_id: AuthUserId,
        login_id: LoginId,
        destination: ResetTokenDestination,
        expires: ExpireDateTime,
        requested_at: AuthDateTime,
    ) {
        // 本当のデータベースでは、reset token が衝突した場合はエラーにする
        let mut store = self.store.lock().unwrap();
        store.insert(
            reset_token,
            EntryResetToken {
                user_id,
                login_id,
                destination,
                expires,
                requested_at,
                reset_at: None,
            },
        );
    }
    pub fn update_reset_at(&self, reset_token: ResetToken, reset_at: AuthDateTime) {
        let mut store = self.store.lock().unwrap();
        if let Some(entry) = store.remove(&reset_token) {
            store.insert(
                reset_token,
                EntryResetToken {
                    reset_at: Some(reset_at),
                    ..entry
                },
            );
        }
    }
}

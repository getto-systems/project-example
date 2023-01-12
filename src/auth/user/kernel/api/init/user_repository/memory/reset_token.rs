use std::{collections::HashMap, sync::Mutex};

use crate::auth::{
    kernel::data::{AuthDateTime, ExpireDateTime},
    user::{
        kernel::data::AuthUserId,
        login_id::kernel::data::LoginId,
        password::reset::{
            kernel::data::{ResetPasswordId, ResetPasswordTokenDestination},
            reset::infra::ResetPasswordTokenMoment,
        },
    },
};

pub struct MapResetToken<'a> {
    store: &'a StoreResetToken,
}
pub type StoreResetToken = Mutex<HashMap<ResetPasswordId, EntryResetToken>>;

pub struct EntryResetToken {
    pub user_id: AuthUserId,
    pub login_id: LoginId,
    pub destination: ResetPasswordTokenDestination,
    pub expires: ExpireDateTime,
    pub requested_at: AuthDateTime,
    pub reset_at: Option<AuthDateTime>,
}

impl<'a> MapResetToken<'a> {
    pub fn new_store() -> StoreResetToken {
        Mutex::new(HashMap::new())
    }
    pub fn new(store: &'a StoreResetToken) -> Self {
        Self { store }
    }

    pub fn get_reset_token_entry(
        &self,
        reset_token: &ResetPasswordId,
    ) -> Option<(
        AuthUserId,
        LoginId,
        ResetPasswordTokenDestination,
        ResetPasswordTokenMoment,
    )> {
        let store = self.store.lock().unwrap();
        store.get(reset_token).map(|entry| {
            (
                entry.user_id.clone(),
                entry.login_id.clone(),
                entry.destination.clone(),
                ResetPasswordTokenMoment::restore(entry.expires.clone(), entry.reset_at.clone()),
            )
        })
    }

    pub fn insert_entry(&self, reset_token: ResetPasswordId, entry: EntryResetToken) {
        let mut store = self.store.lock().unwrap();
        store.insert(reset_token, entry);
    }
    pub fn insert_reset_token(
        &self,
        reset_token: ResetPasswordId,
        user_id: AuthUserId,
        login_id: LoginId,
        destination: ResetPasswordTokenDestination,
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
    pub fn update_reset_at(&self, reset_token: ResetPasswordId, reset_at: AuthDateTime) {
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

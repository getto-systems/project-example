use crate::auth::user::kernel::detail::repository::memory::StoreLoginId;

use crate::auth::user::{
    kernel::data::AuthUserId, login_id::kernel::data::LoginId,
    password::reset::kernel::data::ResetPasswordTokenDestination,
};

pub struct MapLoginId;

impl MapLoginId {
    pub fn get_user_id(store: &StoreLoginId, login_id: &LoginId) -> Option<AuthUserId> {
        let store = store.lock().unwrap();
        store
            .get(login_id)
            .map(|(user_id, _reset_token_destination)| user_id.clone())
    }
    pub fn get_reset_token_entry(
        store: &StoreLoginId,
        login_id: &LoginId,
    ) -> Option<(AuthUserId, ResetPasswordTokenDestination)> {
        let store = store.lock().unwrap();
        store
            .get(login_id)
            .map(|(user_id, reset_token_destination)| {
                (user_id.clone(), reset_token_destination.clone())
            })
    }
    pub fn get_reset_token_destination(
        store: &StoreLoginId,
        login_id: &LoginId,
    ) -> Option<ResetPasswordTokenDestination> {
        let store = store.lock().unwrap();
        store
            .get(login_id)
            .map(|(_user_id, reset_token_destination)| reset_token_destination.clone())
    }

    pub fn insert_entry(
        store: &StoreLoginId,
        login_id: LoginId,
        user_id: AuthUserId,
        reset_token_destination: ResetPasswordTokenDestination,
    ) {
        // 本当のデータベースでは login_id がすでに存在したらエラーにする
        let mut store = store.lock().unwrap();
        store.insert(login_id, (user_id, reset_token_destination));
    }
    pub fn remove_entry(
        store: &StoreLoginId,
        login_id: &LoginId,
    ) -> Option<(AuthUserId, ResetPasswordTokenDestination)> {
        let mut store = store.lock().unwrap();
        store.remove(login_id)
    }

    pub fn update_reset_token_destination(
        store: &StoreLoginId,
        login_id: LoginId,
        new_destination: ResetPasswordTokenDestination,
    ) {
        let mut store = store.lock().unwrap();
        if let Some((user_id, _reset_token_destination)) = store.remove(&login_id) {
            store.insert(login_id, (user_id, new_destination));
        }
    }

    pub fn find_all(store: &StoreLoginId) -> Vec<(LoginId, ResetPasswordTokenDestination)> {
        let store = store.lock().unwrap();
        store
            .iter()
            .map(|(login_id, (_user_id, reset_token_destination))| {
                (login_id.clone(), reset_token_destination.clone())
            })
            .collect()
    }
}

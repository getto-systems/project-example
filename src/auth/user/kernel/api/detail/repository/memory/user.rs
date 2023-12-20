use crate::auth::user::kernel::detail::repository::memory::StoreUser;

use crate::auth::user::password::kernel::infra::HashedPassword;

use crate::auth::{
    ticket::kernel::data::AuthPermissionGranted,
    user::{
        account::kernel::data::{AuthUserAccountAttrs, AuthUserMemo},
        kernel::data::AuthUserId,
        login_id::kernel::data::LoginId,
    },
};

pub struct MapUser;

impl MapUser {
    pub fn get_password_and_granted(
        store: &StoreUser,
        user_id: &AuthUserId,
    ) -> Option<(HashedPassword, Option<AuthPermissionGranted>)> {
        let store = store.lock().unwrap();
        store
            .get(user_id)
            .and_then(|(_login_id, granted, password, _memo)| {
                let password = password.clone();
                let granted = granted.clone();

                password.map(|password| (password, granted))
            })
    }
    pub fn get_password(store: &StoreUser, user_id: &AuthUserId) -> Option<HashedPassword> {
        let store = store.lock().unwrap();
        store
            .get(user_id)
            .and_then(|(_login_id, _granted, password, _memo)| password.clone())
    }
    pub fn get_granted(store: &StoreUser, user_id: &AuthUserId) -> Option<AuthPermissionGranted> {
        let store = store.lock().unwrap();
        store
            .get(user_id)
            .map(|(_login_id, granted, _password, _memo)| granted.clone().unwrap_or_default())
    }
    pub fn get_attrs(store: &StoreUser, user_id: &AuthUserId) -> Option<AuthUserAccountAttrs> {
        let store = store.lock().unwrap();
        store.get(user_id).map(
            |(_login_id, granted, _password, memo)| AuthUserAccountAttrs {
                granted: granted.clone().unwrap_or_default(),
                memo: memo.clone().unwrap_or_default(),
            },
        )
    }

    pub fn insert_entry(
        store: &StoreUser,
        user_id: AuthUserId,
        entry: (
            LoginId,
            Option<AuthPermissionGranted>,
            Option<HashedPassword>,
            Option<AuthUserMemo>,
        ),
    ) {
        // 本当のデータベースでは user_id がすでに存在したらエラーにする
        let mut store = store.lock().unwrap();
        store.insert(user_id, entry);
    }
    pub fn remove_entry(store: &StoreUser, user_id: &AuthUserId) {
        let mut store = store.lock().unwrap();
        store.remove(user_id);
    }

    pub fn update_login_id(store: &StoreUser, user_id: AuthUserId, login_id: LoginId) {
        let mut store = store.lock().unwrap();
        if let Some((_login_id, granted, password, memo)) = store.remove(&user_id) {
            store.insert(user_id, (login_id, granted, password, memo));
        }
    }
    pub fn update_password(store: &StoreUser, user_id: AuthUserId, new_password: HashedPassword) {
        let mut store = store.lock().unwrap();
        if let Some((login_id, granted, _password, memo)) = store.remove(&user_id) {
            store.insert(user_id, (login_id, granted, Some(new_password), memo));
        }
    }
    pub fn update_user(store: &StoreUser, user_id: AuthUserId, attrs: AuthUserAccountAttrs) {
        let mut store = store.lock().unwrap();
        if let Some((login_id, _granted, password, _memo)) = store.remove(&user_id) {
            store.insert(
                user_id,
                (login_id, Some(attrs.granted), password, Some(attrs.memo)),
            );
        }
    }

    pub fn find_all(
        store: &StoreUser,
    ) -> Vec<(LoginId, Option<AuthPermissionGranted>, Option<AuthUserMemo>)> {
        let store = store.lock().unwrap();
        store
            .iter()
            .map(|(_user_id, (login_id, granted, _password, memo))| {
                (login_id.clone(), granted.clone(), memo.clone())
            })
            .collect()
    }
}

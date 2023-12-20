use crate::auth::user::kernel::detail::repository::memory::StoreResetToken;

use crate::auth::user::password::reset::reset::infra::ResetPasswordTokenMoment;

use crate::auth::{
    kernel::data::{AuthDateTime, ExpireDateTime},
    user::{
        kernel::data::AuthUserId,
        password::reset::kernel::data::{ResetPasswordId, ResetPasswordTokenDestination},
    },
};

pub struct MapResetToken;

impl MapResetToken {
    pub fn get_reset_token_entry(
        store: &StoreResetToken,
        reset_token: &ResetPasswordId,
    ) -> Option<(
        AuthUserId,
        ResetPasswordTokenDestination,
        ResetPasswordTokenMoment,
    )> {
        let store = store.lock().unwrap();
        store.get(reset_token).map(
            |(user_id, reset_token_destination, expires, _requested_at, reset_at)| {
                (
                    user_id.clone(),
                    reset_token_destination.clone(),
                    ResetPasswordTokenMoment::restore(expires.clone(), reset_at.clone()),
                )
            },
        )
    }

    pub fn insert_entry(
        store: &StoreResetToken,
        reset_token: ResetPasswordId,
        user_id: AuthUserId,
        reset_token_destination: ResetPasswordTokenDestination,
        expires: ExpireDateTime,
        requested_at: AuthDateTime,
        reset_at: Option<AuthDateTime>,
    ) {
        let mut store = store.lock().unwrap();
        store.insert(
            reset_token,
            (
                user_id,
                reset_token_destination,
                expires,
                requested_at,
                reset_at,
            ),
        );
    }
    pub fn insert_reset_token(
        store: &StoreResetToken,
        reset_token: ResetPasswordId,
        user_id: AuthUserId,
        destination: ResetPasswordTokenDestination,
        expires: ExpireDateTime,
        requested_at: AuthDateTime,
    ) {
        // 本当のデータベースでは、reset token が衝突した場合はエラーにする
        let mut store = store.lock().unwrap();
        store.insert(
            reset_token,
            (user_id, destination, expires, requested_at, None),
        );
    }
    pub fn update_reset_at(
        store: &StoreResetToken,
        reset_token: ResetPasswordId,
        reset_at: AuthDateTime,
    ) {
        let mut store = store.lock().unwrap();
        if let Some((user_id, reset_token_destination, expires, requested_at, _reset_at)) =
            store.remove(&reset_token)
        {
            store.insert(
                reset_token,
                (
                    user_id,
                    reset_token_destination,
                    expires,
                    requested_at,
                    Some(reset_at),
                ),
            );
        }
    }
}

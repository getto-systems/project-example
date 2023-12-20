pub mod login_id;
pub mod reset_token;
pub mod user;

use std::{collections::HashMap, sync::Mutex};

use crate::auth::user::password::kernel::infra::HashedPassword;

use crate::auth::{
    kernel::data::{AuthDateTime, ExpireDateTime},
    ticket::kernel::data::AuthPermissionGranted,
    user::{
        account::kernel::data::AuthUserMemo,
        kernel::data::AuthUserId,
        login_id::kernel::data::LoginId,
        password::reset::kernel::data::{ResetPasswordId, ResetPasswordTokenDestination},
    },
};

pub type StoreLoginId = Mutex<HashMap<LoginId, (AuthUserId, ResetPasswordTokenDestination)>>;
pub type StoreUser = Mutex<
    HashMap<
        AuthUserId,
        (
            LoginId,
            Option<AuthPermissionGranted>,
            Option<HashedPassword>,
            Option<AuthUserMemo>,
        ),
    >,
>;
pub type StoreResetToken = Mutex<
    HashMap<
        ResetPasswordId,
        (
            AuthUserId,
            ResetPasswordTokenDestination,
            ExpireDateTime,
            AuthDateTime,
            Option<AuthDateTime>,
        ),
    >,
>;

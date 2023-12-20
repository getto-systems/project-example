use std::sync::Arc;

use crate::x_content::permission::AuthPermission;

use crate::{
    auth::user::kernel::detail::repository::memory::{user::MapUser, StoreUser},
    common::api::feature::AsInfra,
};

use crate::auth::user::password::change::action::{ChangePasswordAction, ChangePasswordInfo};

use crate::auth::user::password::{
    change::infra::ChangePasswordFields,
    kernel::infra::{HashedPassword, PlainPassword},
};

use crate::auth::{
    ticket::kernel::data::{AuthPermissionGranted, AuthPermissionRequired},
    user::{
        kernel::data::{AuthUser, AuthUserId},
        login_id::kernel::data::LoginId,
        password::change::data::ChangePasswordError,
    },
};

#[tokio::test]
async fn info() {
    assert_eq!(
        ChangePasswordInfo::required(),
        AuthPermissionRequired::Nothing,
    );
}

#[tokio::test]
async fn success() -> Result<(), ChangePasswordError> {
    let feature = feature(Infra {
        user: vec![(
            AuthUser {
                user_id: AuthUserId::restore("user-id".to_owned()),
                granted: AuthPermissionGranted::restore(
                    vec![AuthPermission::AuthUser].into_iter().collect(),
                ),
            },
            LoginId::restore("login-id".to_owned()),
            HashedPassword::restore("password".to_owned()),
        )],
    });
    let action = ChangePasswordAction::mock(feature.as_infra());

    let user_id = AuthUserId::restore("user-id".to_owned());
    let fields = ChangePasswordFields {
        current_password: PlainPassword::restore("password".to_owned()),
        new_password: PlainPassword::restore("new-password".to_owned()),
    };

    action.change(user_id, fields).await?;

    Ok(())
}

#[tokio::test]
async fn error_user_not_found() {
    let feature = feature(Infra {
        user: vec![(
            AuthUser {
                user_id: AuthUserId::restore("user-id".to_owned()),
                granted: AuthPermissionGranted::restore(
                    vec![AuthPermission::AuthUser].into_iter().collect(),
                ),
            },
            LoginId::restore("login-id".to_owned()),
            HashedPassword::restore("password".to_owned()),
        )],
    });
    let action = ChangePasswordAction::mock(feature.as_infra());

    let user_id = AuthUserId::restore("UNKNOWN-user-id".to_owned());
    let fields = ChangePasswordFields {
        current_password: PlainPassword::restore("password".to_owned()),
        new_password: PlainPassword::restore("new-password".to_owned()),
    };

    let err = action.change(user_id, fields).await.unwrap_err();

    assert_eq!(
        format!("{}", err),
        format!("{}", ChangePasswordError::NotFound),
    );
}

#[tokio::test]
async fn error_password_not_matched() {
    let feature = feature(Infra {
        user: vec![(
            AuthUser {
                user_id: AuthUserId::restore("user-id".to_owned()),
                granted: AuthPermissionGranted::restore(
                    vec![AuthPermission::AuthUser].into_iter().collect(),
                ),
            },
            LoginId::restore("login-id".to_owned()),
            HashedPassword::restore("password".to_owned()),
        )],
    });
    let action = ChangePasswordAction::mock(feature.as_infra());

    let user_id = AuthUserId::restore("user-id".to_owned());
    let fields = ChangePasswordFields {
        current_password: PlainPassword::restore("INVALID-password".to_owned()),
        new_password: PlainPassword::restore("new-password".to_owned()),
    };

    let err = action.change(user_id, fields).await.unwrap_err();

    assert_eq!(
        format!("{}", err),
        format!("{}", ChangePasswordError::PasswordNotMatched),
    );
}

struct Infra {
    user: Vec<(AuthUser, LoginId, HashedPassword)>,
}

fn feature(infra: Infra) -> Arc<StoreUser> {
    let user_store = Arc::new(StoreUser::default());

    for (user, login_id, password) in infra.user {
        MapUser::insert_entry(
            &user_store,
            user.user_id,
            (login_id, Some(user.granted), Some(password), None),
        );
    }

    user_store
}

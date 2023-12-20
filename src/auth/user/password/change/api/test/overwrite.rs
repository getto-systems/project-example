use std::sync::Arc;

use crate::x_content::permission::AuthPermission;

use crate::{
    auth::user::kernel::detail::repository::memory::{
        login_id::MapLoginId, user::MapUser, StoreLoginId, StoreUser,
    },
    common::api::feature::AsInfra,
};

use crate::auth::user::password::change::action::{OverwritePasswordAction, OverwritePasswordInfo};

use crate::auth::user::password::{
    change::infra::OverwritePasswordFields,
    kernel::infra::{HashedPassword, PlainPassword},
};

use crate::auth::{
    ticket::kernel::data::{AuthPermissionGranted, AuthPermissionRequired},
    user::{
        kernel::data::{AuthUser, AuthUserId},
        login_id::kernel::data::LoginId,
        password::change::data::OverwritePasswordError,
    },
};

#[tokio::test]
async fn info() {
    assert_eq!(
        OverwritePasswordInfo::required(),
        AuthPermissionRequired::user(),
    );
}

#[tokio::test]
async fn success() -> Result<(), OverwritePasswordError> {
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
    let action = OverwritePasswordAction::mock(feature.as_infra());

    let fields = OverwritePasswordFields {
        login_id: LoginId::restore("login-id".to_owned()),
        new_password: PlainPassword::restore("new-password".to_owned()),
    };

    action.overwrite(fields).await?;

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
    let action = OverwritePasswordAction::mock(feature.as_infra());

    let fields = OverwritePasswordFields {
        login_id: LoginId::restore("UNKNOWN-login-id".to_owned()),
        new_password: PlainPassword::restore("new-password".to_owned()),
    };

    let err = action.overwrite(fields).await.unwrap_err();

    assert_eq!(
        format!("{}", err),
        format!("{}", OverwritePasswordError::NotFound),
    );
}

struct Infra {
    user: Vec<(AuthUser, LoginId, HashedPassword)>,
}

fn feature(infra: Infra) -> (Arc<StoreLoginId>, Arc<StoreUser>) {
    let login_id_store = Arc::new(StoreLoginId::default());
    let user_store = Arc::new(StoreUser::default());

    for (user, login_id, password) in infra.user {
        MapLoginId::insert_entry(
            &login_id_store,
            login_id.clone(),
            user.user_id.clone(),
            Default::default(),
        );
        MapUser::insert_entry(
            &user_store,
            user.user_id,
            (login_id, Some(user.granted), Some(password), None),
        );
    }

    (login_id_store, user_store)
}

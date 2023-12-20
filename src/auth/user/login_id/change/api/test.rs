use std::sync::Arc;

use pretty_assertions::assert_eq;

use crate::x_content::permission::AuthPermission;

use crate::{
    auth::user::kernel::detail::repository::memory::{
        login_id::MapLoginId, user::MapUser, StoreLoginId, StoreUser,
    },
    common::api::feature::AsInfra,
};

use crate::auth::user::login_id::change::action::{OverwriteLoginIdAction, OverwriteLoginIdInfo};

use crate::auth::user::{
    login_id::change::infra::OverwriteLoginIdFields, password::kernel::infra::HashedPassword,
};

use crate::auth::{
    ticket::kernel::data::{AuthPermissionGranted, AuthPermissionRequired},
    user::{
        kernel::data::{AuthUser, AuthUserId},
        login_id::{change::data::OverwriteLoginIdError, kernel::data::LoginId},
    },
};

#[tokio::test]
async fn info() {
    assert_eq!(
        OverwriteLoginIdInfo::required(),
        AuthPermissionRequired::Nothing,
    );
}

#[tokio::test]
async fn success() -> Result<(), OverwriteLoginIdError> {
    let feature = feature(Infra {
        user: vec![(
            AuthUser {
                user_id: AuthUserId::restore("user-id".to_owned()),
                granted: AuthPermissionGranted::restore(
                    vec![AuthPermission::AuthUser].into_iter().collect(),
                ),
            },
            LoginId::restore("login-id".to_owned()),
        )],
    });
    let action = OverwriteLoginIdAction::mock(feature.as_infra());

    let fields = OverwriteLoginIdFields {
        login_id: LoginId::restore("login-id".to_owned()),
        new_login_id: LoginId::restore("new-login-id".to_owned()),
    };

    action.overwrite(fields).await?;

    Ok(())
}

#[tokio::test]
async fn error_already_registered() {
    let feature = feature(Infra {
        user: vec![
            (
                AuthUser {
                    user_id: AuthUserId::restore("user-id".to_owned()),
                    granted: AuthPermissionGranted::restore(
                        vec![AuthPermission::AuthUser].into_iter().collect(),
                    ),
                },
                LoginId::restore("login-id".to_owned()),
            ),
            (
                AuthUser {
                    user_id: AuthUserId::restore("user-id-another".to_owned()),
                    granted: AuthPermissionGranted::restore(
                        vec![AuthPermission::AuthUser].into_iter().collect(),
                    ),
                },
                LoginId::restore("login-id-another".to_owned()),
            ),
        ],
    });
    let action = OverwriteLoginIdAction::mock(feature.as_infra());

    let fields = OverwriteLoginIdFields {
        login_id: LoginId::restore("login-id".to_owned()),
        new_login_id: LoginId::restore("login-id-another".to_owned()),
    };

    let err = action.overwrite(fields).await.unwrap_err();

    assert_eq!(
        format!("{}", err),
        format!("{}", OverwriteLoginIdError::AlreadyRegistered),
    )
}

#[tokio::test]
async fn error_not_found() {
    let feature = feature(Infra {
        user: vec![(
            AuthUser {
                user_id: AuthUserId::restore("user-id".to_owned()),
                granted: AuthPermissionGranted::restore(
                    vec![AuthPermission::AuthUser].into_iter().collect(),
                ),
            },
            LoginId::restore("login-id".to_owned()),
        )],
    });
    let action = OverwriteLoginIdAction::mock(feature.as_infra());

    let fields = OverwriteLoginIdFields {
        login_id: LoginId::restore("UNKNOWN-login-id".to_owned()),
        new_login_id: LoginId::restore("new-login-id".to_owned()),
    };

    let err = action.overwrite(fields).await.unwrap_err();

    assert_eq!(
        format!("{}", err),
        format!("{}", OverwriteLoginIdError::NotFound),
    )
}

struct Infra {
    user: Vec<(AuthUser, LoginId)>,
}

fn feature(infra: Infra) -> (Arc<StoreLoginId>, Arc<StoreUser>) {
    let login_id_store = Arc::new(StoreLoginId::default());
    let user_store = Arc::new(StoreUser::default());

    for (user, login_id) in infra.user {
        MapLoginId::insert_entry(
            &login_id_store,
            login_id.clone(),
            user.user_id.clone(),
            Default::default(),
        );
        MapUser::insert_entry(
            &user_store,
            user.user_id,
            (
                login_id,
                Some(user.granted),
                Some(HashedPassword::restore("PASSWORD".to_owned())),
                None,
            ),
        );
    }

    (login_id_store, user_store)
}

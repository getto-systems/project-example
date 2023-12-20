use std::sync::Arc;

use pretty_assertions::assert_eq;

use crate::x_content::permission::AuthPermission;

use crate::common::api::feature::AsInfra;

use crate::auth::user::kernel::detail::repository::memory::{
    login_id::MapLoginId, user::MapUser, StoreLoginId, StoreUser,
};

use crate::auth::user::account::modify::action::{
    ModifyAuthUserAccountAction, ModifyAuthUserAccountInfo,
};

use crate::auth::user::{
    account::modify::infra::ModifyAuthUserAccountFields, password::kernel::infra::HashedPassword,
};

use crate::auth::{
    ticket::kernel::data::{AuthPermissionGranted, AuthPermissionRequired},
    user::{
        account::{
            kernel::data::{AuthUserAccountAttrs, AuthUserMemo},
            modify::data::ModifyAuthUserAccountError,
        },
        kernel::data::{AuthUser, AuthUserId},
        login_id::kernel::data::LoginId,
    },
};

#[tokio::test]
async fn info() {
    assert_eq!(
        ModifyAuthUserAccountInfo::required(),
        AuthPermissionRequired::user(),
    );
}

#[tokio::test]
async fn success() -> Result<(), ModifyAuthUserAccountError> {
    let feature = feature(Infra {
        user: vec![(
            AuthUser {
                user_id: AuthUserId::restore("user-id".to_owned()),
                granted: AuthPermissionGranted::restore(
                    vec![AuthPermission::AuthUser].into_iter().collect(),
                ),
            },
            LoginId::restore("login-id".to_owned()),
            Some(AuthUserMemo::restore("memo".to_owned())),
        )],
    });
    let action = ModifyAuthUserAccountAction::mock(feature.as_infra());

    let fields = ModifyAuthUserAccountFields {
        login_id: LoginId::restore("login-id".to_owned()),
        from: AuthUserAccountAttrs {
            granted: AuthPermissionGranted::restore(
                vec![AuthPermission::AuthUser].into_iter().collect(),
            ),
            memo: AuthUserMemo::restore("memo".to_owned()),
        },
        to: AuthUserAccountAttrs {
            granted: AuthPermissionGranted::restore(
                vec![AuthPermission::AuthUser].into_iter().collect(),
            ),
            memo: AuthUserMemo::restore("modified-memo".to_owned()),
        },
    };

    action.modify(fields).await?;

    Ok(())
}

#[tokio::test]
async fn error_user_id_not_found() {
    let feature = feature(Infra {
        user: vec![(
            AuthUser {
                user_id: AuthUserId::restore("user-id".to_owned()),
                granted: AuthPermissionGranted::restore(
                    vec![AuthPermission::AuthUser].into_iter().collect(),
                ),
            },
            LoginId::restore("login-id".to_owned()),
            Some(AuthUserMemo::restore("memo".to_owned())),
        )],
    });
    let action = ModifyAuthUserAccountAction::mock(feature.as_infra());

    let fields = ModifyAuthUserAccountFields {
        login_id: LoginId::restore("UNKNOWN-login-id".to_owned()),
        from: AuthUserAccountAttrs {
            granted: AuthPermissionGranted::restore(
                vec![AuthPermission::AuthUser].into_iter().collect(),
            ),
            memo: AuthUserMemo::restore("memo".to_owned()),
        },
        to: AuthUserAccountAttrs {
            granted: AuthPermissionGranted::restore(
                vec![AuthPermission::AuthUser].into_iter().collect(),
            ),
            memo: AuthUserMemo::restore("modified-memo".to_owned()),
        },
    };

    let err = action.modify(fields).await.unwrap_err();

    assert_eq!(
        format!("{}", err),
        format!("{}", ModifyAuthUserAccountError::NotFound),
    )
}

#[tokio::test]
async fn error_conflict() {
    let feature = feature(Infra {
        user: vec![(
            AuthUser {
                user_id: AuthUserId::restore("user-id".to_owned()),
                granted: AuthPermissionGranted::restore(
                    vec![AuthPermission::AuthUser].into_iter().collect(),
                ),
            },
            LoginId::restore("login-id".to_owned()),
            Some(AuthUserMemo::restore("memo".to_owned())),
        )],
    });
    let action = ModifyAuthUserAccountAction::mock(feature.as_infra());

    let fields = ModifyAuthUserAccountFields {
        login_id: LoginId::restore("login-id".to_owned()),
        from: AuthUserAccountAttrs {
            granted: AuthPermissionGranted::restore(
                vec![AuthPermission::AuthUser].into_iter().collect(),
            ),
            memo: AuthUserMemo::restore("conflict-memo".to_owned()),
        },
        to: AuthUserAccountAttrs {
            granted: AuthPermissionGranted::restore(
                vec![AuthPermission::AuthUser].into_iter().collect(),
            ),
            memo: AuthUserMemo::restore("modified-memo".to_owned()),
        },
    };

    let err = action.modify(fields).await.unwrap_err();

    assert_eq!(
        format!("{}", err),
        format!("{}", ModifyAuthUserAccountError::Conflict),
    )
}

struct Infra {
    user: Vec<(AuthUser, LoginId, Option<AuthUserMemo>)>,
}

fn feature(infra: Infra) -> (Arc<StoreLoginId>, Arc<StoreUser>) {
    let login_id_store = Arc::new(StoreLoginId::default());
    let user_store = Arc::new(StoreUser::default());

    for (user, login_id, memo) in infra.user {
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
                memo,
            ),
        );
    }

    (login_id_store, user_store)
}

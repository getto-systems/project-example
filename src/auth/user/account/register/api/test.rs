use std::sync::Arc;

use pretty_assertions::assert_eq;

use crate::x_content::permission::AuthPermission;

use crate::{
    auth::user::{
        account::register::api::detail::test::MockAuthUserIdGenerator,
        kernel::detail::repository::memory::{
            login_id::MapLoginId, user::MapUser, StoreLoginId, StoreUser,
        },
    },
    common::api::feature::AsInfra,
};

use crate::auth::user::account::register::action::{
    RegisterAuthUserAccountAction, RegisterAuthUserAccountInfo,
};

use crate::auth::user::password::kernel::infra::HashedPassword;

use crate::auth::{
    ticket::kernel::data::{AuthPermissionGranted, AuthPermissionRequired},
    user::{
        account::{
            kernel::data::{AuthUserAccount, AuthUserAccountAttrs, AuthUserMemo},
            register::data::RegisterAuthUserAccountError,
        },
        kernel::data::{AuthUser, AuthUserId},
        login_id::kernel::data::LoginId,
        password::reset::kernel::data::ResetPasswordTokenDestination,
    },
};

#[tokio::test]
async fn info() {
    assert_eq!(
        RegisterAuthUserAccountInfo::required(),
        AuthPermissionRequired::user(),
    );
}

#[tokio::test]
async fn success() -> Result<(), RegisterAuthUserAccountError> {
    let feature = feature(Infra {
        user: vec![(
            AuthUser {
                user_id: AuthUserId::restore("stored-user-id".to_owned()),
                granted: AuthPermissionGranted::restore(
                    vec![AuthPermission::AuthUser].into_iter().collect(),
                ),
            },
            LoginId::restore("stored-login-id".to_owned()),
        )],
    });
    let action = RegisterAuthUserAccountAction::mock(feature.as_infra());

    let fields = AuthUserAccount {
        login_id: LoginId::restore("login-id".to_owned()),
        attrs: AuthUserAccountAttrs {
            granted: AuthPermissionGranted::restore(
                vec![AuthPermission::AuthUser].into_iter().collect(),
            ),
            memo: AuthUserMemo::restore("memo".to_owned()),
        },
        reset_token_destination: ResetPasswordTokenDestination::None,
    };

    action.register(fields).await?;

    Ok(())
}

#[tokio::test]
async fn error_login_id_already_registered() {
    let feature = feature(Infra {
        user: vec![(
            AuthUser {
                user_id: AuthUserId::restore("stored-user-id".to_owned()),
                granted: AuthPermissionGranted::restore(
                    vec![AuthPermission::AuthUser].into_iter().collect(),
                ),
            },
            LoginId::restore("stored-login-id".to_owned()),
        )],
    });
    let action = RegisterAuthUserAccountAction::mock(feature.as_infra());

    let fields = AuthUserAccount {
        login_id: LoginId::restore("stored-login-id".to_owned()),
        attrs: AuthUserAccountAttrs {
            granted: AuthPermissionGranted::restore(
                vec![AuthPermission::AuthUser].into_iter().collect(),
            ),
            memo: AuthUserMemo::restore("memo".to_owned()),
        },
        reset_token_destination: ResetPasswordTokenDestination::None,
    };

    let err = action.register(fields).await.unwrap_err();

    assert_eq!(
        format!("{}", err),
        format!("{}", RegisterAuthUserAccountError::LoginIdAlreadyRegistered),
    )
}

struct Infra {
    user: Vec<(AuthUser, LoginId)>,
}

fn feature(infra: Infra) -> (MockAuthUserIdGenerator, Arc<StoreLoginId>, Arc<StoreUser>) {
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

    (
        MockAuthUserIdGenerator::new(AuthUserId::restore("user-id".to_owned())),
        login_id_store,
        user_store,
    )
}

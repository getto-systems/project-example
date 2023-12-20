use std::sync::Arc;

use pretty_assertions::assert_eq;

use crate::{
    auth::user::{
        kernel::detail::repository::memory::{
            login_id::MapLoginId, user::MapUser, StoreLoginId, StoreUser,
        },
        password::authenticate::{
            action::AuthenticateWithPasswordAction, data::AuthenticateWithPasswordError,
        },
    },
    common::api::feature::AsInfra,
    x_content::permission::AuthPermission,
};

use crate::auth::user::password::{
    authenticate::infra::AuthenticateWithPasswordFields,
    kernel::infra::{HashedPassword, PlainPassword},
};

use crate::auth::{
    ticket::kernel::data::{AuthPermissionGranted, AuthenticateSuccess},
    user::{
        kernel::data::{AuthUser, AuthUserId},
        login_id::kernel::data::LoginId,
    },
};

#[tokio::test]
async fn success() -> Result<(), AuthenticateWithPasswordError> {
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
    let action = AuthenticateWithPasswordAction::mock(feature.as_infra());

    let fields = AuthenticateWithPasswordFields {
        login_id: LoginId::restore("login-id".to_owned()),
        plain_password: PlainPassword::restore("password".to_owned()),
    };

    let auth = action.authenticate(fields).await?;

    assert_eq!(
        auth,
        AuthenticateSuccess::new(AuthUser {
            user_id: AuthUserId::restore("user-id".to_owned()),
            granted: AuthPermissionGranted::restore(
                vec![AuthPermission::AuthUser].into_iter().collect(),
            ),
        }),
    );

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
    let action = AuthenticateWithPasswordAction::mock(feature.as_infra());

    let fields = AuthenticateWithPasswordFields {
        login_id: LoginId::restore("UNKNOWN-login-id".to_owned()),
        plain_password: PlainPassword::restore("password".to_owned()),
    };

    let err = action.authenticate(fields).await.unwrap_err();

    assert_eq!(
        format!("{}", err),
        format!(
            "{}",
            AuthenticateWithPasswordError::NotFound(LoginId::restore(
                "UNKNOWN-login-id".to_owned()
            ))
        ),
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
    let action = AuthenticateWithPasswordAction::mock(feature.as_infra());

    let fields = AuthenticateWithPasswordFields {
        login_id: LoginId::restore("login-id".to_owned()),
        plain_password: PlainPassword::restore("UNKNOWN-password".to_owned()),
    };

    let err = action.authenticate(fields).await.unwrap_err();

    assert_eq!(
        format!("{}", err),
        format!("{}", AuthenticateWithPasswordError::PasswordNotMatched),
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

use std::sync::Arc;

use pretty_assertions::assert_eq;

use crate::{
    auth::{
        ticket::kernel::detail::repository::memory::StoreTicket,
        user::kernel::detail::repository::memory::{
            login_id::MapLoginId, user::MapUser, StoreLoginId, StoreUser,
        },
    },
    common::api::feature::AsInfra,
    x_content::permission::AuthPermission,
};

use crate::auth::user::account::unregister::action::{
    UnregisterAuthUserAccountAction, UnregisterAuthUserAccountInfo,
};

use crate::auth::user::{
    account::unregister::infra::UnregisterAuthUserAccountFields,
    password::kernel::infra::HashedPassword,
};

use crate::auth::{
    ticket::kernel::data::{AuthPermissionGranted, AuthPermissionRequired},
    user::{
        account::unregister::data::UnregisterAuthUserAccountError,
        kernel::data::{AuthUser, AuthUserId},
        login_id::kernel::data::LoginId,
    },
};

#[tokio::test]
async fn info() {
    assert_eq!(
        UnregisterAuthUserAccountInfo::required(),
        AuthPermissionRequired::user(),
    );
}

#[tokio::test]
async fn success() -> Result<(), UnregisterAuthUserAccountError> {
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
    let action = UnregisterAuthUserAccountAction::mock(feature.as_infra());

    let fields = UnregisterAuthUserAccountFields {
        login_id: LoginId::restore("login-id".to_owned()),
    };

    action.unregister(fields).await?;

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
        )],
    });
    let action = UnregisterAuthUserAccountAction::mock(feature.as_infra());

    let fields = UnregisterAuthUserAccountFields {
        login_id: LoginId::restore("UNKNOWN-login-id".to_owned()),
    };

    let err = action.unregister(fields).await.unwrap_err();

    assert_eq!(
        format!("{}", err),
        format!("{}", UnregisterAuthUserAccountError::NotFound),
    )
}

struct Infra {
    user: Vec<(AuthUser, LoginId)>,
}

fn feature(infra: Infra) -> (Arc<StoreTicket>, Arc<StoreLoginId>, Arc<StoreUser>) {
    let ticket_store = Arc::new(StoreTicket::default());
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

    (ticket_store, login_id_store, user_store)
}

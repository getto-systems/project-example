use std::sync::Arc;

use pretty_assertions::assert_eq;

use crate::x_content::permission::AuthPermission;

use crate::{
    auth::user::kernel::detail::repository::memory::{login_id::MapLoginId, StoreLoginId},
    common::api::feature::AsInfra,
};

use crate::auth::user::password::reset::token_destination::change::action::{
    ChangeResetTokenDestinationAction, ChangeResetTokenDestinationInfo,
};

use crate::auth::user::password::reset::token_destination::change::infra::ChangeResetTokenDestinationFields;

use crate::auth::{
    ticket::kernel::data::{AuthPermissionGranted, AuthPermissionRequired},
    user::{
        kernel::data::{AuthUser, AuthUserId},
        login_id::kernel::data::LoginId,
        password::reset::{
            kernel::data::{ResetPasswordTokenDestination, ResetPasswordTokenDestinationEmail},
            token_destination::change::data::ChangeResetTokenDestinationError,
        },
    },
};

#[tokio::test]
async fn info() {
    assert_eq!(
        ChangeResetTokenDestinationInfo::required(),
        AuthPermissionRequired::user(),
    );
}

#[tokio::test]
async fn success() -> Result<(), ChangeResetTokenDestinationError> {
    let feature = feature(Infra {
        user: vec![(
            AuthUser {
                user_id: AuthUserId::restore("user-id".to_owned()),
                granted: AuthPermissionGranted::restore(
                    vec![AuthPermission::AuthUser].into_iter().collect(),
                ),
            },
            LoginId::restore("login-id".to_owned()),
            ResetPasswordTokenDestination::Email(ResetPasswordTokenDestinationEmail::restore(
                "destination@example.com".to_owned(),
            )),
        )],
    });
    let action = ChangeResetTokenDestinationAction::mock(feature.as_infra());

    let fields = ChangeResetTokenDestinationFields {
        login_id: LoginId::restore("login-id".to_owned()),
        from: ResetPasswordTokenDestination::Email(ResetPasswordTokenDestinationEmail::restore(
            "destination@example.com".to_owned(),
        )),
        to: ResetPasswordTokenDestination::Email(ResetPasswordTokenDestinationEmail::restore(
            "new-destination@example.com".to_owned(),
        )),
    };

    action.change(fields).await?;

    Ok(())
}

#[tokio::test]
async fn success_with_no_destination() -> Result<(), ChangeResetTokenDestinationError> {
    let feature = feature(Infra {
        user: vec![(
            AuthUser {
                user_id: AuthUserId::restore("user-id".to_owned()),
                granted: AuthPermissionGranted::restore(
                    vec![AuthPermission::AuthUser].into_iter().collect(),
                ),
            },
            LoginId::restore("login-id".to_owned()),
            ResetPasswordTokenDestination::None,
        )],
    });
    let action = ChangeResetTokenDestinationAction::mock(feature.as_infra());

    let fields = ChangeResetTokenDestinationFields {
        login_id: LoginId::restore("login-id".to_owned()),
        from: ResetPasswordTokenDestination::None,
        to: ResetPasswordTokenDestination::Email(ResetPasswordTokenDestinationEmail::restore(
            "new-destination@example.com".to_owned(),
        )),
    };

    action.change(fields).await?;

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
            ResetPasswordTokenDestination::None,
        )],
    });
    let action = ChangeResetTokenDestinationAction::mock(feature.as_infra());

    let fields = ChangeResetTokenDestinationFields {
        login_id: LoginId::restore("UNKNOWN-login-id".to_owned()),
        from: ResetPasswordTokenDestination::None,
        to: ResetPasswordTokenDestination::Email(ResetPasswordTokenDestinationEmail::restore(
            "new-destination@example.com".to_owned(),
        )),
    };

    let err = action.change(fields).await.unwrap_err();

    assert_eq!(
        format!("{}", err),
        format!("{}", ChangeResetTokenDestinationError::NotFound),
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
            ResetPasswordTokenDestination::None,
        )],
    });
    let action = ChangeResetTokenDestinationAction::mock(feature.as_infra());

    let fields = ChangeResetTokenDestinationFields {
        login_id: LoginId::restore("login-id".to_owned()),
        from: ResetPasswordTokenDestination::Email(ResetPasswordTokenDestinationEmail::restore(
            "destination@example.com".to_owned(),
        )),
        to: ResetPasswordTokenDestination::Email(ResetPasswordTokenDestinationEmail::restore(
            "new-destination@example.com".to_owned(),
        )),
    };

    let err = action.change(fields).await.unwrap_err();

    assert_eq!(
        format!("{}", err),
        format!("{}", ChangeResetTokenDestinationError::Conflict),
    )
}

struct Infra {
    user: Vec<(AuthUser, LoginId, ResetPasswordTokenDestination)>,
}

fn feature(infra: Infra) -> Arc<StoreLoginId> {
    let login_id_store = Arc::new(StoreLoginId::default());

    for (user, login_id, reset_token_destination) in infra.user {
        MapLoginId::insert_entry(
            &login_id_store,
            login_id.clone(),
            user.user_id.clone(),
            reset_token_destination,
        );
    }

    login_id_store
}

use std::sync::Arc;

use chrono::{DateTime, TimeZone, Utc};
use pretty_assertions::assert_eq;

use crate::x_content::permission::AuthPermission;

use crate::{
    auth::{
        kernel::detail::test::MockChronoAuthClock,
        user::{
            kernel::detail::repository::memory::{
                reset_token::MapResetToken, user::MapUser, StoreResetToken, StoreUser,
            },
            password::reset::kernel::detail::token::decoder::test::MockResetTokenDecoder,
        },
    },
    common::api::feature::AsInfra,
};

use crate::auth::user::password::reset::reset::action::ResetPasswordAction;

use crate::auth::user::password::{
    kernel::infra::PlainPassword, reset::reset::infra::ResetPasswordFields,
};

use crate::auth::{
    kernel::data::{AuthDateTime, ExpireDateTime},
    ticket::kernel::data::{AuthPermissionGranted, AuthenticateSuccess},
    user::{
        kernel::data::{AuthUser, AuthUserId},
        login_id::kernel::data::LoginId,
        password::reset::{
            kernel::data::{
                ResetPasswordId, ResetPasswordToken, ResetPasswordTokenDestination,
                ResetPasswordTokenDestinationEmail,
            },
            reset::data::ResetPasswordError,
        },
    },
};

#[tokio::test]
async fn success() -> Result<(), ResetPasswordError> {
    let feature = feature(Infra {
        now: Utc.with_ymd_and_hms(2021, 1, 1, 10, 0, 0).unwrap(),
        user: vec![(
            AuthUserId::restore("user-id".to_owned()),
            LoginId::restore("login-id".to_owned()),
            AuthPermissionGranted::restore(vec![AuthPermission::AuthUser].into_iter().collect()),
        )],
        reset_token: vec![(
            ResetPasswordToken::restore("TOKEN".to_owned()),
            ResetPasswordId::restore("reset-id".to_owned()),
            AuthUserId::restore("user-id".to_owned()),
            ResetPasswordTokenDestination::Email(ResetPasswordTokenDestinationEmail::restore(
                "destination@example.com".to_owned(),
            )),
            ExpireDateTime::restore(Utc.with_ymd_and_hms(2021, 1, 2, 10, 0, 0).unwrap()),
            AuthDateTime::restore(Utc.with_ymd_and_hms(2021, 1, 1, 10, 0, 0).unwrap()),
            None,
        )],
    });
    let action = ResetPasswordAction::mock(feature.as_infra());

    let fields = ResetPasswordFields {
        reset_token: ResetPasswordToken::restore("TOKEN".to_owned()),
        new_password: PlainPassword::restore("new-password".to_owned()),
    };

    let auth = action.reset(fields).await?;

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
async fn error_reset_id_expired() {
    let feature = feature(Infra {
        now: Utc.with_ymd_and_hms(2021, 1, 1, 10, 0, 0).unwrap(),
        user: vec![(
            AuthUserId::restore("user-id".to_owned()),
            LoginId::restore("login-id".to_owned()),
            AuthPermissionGranted::restore(vec![AuthPermission::AuthUser].into_iter().collect()),
        )],
        reset_token: vec![(
            ResetPasswordToken::restore("TOKEN".to_owned()),
            ResetPasswordId::restore("reset-id".to_owned()),
            AuthUserId::restore("user-id".to_owned()),
            ResetPasswordTokenDestination::Email(ResetPasswordTokenDestinationEmail::restore(
                "destination@example.com".to_owned(),
            )),
            ExpireDateTime::restore(Utc.with_ymd_and_hms(2021, 1, 1, 9, 0, 0).unwrap()),
            AuthDateTime::restore(Utc.with_ymd_and_hms(2021, 1, 1, 9, 0, 0).unwrap()),
            None,
        )],
    });
    let action = ResetPasswordAction::mock(feature.as_infra());

    let fields = ResetPasswordFields {
        reset_token: ResetPasswordToken::restore("TOKEN".to_owned()),
        new_password: PlainPassword::restore("new-password".to_owned()),
    };

    let err = action.reset(fields).await.unwrap_err();

    assert_eq!(
        format!("{}", err),
        format!("{}", ResetPasswordError::ResetTokenExpired),
    );
}

#[tokio::test]
async fn error_reset_id_already_reset() {
    let feature = feature(Infra {
        now: Utc.with_ymd_and_hms(2021, 1, 1, 10, 0, 0).unwrap(),
        user: vec![(
            AuthUserId::restore("user-id".to_owned()),
            LoginId::restore("login-id".to_owned()),
            AuthPermissionGranted::restore(vec![AuthPermission::AuthUser].into_iter().collect()),
        )],
        reset_token: vec![(
            ResetPasswordToken::restore("TOKEN".to_owned()),
            ResetPasswordId::restore("reset-id".to_owned()),
            AuthUserId::restore("user-id".to_owned()),
            ResetPasswordTokenDestination::Email(ResetPasswordTokenDestinationEmail::restore(
                "destination@example.com".to_owned(),
            )),
            ExpireDateTime::restore(Utc.with_ymd_and_hms(2021, 1, 1, 9, 0, 0).unwrap()),
            AuthDateTime::restore(Utc.with_ymd_and_hms(2021, 1, 1, 9, 0, 0).unwrap()),
            Some(AuthDateTime::restore(
                Utc.with_ymd_and_hms(2021, 1, 1, 9, 0, 0).unwrap(),
            )),
        )],
    });
    let action = ResetPasswordAction::mock(feature.as_infra());

    let fields = ResetPasswordFields {
        reset_token: ResetPasswordToken::restore("TOKEN".to_owned()),
        new_password: PlainPassword::restore("new-password".to_owned()),
    };

    let err = action.reset(fields).await.unwrap_err();

    assert_eq!(
        format!("{}", err),
        format!("{}", ResetPasswordError::AlreadyReset),
    );
}

struct Infra {
    now: DateTime<Utc>,
    user: Vec<(AuthUserId, LoginId, AuthPermissionGranted)>,
    reset_token: Vec<(
        ResetPasswordToken,
        ResetPasswordId,
        AuthUserId,
        ResetPasswordTokenDestination,
        ExpireDateTime,
        AuthDateTime,
        Option<AuthDateTime>,
    )>,
}

fn feature(
    infra: Infra,
) -> (
    MockChronoAuthClock,
    Arc<StoreUser>,
    Arc<StoreResetToken>,
    MockResetTokenDecoder,
) {
    let user_store = Arc::new(StoreUser::default());
    let reset_token_store = Arc::new(StoreResetToken::default());

    let mut decoder = Vec::new();

    for (user_id, login_id, granted) in infra.user {
        MapUser::insert_entry(&user_store, user_id, (login_id, Some(granted), None, None));
    }
    for (
        reset_token,
        reset_id,
        user_id,
        reset_token_destination,
        expires,
        requested_at,
        reset_at,
    ) in infra.reset_token
    {
        MapResetToken::insert_entry(
            &reset_token_store,
            reset_id.clone(),
            user_id,
            reset_token_destination,
            expires,
            requested_at,
            reset_at,
        );
        decoder.push((reset_token.extract(), Ok(reset_id)))
    }

    (
        MockChronoAuthClock::new(infra.now),
        user_store,
        reset_token_store,
        MockResetTokenDecoder::new(decoder.into_iter().collect()),
    )
}

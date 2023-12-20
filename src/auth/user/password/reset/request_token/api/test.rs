use std::sync::Arc;

use chrono::{DateTime, Duration, TimeZone, Utc};
use pretty_assertions::assert_eq;

use crate::{
    auth::{
        kernel::detail::test::MockChronoAuthClock,
        user::{
            kernel::detail::repository::memory::{
                login_id::MapLoginId, StoreLoginId, StoreResetToken,
            },
            password::reset::request_token::api::detail::test::MockResetTokenGenerator,
        },
    },
    common::api::feature::AsInfra,
};

use crate::auth::user::password::reset::request_token::action::RequestResetPasswordTokenAction;

use crate::auth::user::password::reset::request_token::infra::{
    RequestResetPasswordTokenConfig, RequestResetPasswordTokenFields,
};

use crate::auth::{
    kernel::data::ExpireDuration,
    user::{
        kernel::data::AuthUserId,
        login_id::kernel::data::LoginId,
        password::reset::{
            kernel::data::{
                ResetPasswordId, ResetPasswordTokenDestination, ResetPasswordTokenDestinationEmail,
            },
            request_token::data::RequestResetPasswordTokenError,
        },
    },
};

#[tokio::test]
async fn success() -> Result<(), RequestResetPasswordTokenError> {
    let feature = feature(Infra {
        now: Utc.with_ymd_and_hms(2021, 1, 1, 10, 0, 0).unwrap(),
        request_id: ResetPasswordId::restore("request-id".to_owned()),
        user: vec![(
            AuthUserId::restore("user-id".to_owned()),
            LoginId::restore("login-id".to_owned()),
            ResetPasswordTokenDestination::Email(ResetPasswordTokenDestinationEmail::restore(
                "destination@example.com".to_owned(),
            )),
        )],
        config: RequestResetPasswordTokenConfig {
            token_expires: ExpireDuration::with_duration(Duration::days(1)),
        },
    });
    let action = RequestResetPasswordTokenAction::mock(feature.as_infra());

    let fields = RequestResetPasswordTokenFields {
        login_id: LoginId::restore("login-id".to_owned()),
    };

    action.request(fields).await?;

    Ok(())
}

#[tokio::test]
async fn error_user_not_found() {
    let feature = feature(Infra {
        now: Utc.with_ymd_and_hms(2021, 1, 1, 10, 0, 0).unwrap(),
        request_id: ResetPasswordId::restore("request-id".to_owned()),
        user: vec![(
            AuthUserId::restore("user-id".to_owned()),
            LoginId::restore("login-id".to_owned()),
            ResetPasswordTokenDestination::Email(ResetPasswordTokenDestinationEmail::restore(
                "destination@example.com".to_owned(),
            )),
        )],
        config: RequestResetPasswordTokenConfig {
            token_expires: ExpireDuration::with_duration(Duration::days(1)),
        },
    });
    let action = RequestResetPasswordTokenAction::mock(feature.as_infra());

    let fields = RequestResetPasswordTokenFields {
        login_id: LoginId::restore("UNKNOWN-login-id".to_owned()),
    };

    let err = action.request(fields).await.unwrap_err();

    assert_eq!(
        format!("{}", err),
        format!("{}", RequestResetPasswordTokenError::NotFound),
    )
}

struct Infra {
    now: DateTime<Utc>,
    request_id: ResetPasswordId,
    user: Vec<(AuthUserId, LoginId, ResetPasswordTokenDestination)>,
    config: RequestResetPasswordTokenConfig,
}

fn feature(
    infra: Infra,
) -> (
    MockChronoAuthClock,
    Arc<StoreLoginId>,
    Arc<StoreResetToken>,
    MockResetTokenGenerator,
    RequestResetPasswordTokenConfig,
) {
    let login_id_store = Arc::new(StoreLoginId::default());

    for (user_id, login_id, reset_token_destination) in infra.user {
        MapLoginId::insert_entry(&login_id_store, login_id, user_id, reset_token_destination);
    }

    (
        MockChronoAuthClock::new(infra.now),
        login_id_store,
        Arc::new(StoreResetToken::default()),
        MockResetTokenGenerator::new(infra.request_id),
        infra.config,
    )
}

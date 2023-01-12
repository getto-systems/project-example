use chrono::{DateTime, Duration, TimeZone, Utc};
use pretty_assertions::assert_eq;

use getto_application_test::ApplicationActionStateHolder;

use crate::auth::{
    kernel::init::clock::test::StaticChronoAuthClock,
    user::{
        kernel::init::user_repository::memory::{MemoryAuthUserRepository, MemoryAuthUserStore},
        password::reset::request_token::init::test::{
            StaticRequestResetTokenFields, StaticRequestResetTokenMaterial,
            StaticResetTokenGenerator,
        },
    },
};

use crate::auth::user::password::reset::request_token::action::RequestResetTokenAction;

use crate::auth::user::password::reset::request_token::infra::{
    RequestResetPasswordTokenConfig, RequestResetPasswordTokenFields,
};

use crate::{
    auth::{
        kernel::data::ExpireDuration,
        user::{
            kernel::data::AuthUserId,
            login_id::kernel::data::{LoginId, ValidateLoginIdError},
            password::reset::kernel::data::{
                ResetPasswordId, ResetPasswordTokenDestination, ResetPasswordTokenDestinationEmail,
            },
        },
    },
    common::api::validate::data::ValidateTextError,
};

#[tokio::test]
async fn info() {
    let store = TestStore::new();
    let material = StaticRequestResetTokenMaterial::new(
        standard_clock(),
        standard_reset_token_repository(&store),
        standard_token_generator(),
        standard_request_token_config(),
    );

    let action = RequestResetTokenAction::with_material(material);

    assert_eq!(action.info.name(), "auth.user.password.reset.request-token");
}

#[tokio::test]
async fn success_request_token() {
    let holder = ApplicationActionStateHolder::new();

    let store = TestStore::new();
    let material = StaticRequestResetTokenMaterial::new(
        standard_clock(),
        standard_reset_token_repository(&store),
        standard_token_generator(),
        standard_request_token_config(),
    );
    let fields = StaticRequestResetTokenFields::Valid(RequestResetPasswordTokenFields {
        login_id: stored_login_id(),
    });

    let mut action = RequestResetTokenAction::with_material(material);
    action.subscribe(holder.handler());

    let result = action.ignite(fields).await;
    assert_eq!(
        holder.extract(),
        vec![
            "token expires calculated; 2021-01-02 10:00:00 UTC",
            "token notified; message-id: message-id",
            "request reset-token success",
        ],
    );
    assert!(result.is_ok());
}

#[tokio::test]
async fn error_invalid_login_id() {
    let holder = ApplicationActionStateHolder::new();

    let store = TestStore::new();
    let material = StaticRequestResetTokenMaterial::new(
        standard_clock(),
        standard_reset_token_repository(&store),
        standard_token_generator(),
        standard_request_token_config(),
    );
    let fields = StaticRequestResetTokenFields::Invalid(ValidateLoginIdError::LoginId(
        ValidateTextError::Empty,
    ));

    let mut action = RequestResetTokenAction::with_material(material);
    action.subscribe(holder.handler());

    let result = action.ignite(fields).await;
    assert_eq!(
        holder.extract(),
        vec!["request reset-token error; invalid; login-id: empty"],
    );
    assert!(result.is_err());
}

#[tokio::test]
async fn error_destination_not_stored() {
    let holder = ApplicationActionStateHolder::new();

    let store = TestStore::new();
    let material = StaticRequestResetTokenMaterial::new(
        standard_clock(),
        no_destination_reset_token_repository(&store),
        standard_token_generator(),
        standard_request_token_config(),
    );
    let fields = StaticRequestResetTokenFields::Valid(RequestResetPasswordTokenFields {
        login_id: stored_login_id(),
    });

    let mut action = RequestResetTokenAction::with_material(material);
    action.subscribe(holder.handler());

    let result = action.ignite(fields).await;
    assert_eq!(
        holder.extract(),
        vec!["request reset-token error; not found"],
    );
    assert!(result.is_err());
}

struct TestStore {
    reset_token: MemoryAuthUserStore,
}

impl TestStore {
    fn new() -> Self {
        Self {
            reset_token: MemoryAuthUserStore::new(),
        }
    }
}

fn standard_request_token_config() -> RequestResetPasswordTokenConfig {
    RequestResetPasswordTokenConfig {
        token_expires: ExpireDuration::with_duration(Duration::days(1)),
    }
}

fn standard_now() -> DateTime<Utc> {
    Utc.with_ymd_and_hms(2021, 1, 1, 10, 0, 0).latest().unwrap()
}
fn standard_clock() -> StaticChronoAuthClock {
    StaticChronoAuthClock::new(standard_now())
}

fn standard_token_generator() -> StaticResetTokenGenerator {
    StaticResetTokenGenerator::new(ResetPasswordId::restore("TOKEN".into()))
}

fn standard_reset_token_repository<'a>(store: &'a TestStore) -> MemoryAuthUserRepository<'a> {
    MemoryAuthUserRepository::with_user_id_and_destination(
        &store.reset_token,
        stored_login_id(),
        stored_user_id(),
        stored_destination(),
    )
}
fn no_destination_reset_token_repository<'a>(store: &'a TestStore) -> MemoryAuthUserRepository<'a> {
    MemoryAuthUserRepository::with_user_id(&store.reset_token, stored_login_id(), stored_user_id())
}

fn stored_user_id() -> AuthUserId {
    AuthUserId::restore("user-id".to_owned())
}
fn stored_login_id() -> LoginId {
    LoginId::restore("login-id".to_owned())
}
fn stored_destination() -> ResetPasswordTokenDestination {
    ResetPasswordTokenDestination::Email(ResetPasswordTokenDestinationEmail::restore(
        "user@example.com".to_owned(),
    ))
}

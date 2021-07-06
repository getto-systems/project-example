use chrono::{DateTime, Duration, TimeZone, Utc};

use getto_application_test::ActionTestRunner;

use crate::auth::{
    auth_ticket::_api::kernel::init::test::{
        MemoryAuthNonceMap, MemoryAuthNonceRepository, MemoryAuthNonceStore, StaticAuthNonceHeader,
        StaticCheckAuthNonceStruct, StaticChronoAuthClock,
    },
    password::{
        _api::kernel::init::test::{
            MemoryAuthUserPasswordMap, MemoryAuthUserPasswordRepository,
            MemoryAuthUserPasswordStore, StaticAuthUserPasswordStruct,
        },
        reset::_api::request_token::init::test::{
            MemoryResetTokenDestinationMap, MemoryResetTokenDestinationRepository,
            MemoryResetTokenDestinationStore, StaticRequestResetTokenMessenger,
            StaticRequestResetTokenStruct, StaticResetTokenEncoder, StaticResetTokenGenerator,
            StaticResetTokenNotifier,
        },
    },
};

use crate::auth::{
    auth_ticket::_api::kernel::infra::AuthNonceConfig,
    password::reset::_api::request_token::infra::{
        RequestResetTokenConfig, RequestResetTokenFieldsExtract,
    },
};

use super::action::{RequestResetTokenAction, RequestResetTokenMaterial};

use crate::auth::{
    auth_ticket::_api::kernel::data::{AuthDateTime, AuthNonceValue, ExpireDuration},
    auth_user::_api::kernel::data::AuthUserId,
    login_id::_api::data::LoginId,
    password::{
        _api::kernel::data::ResetToken,
        reset::_api::request_token::data::{ResetTokenDestination, ResetTokenDestinationExtract},
    },
};

#[tokio::test]
async fn success_request_token() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let feature = TestFeature::standard(&store);

    let mut action = RequestResetTokenAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "token expires calculated; 2021-01-02 10:00:00 UTC",
        "token notified; message-id: message-id",
        "request reset token success",
    ]);
    assert!(result.is_ok());
}

#[tokio::test]
async fn success_expired_nonce() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::expired_nonce();
    let feature = TestFeature::standard(&store);

    let mut action = RequestResetTokenAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "token expires calculated; 2021-01-02 10:00:00 UTC",
        "token notified; message-id: message-id",
        "request reset token success",
    ]);
    assert!(result.is_ok());
}

#[tokio::test]
async fn error_conflict_nonce() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::conflict_nonce();
    let feature = TestFeature::standard(&store);

    let mut action = RequestResetTokenAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "request reset token error; auth nonce error: conflict",
    ]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn error_empty_login_id() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let feature = TestFeature::empty_login_id(&store);

    let mut action = RequestResetTokenAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec!["request reset token error; empty login id"]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn error_too_long_login_id() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let feature = TestFeature::too_long_login_id(&store);

    let mut action = RequestResetTokenAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec!["request reset token error; too long login id"]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn just_max_length_login_id() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let feature = TestFeature::just_max_length_login_id(&store);

    let mut action = RequestResetTokenAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec!["request reset token error; destination not found"]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn error_destination_not_stored() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::destination_not_stored();
    let feature = TestFeature::standard(&store);

    let mut action = RequestResetTokenAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec!["request reset token error; destination not found"]);
    assert!(!result.is_ok());
}

struct TestFeature<'a> {
    request_token: StaticRequestResetTokenStruct<'a>,
}

impl<'a> RequestResetTokenMaterial for TestFeature<'a> {
    type RequestToken = StaticRequestResetTokenStruct<'a>;

    fn request_token(&self) -> &Self::RequestToken {
        &self.request_token
    }
}

struct TestStore {
    nonce: MemoryAuthNonceStore,
    password: MemoryAuthUserPasswordStore,
    destination: MemoryResetTokenDestinationStore,
}

impl TestStore {
    fn standard() -> Self {
        Self {
            nonce: standard_nonce_store(),
            password: standard_password_store(),
            destination: standard_destination_store(),
        }
    }
    fn expired_nonce() -> Self {
        Self {
            nonce: expired_nonce_store(),
            password: standard_password_store(),
            destination: standard_destination_store(),
        }
    }
    fn conflict_nonce() -> Self {
        Self {
            nonce: conflict_nonce_store(),
            password: standard_password_store(),
            destination: standard_destination_store(),
        }
    }
    fn destination_not_stored() -> Self {
        Self {
            nonce: standard_nonce_store(),
            password: standard_password_store(),
            destination: empty_destination_store(),
        }
    }
}

impl<'a> TestFeature<'a> {
    fn standard(store: &'a TestStore) -> Self {
        Self::with_messenger(store, standard_messenger())
    }
    fn empty_login_id(store: &'a TestStore) -> Self {
        Self::with_messenger(store, empty_login_id_messenger())
    }
    fn too_long_login_id(store: &'a TestStore) -> Self {
        Self::with_messenger(store, too_long_login_id_messenger())
    }
    fn just_max_length_login_id(store: &'a TestStore) -> Self {
        Self::with_messenger(store, just_max_length_login_id_messenger())
    }
    fn with_messenger(store: &'a TestStore, messenger: StaticRequestResetTokenMessenger) -> Self {
        Self {
            request_token: StaticRequestResetTokenStruct {
                check_nonce_infra: StaticCheckAuthNonceStruct {
                    config: standard_nonce_config(),
                    clock: standard_clock(),
                    nonce_header: standard_nonce_header(),
                    nonce_repository: MemoryAuthNonceRepository::new(&store.nonce),
                },
                destination_repository: MemoryResetTokenDestinationRepository::new(
                    &store.destination,
                ),
                password_infra: StaticAuthUserPasswordStruct {
                    password_repository: MemoryAuthUserPasswordRepository::new(&store.password),
                },
                token_generator: standard_token_generator(),
                token_encoder: StaticResetTokenEncoder::new(),
                token_notifier: StaticResetTokenNotifier::new(),
                messenger,
                config: standard_request_token_config(),
            },
        }
    }
}

const NONCE: &'static str = "nonce";
const USER_ID: &'static str = "user-id";
const LOGIN_ID: &'static str = "login-id";
const USER_EMAIL: &'static str = "user@example.com";
const RESET_TOKEN: &'static str = "reset-token";

fn standard_nonce_config() -> AuthNonceConfig {
    AuthNonceConfig {
        nonce_expires: ExpireDuration::with_duration(Duration::days(1)),
    }
}
fn standard_request_token_config() -> RequestResetTokenConfig {
    RequestResetTokenConfig {
        token_expires: ExpireDuration::with_duration(Duration::days(1)),
    }
}

fn standard_now() -> DateTime<Utc> {
    Utc.ymd(2021, 1, 1).and_hms(10, 0, 0)
}
fn standard_clock() -> StaticChronoAuthClock {
    StaticChronoAuthClock::new(standard_now())
}

fn standard_nonce_header() -> StaticAuthNonceHeader {
    StaticAuthNonceHeader::Valid(AuthNonceValue::new(NONCE.into()))
}

fn standard_token_generator() -> StaticResetTokenGenerator {
    StaticResetTokenGenerator::new(ResetToken::new(RESET_TOKEN.into()))
}

fn standard_messenger() -> StaticRequestResetTokenMessenger {
    StaticRequestResetTokenMessenger::new(RequestResetTokenFieldsExtract {
        login_id: "login-id".into(),
    })
}
fn empty_login_id_messenger() -> StaticRequestResetTokenMessenger {
    StaticRequestResetTokenMessenger::new(RequestResetTokenFieldsExtract {
        login_id: "".into(),
    })
}
fn too_long_login_id_messenger() -> StaticRequestResetTokenMessenger {
    StaticRequestResetTokenMessenger::new(RequestResetTokenFieldsExtract {
        login_id: vec!["a"; 100 + 1].join(""),
    })
}
fn just_max_length_login_id_messenger() -> StaticRequestResetTokenMessenger {
    StaticRequestResetTokenMessenger::new(RequestResetTokenFieldsExtract {
        login_id: vec!["a"; 100].join(""),
    })
}

fn standard_nonce_store() -> MemoryAuthNonceStore {
    MemoryAuthNonceMap::new().to_store()
}
fn expired_nonce_store() -> MemoryAuthNonceStore {
    let expires = AuthDateTime::restore(standard_now())
        .expires(&ExpireDuration::with_duration(Duration::days(-1)));
    MemoryAuthNonceMap::with_nonce(NONCE.into(), expires).to_store()
}
fn conflict_nonce_store() -> MemoryAuthNonceStore {
    let expires = AuthDateTime::restore(standard_now())
        .expires(&ExpireDuration::with_duration(Duration::days(1)));
    MemoryAuthNonceMap::with_nonce(NONCE.into(), expires).to_store()
}

fn standard_destination_store() -> MemoryResetTokenDestinationStore {
    MemoryResetTokenDestinationMap::with_destination(test_user_login_id(), test_user_destination())
        .to_store()
}
fn empty_destination_store() -> MemoryResetTokenDestinationStore {
    MemoryResetTokenDestinationMap::new().to_store()
}

fn standard_password_store() -> MemoryAuthUserPasswordStore {
    MemoryAuthUserPasswordMap::with_user_id(test_user_login_id(), test_user_id()).to_store()
}

fn test_user_id() -> AuthUserId {
    AuthUserId::restore(USER_ID.to_string())
}
fn test_user_login_id() -> LoginId {
    LoginId::validate(LOGIN_ID.to_string()).unwrap()
}
fn test_user_destination() -> ResetTokenDestination {
    ResetTokenDestinationExtract {
        email: USER_EMAIL.to_string(),
    }
    .into()
}

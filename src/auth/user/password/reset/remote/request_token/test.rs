use chrono::{DateTime, Duration, TimeZone, Utc};

use getto_application_test::ActionTestRunner;

use crate::auth::{
    ticket::remote::{
        check_nonce::init::{
            nonce_repository::test::{
                MemoryAuthNonceMap, MemoryAuthNonceRepository, MemoryAuthNonceStore,
            },
            test::StaticCheckAuthNonceStruct,
        },
        kernel::init::{
            clock::test::StaticChronoAuthClock, nonce_metadata::test::StaticAuthNonceMetadata,
        },
    },
    user::password::{
        remote::kernel::init::password_repository::test::{
            MemoryAuthUserPasswordMap, MemoryAuthUserPasswordRepository,
            MemoryAuthUserPasswordStore,
        },
        reset::remote::request_token::init::{
            destination_repository::test::{
                MemoryResetTokenDestinationMap, MemoryResetTokenDestinationRepository,
                MemoryResetTokenDestinationStore,
            },
            request_decoder::test::StaticRequestResetTokenRequestDecoder,
            token_encoder::test::StaticResetTokenEncoder,
            token_generator::test::StaticResetTokenGenerator,
            token_notifier::test::StaticResetTokenNotifier,
        },
    },
};

use crate::auth::{
    ticket::remote::check_nonce::infra::AuthNonceConfig,
    user::password::reset::remote::request_token::infra::{
        RequestResetTokenConfig, RequestResetTokenFieldsExtract,
    },
};

use super::action::{RequestResetTokenAction, RequestResetTokenMaterial};

use crate::auth::{
    ticket::remote::kernel::data::{AuthDateTime, ExpireDuration},
    user::{
        login_id::remote::data::LoginId,
        password::reset::remote::kernel::data::{
            ResetToken, ResetTokenDestination, ResetTokenDestinationExtract,
        },
        remote::kernel::data::AuthUserId,
    },
};

#[tokio::test]
async fn success_request_token() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let material = TestStruct::new(&store);
    let request_decoder = standard_request_decoder();

    let mut action = RequestResetTokenAction::with_material(request_decoder, material);
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
    let material = TestStruct::new(&store);
    let request_decoder = standard_request_decoder();

    let mut action = RequestResetTokenAction::with_material(request_decoder, material);
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
    let material = TestStruct::new(&store);
    let request_decoder = standard_request_decoder();

    let mut action = RequestResetTokenAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec!["auth nonce error: conflict"]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn error_empty_login_id() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let material = TestStruct::new(&store);
    let request_decoder = empty_login_id_request_decoder();

    let mut action = RequestResetTokenAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "request reset token error; invalid login id: empty login id",
    ]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn error_too_long_login_id() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let material = TestStruct::new(&store);
    let request_decoder = too_long_login_id_request_decoder();

    let mut action = RequestResetTokenAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "request reset token error; invalid login id: too long login id",
    ]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn just_max_length_login_id() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let material = TestStruct::new(&store);
    let request_decoder = just_max_length_login_id_request_decoder();

    let mut action = RequestResetTokenAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec!["request reset token error; destination not found"]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn error_destination_not_stored() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::destination_not_stored();
    let material = TestStruct::new(&store);
    let request_decoder = standard_request_decoder();

    let mut action = RequestResetTokenAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec!["request reset token error; destination not found"]);
    assert!(!result.is_ok());
}

struct TestStruct<'a> {
    pub check_nonce: StaticCheckAuthNonceStruct<'a>,

    pub clock: StaticChronoAuthClock,
    pub password_repository: MemoryAuthUserPasswordRepository<'a>,
    pub destination_repository: MemoryResetTokenDestinationRepository<'a>,
    pub token_generator: StaticResetTokenGenerator,
    pub token_encoder: StaticResetTokenEncoder,
    pub token_notifier: StaticResetTokenNotifier,
    pub config: RequestResetTokenConfig,
}

impl<'a> RequestResetTokenMaterial for TestStruct<'a> {
    type CheckNonce = StaticCheckAuthNonceStruct<'a>;

    type Clock = StaticChronoAuthClock;
    type PasswordRepository = MemoryAuthUserPasswordRepository<'a>;
    type DestinationRepository = MemoryResetTokenDestinationRepository<'a>;
    type TokenGenerator = StaticResetTokenGenerator;
    type TokenEncoder = StaticResetTokenEncoder;
    type TokenNotifier = StaticResetTokenNotifier;

    fn check_nonce(&self) -> &Self::CheckNonce {
        &self.check_nonce
    }

    fn clock(&self) -> &Self::Clock {
        &self.clock
    }
    fn password_repository(&self) -> &Self::PasswordRepository {
        &self.password_repository
    }
    fn destination_repository(&self) -> &Self::DestinationRepository {
        &self.destination_repository
    }
    fn token_generator(&self) -> &Self::TokenGenerator {
        &self.token_generator
    }
    fn token_encoder(&self) -> &Self::TokenEncoder {
        &self.token_encoder
    }
    fn token_notifier(&self) -> &Self::TokenNotifier {
        &self.token_notifier
    }
    fn config(&self) -> &RequestResetTokenConfig {
        &self.config
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

impl<'a> TestStruct<'a> {
    fn new(store: &'a TestStore) -> Self {
        Self {
            check_nonce: StaticCheckAuthNonceStruct {
                config: standard_nonce_config(),
                clock: standard_clock(),
                nonce_metadata: standard_nonce_metadata(),
                nonce_repository: MemoryAuthNonceRepository::new(&store.nonce),
            },
            clock: standard_clock(),
            password_repository: MemoryAuthUserPasswordRepository::new(&store.password),
            destination_repository: MemoryResetTokenDestinationRepository::new(&store.destination),
            token_generator: standard_token_generator(),
            token_encoder: StaticResetTokenEncoder,
            token_notifier: StaticResetTokenNotifier,
            config: standard_request_token_config(),
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

fn standard_nonce_metadata() -> StaticAuthNonceMetadata {
    StaticAuthNonceMetadata::new(NONCE.into())
}

fn standard_token_generator() -> StaticResetTokenGenerator {
    StaticResetTokenGenerator::new(ResetToken::new(RESET_TOKEN.into()))
}

fn standard_request_decoder() -> StaticRequestResetTokenRequestDecoder {
    StaticRequestResetTokenRequestDecoder {
        fields: RequestResetTokenFieldsExtract {
            login_id: "login-id".into(),
        },
    }
}
fn empty_login_id_request_decoder() -> StaticRequestResetTokenRequestDecoder {
    StaticRequestResetTokenRequestDecoder {
        fields: RequestResetTokenFieldsExtract {
            login_id: "".into(),
        },
    }
}
fn too_long_login_id_request_decoder() -> StaticRequestResetTokenRequestDecoder {
    StaticRequestResetTokenRequestDecoder {
        fields: RequestResetTokenFieldsExtract {
            login_id: vec!["a"; 100 + 1].join(""),
        },
    }
}
fn just_max_length_login_id_request_decoder() -> StaticRequestResetTokenRequestDecoder {
    StaticRequestResetTokenRequestDecoder {
        fields: RequestResetTokenFieldsExtract {
            login_id: vec!["a"; 100].join(""),
        },
    }
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
    AuthUserId::restore(USER_ID.into())
}
fn test_user_login_id() -> LoginId {
    LoginId::restore(LOGIN_ID.into())
}
fn test_user_destination() -> ResetTokenDestination {
    ResetTokenDestinationExtract {
        email: USER_EMAIL.into(),
    }
    .restore()
}

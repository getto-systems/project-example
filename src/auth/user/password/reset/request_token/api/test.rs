use chrono::{DateTime, Duration, TimeZone, Utc};

use getto_application_test::ActionTestRunner;

use crate::auth::{
    ticket::{
        kernel::init::clock::test::StaticChronoAuthClock,
        validate::init::{
            nonce_metadata::test::StaticAuthNonceMetadata,
            nonce_repository::memory::MemoryAuthNonceRepository,
            test::StaticValidateAuthNonceStruct,
        },
    },
    user::{
        kernel::init::user_repository::memory::MemoryAuthUserRepository,
        password::reset::request_token::init::{
            request_decoder::test::StaticRequestResetTokenRequestDecoder,
            token_encoder::test::StaticResetTokenEncoder,
            token_generator::test::StaticResetTokenGenerator,
            token_notifier::test::StaticResetTokenNotifier,
        },
    },
};

use crate::auth::ticket::validate::method::AuthNonceConfig;

use super::action::{RequestResetTokenAction, RequestResetTokenMaterial};

use crate::auth::user::password::reset::request_token::infra::{
    RequestResetTokenConfig, RequestResetTokenFieldsExtract,
};

use crate::auth::{
    ticket::kernel::data::ExpireDuration,
    user::{
        kernel::data::AuthUserId,
        login_id::kernel::data::LoginId,
        password::reset::kernel::data::{
            ResetToken, ResetTokenDestination, ResetTokenDestinationExtract,
        },
    },
};

#[tokio::test]
async fn success_request_token() {
    let (handler, assert_state) = ActionTestRunner::new();

    let repository = TestRepository::standard();
    let material = TestStruct::new(repository);
    let request_decoder = standard_request_decoder();

    let mut action = RequestResetTokenAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "token expires calculated; 2021-01-02 10:00:00 UTC",
        "token notified; message-id: message-id",
        "request reset token success",
    ]);
    assert!(result.is_ok());
}

#[tokio::test]
async fn error_empty_login_id() {
    let (handler, assert_state) = ActionTestRunner::new();

    let repository = TestRepository::standard();
    let material = TestStruct::new(repository);
    let request_decoder = empty_login_id_request_decoder();

    let mut action = RequestResetTokenAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "request reset token error; invalid; login-id: empty login id",
    ]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn error_too_long_login_id() {
    let (handler, assert_state) = ActionTestRunner::new();

    let repository = TestRepository::standard();
    let material = TestStruct::new(repository);
    let request_decoder = too_long_login_id_request_decoder();

    let mut action = RequestResetTokenAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "request reset token error; invalid; login-id: too long login id",
    ]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn just_max_length_login_id() {
    let (handler, assert_state) = ActionTestRunner::new();

    let repository = TestRepository::standard();
    let material = TestStruct::new(repository);
    let request_decoder = just_max_length_login_id_request_decoder();

    let mut action = RequestResetTokenAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "request reset token error; not found",
    ]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn error_destination_not_stored() {
    let (handler, assert_state) = ActionTestRunner::new();

    let repository = TestRepository::no_destination();
    let material = TestStruct::new(repository);
    let request_decoder = standard_request_decoder();

    let mut action = RequestResetTokenAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "request reset token error; not found",
    ]);
    assert!(!result.is_ok());
}

struct TestStruct {
    validate_nonce: StaticValidateAuthNonceStruct,

    clock: StaticChronoAuthClock,
    reset_token_repository: MemoryAuthUserRepository,
    token_generator: StaticResetTokenGenerator,
    token_encoder: StaticResetTokenEncoder,
    token_notifier: StaticResetTokenNotifier,
    config: RequestResetTokenConfig,
}

impl RequestResetTokenMaterial for TestStruct {
    type ValidateNonce = StaticValidateAuthNonceStruct;

    type Clock = StaticChronoAuthClock;
    type ResetTokenRepository = MemoryAuthUserRepository;
    type TokenGenerator = StaticResetTokenGenerator;
    type TokenEncoder = StaticResetTokenEncoder;
    type TokenNotifier = StaticResetTokenNotifier;

    fn validate_nonce(&self) -> &Self::ValidateNonce {
        &self.validate_nonce
    }

    fn clock(&self) -> &Self::Clock {
        &self.clock
    }
    fn reset_token_repository(&self) -> &Self::ResetTokenRepository {
        &self.reset_token_repository
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

struct TestRepository {
    reset_token: MemoryAuthUserRepository,
}

impl TestRepository {
    fn standard() -> Self {
        Self {
            reset_token: standard_reset_token_repository(),
        }
    }
    fn no_destination() -> Self {
        Self {
            reset_token: no_destination_reset_token_repository(),
        }
    }
}

impl TestStruct {
    fn new(repository: TestRepository) -> Self {
        Self {
            validate_nonce: StaticValidateAuthNonceStruct {
                config: standard_nonce_config(),
                clock: standard_clock(),
                nonce_metadata: standard_nonce_metadata(),
                nonce_repository: standard_nonce_repository(),
            },
            clock: standard_clock(),
            reset_token_repository: repository.reset_token,
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
    StaticResetTokenGenerator::new(ResetToken::restore(RESET_TOKEN.into()))
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

fn standard_nonce_repository() -> MemoryAuthNonceRepository {
    MemoryAuthNonceRepository::new()
}

fn standard_reset_token_repository() -> MemoryAuthUserRepository {
    MemoryAuthUserRepository::with_user_id_and_destination(
        test_user_login_id(),
        test_user_id(),
        test_user_destination(),
    )
}
fn no_destination_reset_token_repository() -> MemoryAuthUserRepository {
    MemoryAuthUserRepository::with_user_id(test_user_login_id(), test_user_id())
}

fn test_user_id() -> AuthUserId {
    AuthUserId::restore(USER_ID.into())
}
fn test_user_login_id() -> LoginId {
    LoginId::restore(LOGIN_ID.into())
}
fn test_user_destination() -> ResetTokenDestination {
    ResetTokenDestination::restore(ResetTokenDestinationExtract::Email(USER_EMAIL.into()))
}

use std::collections::HashSet;

use chrono::{DateTime, Duration, TimeZone, Utc};
use getto_application_test::ActionTestRunner;

use crate::auth::{
    ticket::{
        kernel::init::clock::test::StaticChronoAuthClock,
        validate::init::{
            nonce_metadata::test::StaticAuthNonceMetadata,
            nonce_repository::memory::MemoryAuthNonceRepository,
            test::{StaticValidateAuthNonceStruct, StaticValidateAuthTokenStruct},
            token_decoder::test::StaticAuthTokenDecoder,
            token_metadata::test::StaticAuthTokenMetadata,
        },
    },
    user::{
        kernel::init::user_repository::memory::MemoryAuthUserRepository,
        login_id::change::init::request_decoder::test::StaticOverrideLoginIdRequestDecoder,
    },
};

use crate::auth::user::login_id::change::action::{OverrideLoginIdAction, OverrideLoginIdMaterial};

use crate::auth::ticket::validate::method::AuthNonceConfig;

use crate::auth::user::{
    login_id::change::infra::OverrideLoginIdFieldsExtract, password::kernel::infra::HashedPassword,
};

use crate::auth::{
    ticket::kernel::data::{AuthTicketExtract, ExpireDuration},
    user::{
        kernel::data::{AuthUser, AuthUserExtract, AuthUserId},
        login_id::kernel::data::LoginId,
    },
};

#[tokio::test]
async fn success_override() {
    let (handler, assert_state) = ActionTestRunner::new();

    let material = TestStruct::standard();
    let request_decoder = standard_request_decoder();

    let mut action = OverrideLoginIdAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "validate success; ticket: ticket-id / user: user-id (granted: [])",
        "override login-id success",
    ]);
    assert!(result.is_ok());
}

#[tokio::test]
async fn error_empty_login_id() {
    let (handler, assert_state) = ActionTestRunner::new();

    let material = TestStruct::standard();
    let request_decoder = empty_new_login_id_request_decoder();

    let mut action = OverrideLoginIdAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "validate success; ticket: ticket-id / user: user-id (granted: [])",
        "override login-id error; invalid; new: empty login id",
    ]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn error_too_long_login_id() {
    let (handler, assert_state) = ActionTestRunner::new();

    let material = TestStruct::standard();
    let request_decoder = too_long_new_login_id_request_decoder();

    let mut action = OverrideLoginIdAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "validate success; ticket: ticket-id / user: user-id (granted: [])",
        "override login-id error; invalid; new: too long login id",
    ]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn just_max_length_login_id() {
    let (handler, assert_state) = ActionTestRunner::new();

    let material = TestStruct::standard();
    let request_decoder = just_max_length_new_login_id_request_decoder();

    let mut action = OverrideLoginIdAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "validate success; ticket: ticket-id / user: user-id (granted: [])",
        "override login-id success",
    ]);
    assert!(result.is_ok());
}

#[tokio::test]
async fn error_login_id_already_registered() {
    let (handler, assert_state) = ActionTestRunner::new();

    let material = TestStruct::standard();
    let request_decoder = already_registered_login_id_request_decoder();

    let mut action = OverrideLoginIdAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "validate success; ticket: ticket-id / user: user-id (granted: [])",
        "override login-id error; new login id is already registered",
    ]);
    assert!(!result.is_ok());
}

struct TestStruct {
    validate: StaticValidateAuthTokenStruct,
    login_id_repository: MemoryAuthUserRepository,
}

impl OverrideLoginIdMaterial for TestStruct {
    type Validate = StaticValidateAuthTokenStruct;

    type LoginIdRepository = MemoryAuthUserRepository;

    fn validate(&self) -> &Self::Validate {
        &self.validate
    }
    fn login_id_repository(&self) -> &Self::LoginIdRepository {
        &self.login_id_repository
    }
}

impl TestStruct {
    fn standard() -> Self {
        Self {
            validate: StaticValidateAuthTokenStruct {
                validate_nonce: StaticValidateAuthNonceStruct {
                    config: standard_nonce_config(),
                    clock: standard_clock(),
                    nonce_metadata: standard_nonce_header(),
                    nonce_repository: standard_nonce_repository(),
                },
                token_metadata: standard_token_header(),
                token_decoder: standard_token_decoder(),
            },
            login_id_repository: standard_login_id_repository(),
        }
    }
}

fn standard_nonce_config() -> AuthNonceConfig {
    AuthNonceConfig {
        nonce_expires: ExpireDuration::with_duration(Duration::days(1)),
    }
}

fn standard_now() -> DateTime<Utc> {
    Utc.ymd(2021, 1, 1).and_hms(10, 0, 0)
}
fn standard_clock() -> StaticChronoAuthClock {
    StaticChronoAuthClock::new(standard_now())
}

fn standard_nonce_header() -> StaticAuthNonceMetadata {
    StaticAuthNonceMetadata::new(NONCE.into())
}
fn standard_token_header() -> StaticAuthTokenMetadata {
    StaticAuthTokenMetadata::new("TOKEN".into())
}

fn standard_token_decoder() -> StaticAuthTokenDecoder {
    StaticAuthTokenDecoder::Valid(AuthTicketExtract {
        ticket_id: TICKET_ID.into(),
        user_id: USER_ID.into(),
        granted_roles: HashSet::new(),
    })
}

const NONCE: &'static str = "nonce";
const TICKET_ID: &'static str = "ticket-id";
const USER_ID: &'static str = "user-id";
const LOGIN_ID: &'static str = "login-id";
const REGISTERED_USER_ID: &'static str = "registered-user-id";
const REGISTERED_LOGIN_ID: &'static str = "registered-login-id";
const PASSWORD: &'static str = "current-password";

fn standard_request_decoder() -> StaticOverrideLoginIdRequestDecoder {
    StaticOverrideLoginIdRequestDecoder::Valid(OverrideLoginIdFieldsExtract {
        login_id: LOGIN_ID.into(),
        new_login_id: "new-login-id".into(),
    })
}
fn empty_new_login_id_request_decoder() -> StaticOverrideLoginIdRequestDecoder {
    StaticOverrideLoginIdRequestDecoder::Valid(OverrideLoginIdFieldsExtract {
        login_id: LOGIN_ID.into(),
        new_login_id: "".into(),
    })
}
fn too_long_new_login_id_request_decoder() -> StaticOverrideLoginIdRequestDecoder {
    StaticOverrideLoginIdRequestDecoder::Valid(OverrideLoginIdFieldsExtract {
        login_id: LOGIN_ID.into(),
        new_login_id: vec!["a"; 100 + 1].join(""),
    })
}
fn just_max_length_new_login_id_request_decoder() -> StaticOverrideLoginIdRequestDecoder {
    StaticOverrideLoginIdRequestDecoder::Valid(OverrideLoginIdFieldsExtract {
        login_id: LOGIN_ID.into(),
        new_login_id: vec!["a"; 100].join(""),
    })
}
fn already_registered_login_id_request_decoder() -> StaticOverrideLoginIdRequestDecoder {
    StaticOverrideLoginIdRequestDecoder::Valid(OverrideLoginIdFieldsExtract {
        login_id: LOGIN_ID.into(),
        new_login_id: REGISTERED_LOGIN_ID.into(),
    })
}

fn standard_nonce_repository() -> MemoryAuthNonceRepository {
    MemoryAuthNonceRepository::new()
}

fn standard_login_id_repository() -> MemoryAuthUserRepository {
    MemoryAuthUserRepository::with_user_and_password(
        test_user_login_id(),
        test_user(),
        test_user_password(),
        vec![(test_registered_login_id(), test_registered_user_id())],
    )
}

fn test_user() -> AuthUser {
    let mut granted_roles = HashSet::new();
    granted_roles.insert("something".into());

    AuthUserExtract {
        user_id: USER_ID.into(),
        granted_roles,
    }
    .restore()
}
fn test_user_login_id() -> LoginId {
    LoginId::restore(LOGIN_ID.into())
}
fn test_user_password() -> HashedPassword {
    HashedPassword::restore(PASSWORD.into())
}

fn test_registered_user_id() -> AuthUserId {
    AuthUserId::restore(REGISTERED_USER_ID.into())
}
fn test_registered_login_id() -> LoginId {
    LoginId::restore(REGISTERED_LOGIN_ID.into())
}

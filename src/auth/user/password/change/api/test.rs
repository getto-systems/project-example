use std::collections::HashSet;

use chrono::{DateTime, Duration, TimeZone, Utc};
use getto_application_test::ActionTestRunner;

use crate::auth::{
    ticket::{
        kernel::init::clock::test::StaticChronoAuthClock,
        validate::init::{
            nonce_metadata::test::StaticAuthNonceMetadata,
            nonce_repository::memory::{
                MemoryAuthNonceMap, MemoryAuthNonceRepository, MemoryAuthNonceStore,
            },
            test::{StaticValidateAuthNonceStruct, StaticValidateAuthTokenStruct},
            token_decoder::test::StaticAuthTokenDecoder,
            token_metadata::test::StaticAuthTokenMetadata,
        },
    },
    user::{
        kernel::init::user_repository::memory::MemoryAuthUserRepository,
        password::{
            change::init::request_decoder::test::StaticChangePasswordRequestDecoder,
            kernel::init::{
                password_hasher::test::PlainPasswordHasher,
                password_matcher::test::PlainPasswordMatcher,
            },
        },
    },
};

use crate::auth::user::password::{
    change::infra::ChangePasswordFieldsExtract, kernel::infra::HashedPassword,
};

use crate::auth::ticket::validate::method::AuthNonceConfig;

use super::action::{ChangePasswordAction, ChangePasswordMaterial};

use crate::auth::{
    ticket::kernel::data::{AuthDateTime, AuthTicketExtract, ExpireDuration},
    user::{
        kernel::data::{AuthUser, AuthUserExtract},
        login_id::kernel::data::LoginId,
    },
};

#[tokio::test]
async fn success_change() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let repository = TestRepository::standard();
    let material = TestStruct::standard(&store, repository);
    let request_decoder = standard_request_decoder();

    let mut action = ChangePasswordAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "validate success; ticket: ticket-id / user: user-id (granted: [])",
        "change password success",
    ]);
    assert!(result.is_ok());
}

#[tokio::test]
async fn success_expired_nonce() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::expired_nonce();
    let repository = TestRepository::standard();
    let material = TestStruct::standard(&store, repository);
    let request_decoder = standard_request_decoder();

    let mut action = ChangePasswordAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "validate success; ticket: ticket-id / user: user-id (granted: [])",
        "change password success",
    ]);
    assert!(result.is_ok());
}

#[tokio::test]
async fn error_conflict_nonce() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::conflict_nonce();
    let repository = TestRepository::standard();
    let material = TestStruct::standard(&store, repository);
    let request_decoder = standard_request_decoder();

    let mut action = ChangePasswordAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce error; conflict",
    ]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn error_empty_current_password() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let repository = TestRepository::standard();
    let material = TestStruct::standard(&store, repository);
    let request_decoder = empty_current_password_request_decoder();

    let mut action = ChangePasswordAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "validate success; ticket: ticket-id / user: user-id (granted: [])",
        "change password error; invalid; current: empty password",
    ]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn error_too_long_current_password() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let repository = TestRepository::standard();
    let material = TestStruct::standard(&store, repository);
    let request_decoder = too_long_current_password_request_decoder();

    let mut action = ChangePasswordAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "validate success; ticket: ticket-id / user: user-id (granted: [])",
        "change password error; invalid; current: too long password",
    ]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn just_max_length_current_password() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let repository = TestRepository::standard();
    let material = TestStruct::standard(&store, repository);
    let request_decoder = just_max_length_current_password_request_decoder();

    let mut action = ChangePasswordAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "validate success; ticket: ticket-id / user: user-id (granted: [])",
        "change password error; password not matched",
    ]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn error_empty_new_password() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let repository = TestRepository::standard();
    let material = TestStruct::standard(&store, repository);
    let request_decoder = empty_new_password_request_decoder();

    let mut action = ChangePasswordAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "validate success; ticket: ticket-id / user: user-id (granted: [])",
        "change password error; invalid; new: empty password",
    ]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn error_too_long_new_password() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let repository = TestRepository::standard();
    let material = TestStruct::standard(&store, repository);
    let request_decoder = too_long_new_password_request_decoder();

    let mut action = ChangePasswordAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "validate success; ticket: ticket-id / user: user-id (granted: [])",
        "change password error; invalid; new: too long password",
    ]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn just_max_length_new_password() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let repository = TestRepository::standard();
    let material = TestStruct::standard(&store, repository);
    let request_decoder = just_max_length_new_password_request_decoder();

    let mut action = ChangePasswordAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "validate success; ticket: ticket-id / user: user-id (granted: [])",
        "change password success",
    ]);
    assert!(result.is_ok());
}

#[tokio::test]
async fn error_failed_to_match_password() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let repository = TestRepository::match_fail();
    let material = TestStruct::standard(&store, repository);
    let request_decoder = standard_request_decoder();

    let mut action = ChangePasswordAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "validate success; ticket: ticket-id / user: user-id (granted: [])",
        "change password error; password not matched",
    ]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn error_password_not_stored() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let repository = TestRepository::no_user();
    let material = TestStruct::standard(&store, repository);
    let request_decoder = standard_request_decoder();

    let mut action = ChangePasswordAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "validate success; ticket: ticket-id / user: user-id (granted: [])",
        "change password error; not found",
    ]);
    assert!(!result.is_ok());
}

struct TestStruct<'a> {
    validate: StaticValidateAuthTokenStruct<'a>,
    password_repository: MemoryAuthUserRepository,
}

impl<'a> ChangePasswordMaterial for TestStruct<'a> {
    type Validate = StaticValidateAuthTokenStruct<'a>;

    type PasswordRepository = MemoryAuthUserRepository;
    type PasswordMatcher = PlainPasswordMatcher;
    type PasswordHasher = PlainPasswordHasher;

    fn validate(&self) -> &Self::Validate {
        &self.validate
    }
    fn password_repository(&self) -> &Self::PasswordRepository {
        &self.password_repository
    }
}

struct TestStore {
    nonce: MemoryAuthNonceStore,
}

impl TestStore {
    fn standard() -> Self {
        Self {
            nonce: standard_nonce_store(),
        }
    }
    fn expired_nonce() -> Self {
        Self {
            nonce: expired_nonce_store(),
        }
    }
    fn conflict_nonce() -> Self {
        Self {
            nonce: conflict_nonce_store(),
        }
    }
}

struct TestRepository {
    password: MemoryAuthUserRepository,
}

impl TestRepository {
    fn standard() -> Self {
        Self {
            password: standard_password_repository(),
        }
    }
    fn match_fail() -> Self {
        Self {
            password: match_fail_password_repository(),
        }
    }
    fn no_user() -> Self {
        Self {
            password: no_user_password_repository(),
        }
    }
}

impl<'a> TestStruct<'a> {
    fn standard(store: &'a TestStore, repository: TestRepository) -> Self {
        Self {
            validate: StaticValidateAuthTokenStruct {
                validate_nonce: StaticValidateAuthNonceStruct {
                    config: standard_nonce_config(),
                    clock: standard_clock(),
                    nonce_metadata: standard_nonce_header(),
                    nonce_repository: MemoryAuthNonceRepository::new(&store.nonce),
                },
                token_metadata: standard_token_header(),
                token_decoder: standard_token_decoder(),
            },
            password_repository: repository.password,
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
const PASSWORD: &'static str = "current-password";
const ANOTHER_PASSWORD: &'static str = "another-password";

fn standard_request_decoder() -> StaticChangePasswordRequestDecoder {
    StaticChangePasswordRequestDecoder::Valid(ChangePasswordFieldsExtract {
        current_password: "current-password".into(),
        new_password: "new-password".into(),
    })
}
fn empty_current_password_request_decoder() -> StaticChangePasswordRequestDecoder {
    StaticChangePasswordRequestDecoder::Valid(ChangePasswordFieldsExtract {
        current_password: "".into(),
        new_password: "new-password".into(),
    })
}
fn too_long_current_password_request_decoder() -> StaticChangePasswordRequestDecoder {
    StaticChangePasswordRequestDecoder::Valid(ChangePasswordFieldsExtract {
        current_password: vec!["a"; 100 + 1].join(""),
        new_password: "new-password".into(),
    })
}
fn just_max_length_current_password_request_decoder() -> StaticChangePasswordRequestDecoder {
    StaticChangePasswordRequestDecoder::Valid(ChangePasswordFieldsExtract {
        current_password: vec!["a"; 100].join(""),
        new_password: "new-password".into(),
    })
}
fn empty_new_password_request_decoder() -> StaticChangePasswordRequestDecoder {
    StaticChangePasswordRequestDecoder::Valid(ChangePasswordFieldsExtract {
        current_password: "current-password".into(),
        new_password: "".into(),
    })
}
fn too_long_new_password_request_decoder() -> StaticChangePasswordRequestDecoder {
    StaticChangePasswordRequestDecoder::Valid(ChangePasswordFieldsExtract {
        current_password: "current-password".into(),
        new_password: vec!["a"; 100 + 1].join(""),
    })
}
fn just_max_length_new_password_request_decoder() -> StaticChangePasswordRequestDecoder {
    StaticChangePasswordRequestDecoder::Valid(ChangePasswordFieldsExtract {
        current_password: "current-password".into(),
        new_password: vec!["a"; 100].join(""),
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

fn standard_password_repository() -> MemoryAuthUserRepository {
    MemoryAuthUserRepository::with_user_and_password(
        test_user_login_id(),
        test_user(),
        test_user_password(),
        vec![],
    )
}
fn match_fail_password_repository() -> MemoryAuthUserRepository {
    MemoryAuthUserRepository::with_user_and_password(
        test_user_login_id(),
        test_user(),
        another_password(),
        vec![],
    )
}
fn no_user_password_repository() -> MemoryAuthUserRepository {
    MemoryAuthUserRepository::new()
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
fn another_password() -> HashedPassword {
    HashedPassword::restore(ANOTHER_PASSWORD.into())
}

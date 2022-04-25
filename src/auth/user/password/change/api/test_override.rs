use std::collections::HashSet;

use chrono::{DateTime, Duration, TimeZone, Utc};
use getto_application_test::ActionTestRunner;

use crate::auth::{
    ticket::{
        kernel::init::clock::test::StaticChronoAuthClock,
        validate::init::{
            nonce_metadata::test::StaticAuthNonceMetadata,
            nonce_repository::memory::{MemoryAuthNonceRepository, MemoryAuthNonceStore},
            test::{StaticValidateAuthNonceStruct, StaticValidateAuthTokenStruct},
            token_decoder::test::StaticAuthTokenDecoder,
            token_metadata::test::StaticAuthTokenMetadata,
        },
    },
    user::{
        kernel::init::user_repository::memory::{MemoryAuthUserRepository, MemoryAuthUserStore},
        password::{
            change::init::request_decoder::test::StaticOverridePasswordRequestDecoder,
            kernel::init::password_hasher::test::PlainPasswordHasher,
        },
    },
};

use crate::auth::user::password::change::action::{
    OverridePasswordAction, OverridePasswordMaterial,
};

use crate::auth::ticket::validate::method::AuthNonceConfig;

use crate::auth::user::password::{
    change::infra::OverridePasswordFieldsExtract, kernel::infra::HashedPassword,
};

use crate::auth::{
    ticket::kernel::data::{AuthTicketExtract, ExpireDuration},
    user::{
        kernel::data::{AuthUser, AuthUserExtract},
        login_id::kernel::data::LoginId,
    },
};

#[tokio::test]
async fn success_override() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::new();
    let material = TestStruct::standard(&store);
    let request_decoder = standard_request_decoder();

    let mut action = OverridePasswordAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "validate success; ticket: ticket-id / user: user-id (granted: [])",
        "override password success",
    ]);
    assert!(result.is_ok());
}

#[tokio::test]
async fn error_empty_password() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::new();
    let material = TestStruct::standard(&store);
    let request_decoder = empty_password_request_decoder();

    let mut action = OverridePasswordAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "validate success; ticket: ticket-id / user: user-id (granted: [])",
        "override password error; invalid; new-password: empty",
    ]);
    assert!(result.is_err());
}

#[tokio::test]
async fn error_too_long_password() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::new();
    let material = TestStruct::standard(&store);
    let request_decoder = too_long_password_request_decoder();

    let mut action = OverridePasswordAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "validate success; ticket: ticket-id / user: user-id (granted: [])",
        "override password error; invalid; new-password: too long",
    ]);
    assert!(result.is_err());
}

#[tokio::test]
async fn just_max_length_password() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::new();
    let material = TestStruct::standard(&store);
    let request_decoder = just_max_length_password_request_decoder();

    let mut action = OverridePasswordAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "validate success; ticket: ticket-id / user: user-id (granted: [])",
        "override password success",
    ]);
    assert!(result.is_ok());
}

struct TestStruct<'a> {
    validate: StaticValidateAuthTokenStruct<'a>,
    password_repository: MemoryAuthUserRepository<'a>,
}

impl<'a> OverridePasswordMaterial for TestStruct<'a> {
    type Validate = StaticValidateAuthTokenStruct<'a>;

    type PasswordRepository = MemoryAuthUserRepository<'a>;
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
    password: MemoryAuthUserStore,
}

impl TestStore {
    fn new() -> Self {
        Self {
            nonce: MemoryAuthNonceStore::new(),
            password: MemoryAuthUserStore::new(),
        }
    }
}

impl<'a> TestStruct<'a> {
    fn standard(store: &'a TestStore) -> Self {
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
            password_repository: standard_password_repository(&store.password),
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

fn standard_request_decoder() -> StaticOverridePasswordRequestDecoder {
    StaticOverridePasswordRequestDecoder::Valid(OverridePasswordFieldsExtract {
        login_id: LOGIN_ID.into(),
        new_password: "new-password".into(),
    })
}
fn empty_password_request_decoder() -> StaticOverridePasswordRequestDecoder {
    StaticOverridePasswordRequestDecoder::Valid(OverridePasswordFieldsExtract {
        login_id: LOGIN_ID.into(),
        new_password: "".into(),
    })
}
fn too_long_password_request_decoder() -> StaticOverridePasswordRequestDecoder {
    StaticOverridePasswordRequestDecoder::Valid(OverridePasswordFieldsExtract {
        login_id: LOGIN_ID.into(),
        new_password: vec!["a"; 100 + 1].join(""),
    })
}
fn just_max_length_password_request_decoder() -> StaticOverridePasswordRequestDecoder {
    StaticOverridePasswordRequestDecoder::Valid(OverridePasswordFieldsExtract {
        login_id: LOGIN_ID.into(),
        new_password: vec!["a"; 100].join(""),
    })
}

fn standard_password_repository<'a>(
    store: &'a MemoryAuthUserStore,
) -> MemoryAuthUserRepository<'a> {
    MemoryAuthUserRepository::with_user_and_password(
        store,
        test_user_login_id(),
        test_user(),
        test_user_password(),
        vec![],
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

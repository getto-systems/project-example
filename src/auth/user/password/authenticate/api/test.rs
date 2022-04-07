use std::collections::HashSet;

use chrono::{DateTime, Duration, TimeZone, Utc};

use getto_application_test::ActionTestRunner;

use crate::auth::{
    ticket::{
        encode::init::{
            test::StaticEncodeAuthTicketStruct,
            token_encoder::test::{StaticAuthTokenEncoder, StaticCloudfrontTokenEncoder},
        },
        issue::init::{
            id_generator::test::StaticAuthTicketIdGenerator, test::StaticIssueAuthTicketStruct,
        },
        kernel::init::{
            clock::test::StaticChronoAuthClock,
            ticket_repository::memory::{
                MemoryAuthTicketMap, MemoryAuthTicketRepository, MemoryAuthTicketStore,
            },
        },
        validate::init::{
            nonce_metadata::test::StaticAuthNonceMetadata,
            nonce_repository::memory::{
                MemoryAuthNonceMap, MemoryAuthNonceRepository, MemoryAuthNonceStore,
            },
            test::StaticValidateAuthNonceStruct,
        },
    },
    user::{
        kernel::init::user_repository::memory::MemoryAuthUserRepository,
        password::{
            authenticate::init::request_decoder::test::StaticAuthenticatePasswordRequestDecoder,
            kernel::init::password_matcher::test::PlainPasswordMatcher,
        },
    },
};

use crate::auth::ticket::{
    encode::method::EncodeAuthTicketConfig, issue::method::IssueAuthTicketConfig,
    validate::method::AuthNonceConfig,
};

use crate::auth::user::password::{
    authenticate::infra::AuthenticatePasswordFieldsExtract, kernel::infra::HashedPassword,
};

use super::action::{AuthenticatePasswordAction, AuthenticatePasswordMaterial};

use crate::auth::{
    ticket::kernel::data::{AuthDateTime, AuthTicketId, ExpansionLimitDuration, ExpireDuration},
    user::{
        kernel::data::{AuthUser, AuthUserExtract},
        login_id::kernel::data::LoginId,
    },
};

#[tokio::test]
async fn success_authenticate() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let repository = TestRepository::standard();
    let material = TestStruct::new(&store, repository);
    let request_decoder = standard_request_decoder();

    let mut action = AuthenticatePasswordAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "authenticate password success; user: test-user-id (granted: [user])",
        "expansion limit calculated; 2021-01-11 10:00:00 UTC",
        "issue success; ticket: ticket-id / user: test-user-id (granted: [user])",
        "token expires calculated; ticket: 2021-01-02 10:00:00 UTC / api: 2021-01-01 10:01:00 UTC / cloudfront: 2021-01-01 10:01:00 UTC",
        "encode success",
    ]);
    assert!(result.is_ok());
}

#[tokio::test]
async fn success_expired_nonce() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::expired_nonce();
    let repository = TestRepository::standard();
    let material = TestStruct::new(&store, repository);
    let request_decoder = standard_request_decoder();

    let mut action = AuthenticatePasswordAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "authenticate password success; user: test-user-id (granted: [user])",
        "expansion limit calculated; 2021-01-11 10:00:00 UTC",
        "issue success; ticket: ticket-id / user: test-user-id (granted: [user])",
        "token expires calculated; ticket: 2021-01-02 10:00:00 UTC / api: 2021-01-01 10:01:00 UTC / cloudfront: 2021-01-01 10:01:00 UTC",
        "encode success",
    ]);
    assert!(result.is_ok());
}

#[tokio::test]
async fn error_conflict_nonce() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::conflict_nonce();
    let repository = TestRepository::standard();
    let material = TestStruct::new(&store, repository);
    let request_decoder = standard_request_decoder();

    let mut action = AuthenticatePasswordAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce error; conflict",
    ]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn error_empty_login_id() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let repository = TestRepository::standard();
    let material = TestStruct::new(&store, repository);
    let request_decoder = empty_login_id_request_decoder();

    let mut action = AuthenticatePasswordAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "authenticate password error; invalid; login-id: empty login id",
    ]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn error_too_long_login_id() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let repository = TestRepository::standard();
    let material = TestStruct::new(&store, repository);
    let request_decoder = too_long_login_id_request_decoder();

    let mut action = AuthenticatePasswordAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "authenticate password error; invalid; login-id: too long login id",
    ]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn just_max_length_login_id() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let repository = TestRepository::standard();
    let material = TestStruct::new(&store, repository);
    let request_decoder = just_max_length_login_id_request_decoder();

    let mut action = AuthenticatePasswordAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "authenticate password error; not found",
    ]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn error_empty_password() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let repository = TestRepository::standard();
    let material = TestStruct::new(&store, repository);
    let request_decoder = empty_password_request_decoder();

    let mut action = AuthenticatePasswordAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "authenticate password error; invalid; password: empty password",
    ]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn error_too_long_password() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let repository = TestRepository::standard();
    let material = TestStruct::new(&store, repository);
    let request_decoder = too_long_password_request_decoder();

    let mut action = AuthenticatePasswordAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "authenticate password error; invalid; password: too long password",
    ]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn just_max_length_password() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let repository = TestRepository::standard();
    let material = TestStruct::new(&store, repository);
    let request_decoder = just_max_length_password_request_decoder();

    let mut action = AuthenticatePasswordAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "authenticate password error; password not matched",
    ]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn error_failed_to_match_password() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let repository = TestRepository::match_fail();
    let material = TestStruct::new(&store, repository);
    let request_decoder = standard_request_decoder();

    let mut action = AuthenticatePasswordAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "authenticate password error; password not matched",
    ]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn error_no_user() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let repository = TestRepository::no_user();
    let material = TestStruct::new(&store, repository);
    let request_decoder = standard_request_decoder();

    let mut action = AuthenticatePasswordAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "authenticate password error; not found",
    ]);
    assert!(!result.is_ok());
}

struct TestStruct<'a> {
    validate_nonce: StaticValidateAuthNonceStruct<'a>,
    issue: StaticIssueAuthTicketStruct<'a>,
    encode: StaticEncodeAuthTicketStruct<'a>,

    password_repository: MemoryAuthUserRepository,
}

impl<'a> AuthenticatePasswordMaterial for TestStruct<'a> {
    type ValidateNonce = StaticValidateAuthNonceStruct<'a>;
    type Issue = StaticIssueAuthTicketStruct<'a>;
    type Encode = StaticEncodeAuthTicketStruct<'a>;

    type PasswordRepository = MemoryAuthUserRepository;
    type PasswordMatcher = PlainPasswordMatcher;

    fn validate_nonce(&self) -> &Self::ValidateNonce {
        &self.validate_nonce
    }
    fn issue(&self) -> &Self::Issue {
        &self.issue
    }
    fn encode(&self) -> &Self::Encode {
        &self.encode
    }

    fn password_repository(&self) -> &Self::PasswordRepository {
        &self.password_repository
    }
}

struct TestStore {
    nonce: MemoryAuthNonceStore,
    ticket: MemoryAuthTicketStore,
}

impl TestStore {
    fn standard() -> Self {
        Self {
            nonce: standard_nonce_store(),
            ticket: standard_ticket_store(),
        }
    }
    fn expired_nonce() -> Self {
        Self {
            nonce: expired_nonce_store(),
            ticket: standard_ticket_store(),
        }
    }
    fn conflict_nonce() -> Self {
        Self {
            nonce: conflict_nonce_store(),
            ticket: standard_ticket_store(),
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
            password: empty_password_repository(),
        }
    }
}

impl<'a> TestStruct<'a> {
    fn new(store: &'a TestStore, repository: TestRepository) -> Self {
        Self {
            validate_nonce: StaticValidateAuthNonceStruct {
                config: standard_nonce_config(),
                clock: standard_clock(),
                nonce_metadata: standard_nonce_metadata(),
                nonce_repository: MemoryAuthNonceRepository::new(&store.nonce),
            },
            issue: StaticIssueAuthTicketStruct {
                clock: standard_clock(),
                ticket_repository: MemoryAuthTicketRepository::new(&store.ticket),
                ticket_id_generator: StaticAuthTicketIdGenerator::new(AuthTicketId::new(
                    "ticket-id".into(),
                )),
                config: standard_issue_config(),
            },
            encode: StaticEncodeAuthTicketStruct {
                clock: standard_clock(),
                ticket_repository: MemoryAuthTicketRepository::new(&store.ticket),
                ticket_encoder: StaticAuthTokenEncoder,
                api_encoder: StaticAuthTokenEncoder,
                cloudfront_encoder: StaticCloudfrontTokenEncoder,
                config: standard_encode_config(),
            },

            password_repository: repository.password,
        }
    }
}

const NONCE: &'static str = "nonce";
const LOGIN_ID: &'static str = "login-id";
const PASSWORD: &'static str = "password";
const ANOTHER_PASSWORD: &'static str = "another-password";

fn standard_nonce_config() -> AuthNonceConfig {
    AuthNonceConfig {
        nonce_expires: ExpireDuration::with_duration(Duration::days(1)),
    }
}
fn standard_encode_config() -> EncodeAuthTicketConfig {
    EncodeAuthTicketConfig {
        ticket_expires: ExpireDuration::with_duration(Duration::days(1)),
        api_expires: ExpireDuration::with_duration(Duration::minutes(1)),
        cloudfront_expires: ExpireDuration::with_duration(Duration::minutes(1)),
    }
}
fn standard_issue_config() -> IssueAuthTicketConfig {
    IssueAuthTicketConfig {
        ticket_expansion_limit: ExpansionLimitDuration::with_duration(Duration::days(10)),
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

fn standard_request_decoder() -> StaticAuthenticatePasswordRequestDecoder {
    StaticAuthenticatePasswordRequestDecoder::Valid(AuthenticatePasswordFieldsExtract {
        login_id: "login-id".into(),
        password: "password".into(),
    })
}
fn empty_login_id_request_decoder() -> StaticAuthenticatePasswordRequestDecoder {
    StaticAuthenticatePasswordRequestDecoder::Valid(AuthenticatePasswordFieldsExtract {
        login_id: "".into(),
        password: "password".into(),
    })
}
fn too_long_login_id_request_decoder() -> StaticAuthenticatePasswordRequestDecoder {
    StaticAuthenticatePasswordRequestDecoder::Valid(AuthenticatePasswordFieldsExtract {
        login_id: vec!["a"; 100 + 1].join(""),
        password: "password".into(),
    })
}
fn just_max_length_login_id_request_decoder() -> StaticAuthenticatePasswordRequestDecoder {
    StaticAuthenticatePasswordRequestDecoder::Valid(AuthenticatePasswordFieldsExtract {
        login_id: vec!["a"; 100].join(""),
        password: "password".into(),
    })
}
fn empty_password_request_decoder() -> StaticAuthenticatePasswordRequestDecoder {
    StaticAuthenticatePasswordRequestDecoder::Valid(AuthenticatePasswordFieldsExtract {
        login_id: "login-id".into(),
        password: "".into(),
    })
}
fn too_long_password_request_decoder() -> StaticAuthenticatePasswordRequestDecoder {
    StaticAuthenticatePasswordRequestDecoder::Valid(AuthenticatePasswordFieldsExtract {
        login_id: "login-id".into(),
        password: vec!["a"; 100 + 1].join(""),
    })
}
fn just_max_length_password_request_decoder() -> StaticAuthenticatePasswordRequestDecoder {
    StaticAuthenticatePasswordRequestDecoder::Valid(AuthenticatePasswordFieldsExtract {
        login_id: "login-id".into(),
        password: vec!["a"; 100].join(""),
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

fn standard_ticket_store() -> MemoryAuthTicketStore {
    MemoryAuthTicketMap::new().to_store()
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
fn empty_password_repository() -> MemoryAuthUserRepository {
    MemoryAuthUserRepository::new()
}

fn test_user() -> AuthUser {
    let mut granted_roles = HashSet::new();
    granted_roles.insert("user".into());

    AuthUserExtract {
        user_id: "test-user-id".into(),
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

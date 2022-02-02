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
        kernel::remote::init::{
            clock::test::StaticChronoAuthClock,
            ticket_repository::test::{
                MemoryAuthTicketMap, MemoryAuthTicketRepository, MemoryAuthTicketStore,
            },
        },
        validate::init::{
            nonce_metadata::test::StaticAuthNonceMetadata,
            nonce_repository::test::{
                MemoryAuthNonceMap, MemoryAuthNonceRepository, MemoryAuthNonceStore,
            },
            test::StaticValidateAuthNonceStruct,
        },
    },
    user::{
        kernel::init::user_repository::test::{
            MemoryAuthUserMap, MemoryAuthUserRepository, MemoryAuthUserStore,
        },
        password::{
            authenticate::remote::init::request_decoder::test::StaticAuthenticatePasswordRequestDecoder,
            kernel::init::{
                password_matcher::test::PlainPasswordMatcher,
                password_repository::test::{
                    MemoryAuthUserPasswordMap, MemoryAuthUserPasswordRepository,
                    MemoryAuthUserPasswordStore,
                },
            },
        },
    },
};

use crate::auth::{
    ticket::{
        encode::method::EncodeAuthTicketConfig, issue::method::IssueAuthTicketConfig,
        validate::method::AuthNonceConfig,
    },
    user::password::{
        authenticate::remote::infra::AuthenticatePasswordFieldsExtract,
        kernel::infra::HashedPassword,
    },
};

use super::action::{AuthenticatePasswordAction, AuthenticatePasswordMaterial};

use crate::auth::{
    ticket::kernel::remote::data::{
        AuthDateTime, AuthTicketId, ExpansionLimitDuration, ExpireDuration,
    },
    user::{
        kernel::data::{AuthUser, AuthUserExtract},
        login_id::kernel::data::LoginId,
    },
};

#[tokio::test]
async fn success_authenticate() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let material = TestStruct::new(&store);
    let request_decoder = standard_request_decoder();

    let mut action = AuthenticatePasswordAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "authenticate password success; user: test-user-id (granted: [something])",
        "expansion limit calculated; 2021-01-11 10:00:00 UTC",
        "issue success; ticket: ticket-id / user: test-user-id (granted: [something])",
        "token expires calculated; ticket: 2021-01-02 10:00:00 UTC / api: 2021-01-01 10:01:00 UTC / cloudfront: 2021-01-01 10:01:00 UTC",
        "encode success",
    ]);
    assert!(result.is_ok());
}

#[tokio::test]
async fn success_expired_nonce() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::expired_nonce();
    let material = TestStruct::new(&store);
    let request_decoder = standard_request_decoder();

    let mut action = AuthenticatePasswordAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "authenticate password success; user: test-user-id (granted: [something])",
        "expansion limit calculated; 2021-01-11 10:00:00 UTC",
        "issue success; ticket: ticket-id / user: test-user-id (granted: [something])",
        "token expires calculated; ticket: 2021-01-02 10:00:00 UTC / api: 2021-01-01 10:01:00 UTC / cloudfront: 2021-01-01 10:01:00 UTC",
        "encode success",
    ]);
    assert!(result.is_ok());
}

#[tokio::test]
async fn error_conflict_nonce() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::conflict_nonce();
    let material = TestStruct::new(&store);
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
    let material = TestStruct::new(&store);
    let request_decoder = empty_login_id_request_decoder();

    let mut action = AuthenticatePasswordAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "authenticate password error; invalid login id: empty login id",
    ]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn error_too_long_login_id() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let material = TestStruct::new(&store);
    let request_decoder = too_long_login_id_request_decoder();

    let mut action = AuthenticatePasswordAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "authenticate password error; invalid login id: too long login id",
    ]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn just_max_length_login_id() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let material = TestStruct::new(&store);
    let request_decoder = just_max_length_login_id_request_decoder();

    let mut action = AuthenticatePasswordAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "authenticate password error; password not found",
    ]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn error_empty_password() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let material = TestStruct::new(&store);
    let request_decoder = empty_password_request_decoder();

    let mut action = AuthenticatePasswordAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "authenticate password error; invalid password: empty password",
    ]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn error_too_long_password() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let material = TestStruct::new(&store);
    let request_decoder = too_long_password_request_decoder();

    let mut action = AuthenticatePasswordAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "authenticate password error; invalid password: too long password",
    ]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn just_max_length_password() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let material = TestStruct::new(&store);
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

    let store = TestStore::match_fail_password();
    let material = TestStruct::new(&store);
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
async fn error_password_not_stored() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::password_not_stored();
    let material = TestStruct::new(&store);
    let request_decoder = standard_request_decoder();

    let mut action = AuthenticatePasswordAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "authenticate password error; password not found",
    ]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn error_user_not_stored() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::user_not_stored();
    let material = TestStruct::new(&store);
    let request_decoder = standard_request_decoder();

    let mut action = AuthenticatePasswordAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "authenticate password error; user not found",
    ]);
    assert!(!result.is_ok());
}

struct TestStruct<'a> {
    validate_nonce: StaticValidateAuthNonceStruct<'a>,
    issue: StaticIssueAuthTicketStruct<'a>,
    encode: StaticEncodeAuthTicketStruct<'a>,

    user_repository: MemoryAuthUserRepository<'a>,
    password_repository: MemoryAuthUserPasswordRepository<'a>,
}

impl<'a> AuthenticatePasswordMaterial for TestStruct<'a> {
    type ValidateNonce = StaticValidateAuthNonceStruct<'a>;
    type Issue = StaticIssueAuthTicketStruct<'a>;
    type Encode = StaticEncodeAuthTicketStruct<'a>;

    type UserRepository = MemoryAuthUserRepository<'a>;
    type PasswordRepository = MemoryAuthUserPasswordRepository<'a>;
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

    fn user_repository(&self) -> &Self::UserRepository {
        &self.user_repository
    }
    fn password_repository(&self) -> &Self::PasswordRepository {
        &self.password_repository
    }
}

struct TestStore {
    nonce: MemoryAuthNonceStore,
    ticket: MemoryAuthTicketStore,
    password: MemoryAuthUserPasswordStore,
    user: MemoryAuthUserStore,
}

impl TestStore {
    fn standard() -> Self {
        Self {
            nonce: standard_nonce_store(),
            ticket: standard_ticket_store(),
            password: standard_password_store(),
            user: standard_user_store(),
        }
    }
    fn expired_nonce() -> Self {
        Self {
            nonce: expired_nonce_store(),
            ticket: standard_ticket_store(),
            password: standard_password_store(),
            user: standard_user_store(),
        }
    }
    fn conflict_nonce() -> Self {
        Self {
            nonce: conflict_nonce_store(),
            ticket: standard_ticket_store(),
            password: standard_password_store(),
            user: standard_user_store(),
        }
    }
    fn match_fail_password() -> Self {
        Self {
            nonce: standard_nonce_store(),
            ticket: standard_ticket_store(),
            password: match_fail_password_store(),
            user: standard_user_store(),
        }
    }
    fn password_not_stored() -> Self {
        Self {
            nonce: standard_nonce_store(),
            ticket: standard_ticket_store(),
            password: not_stored_password_store(),
            user: standard_user_store(),
        }
    }
    fn user_not_stored() -> Self {
        Self {
            nonce: standard_nonce_store(),
            ticket: standard_ticket_store(),
            password: standard_password_store(),
            user: not_stored_user_store(),
        }
    }
}

impl<'a> TestStruct<'a> {
    fn new(store: &'a TestStore) -> Self {
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

            user_repository: MemoryAuthUserRepository::new(&store.user),
            password_repository: MemoryAuthUserPasswordRepository::new(&store.password),
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

fn standard_password_store() -> MemoryAuthUserPasswordStore {
    MemoryAuthUserPasswordMap::with_password(
        test_user_login_id(),
        test_user(),
        test_user_password(),
    )
    .to_store()
}
fn match_fail_password_store() -> MemoryAuthUserPasswordStore {
    MemoryAuthUserPasswordMap::with_password(test_user_login_id(), test_user(), another_password())
        .to_store()
}
fn not_stored_password_store() -> MemoryAuthUserPasswordStore {
    MemoryAuthUserPasswordMap::new().to_store()
}

fn standard_user_store() -> MemoryAuthUserStore {
    MemoryAuthUserMap::with_user(test_user()).to_store()
}
fn not_stored_user_store() -> MemoryAuthUserStore {
    MemoryAuthUserMap::new().to_store()
}

fn test_user() -> AuthUser {
    let mut granted_roles = HashSet::new();
    granted_roles.insert("something".into());

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
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
            ticket_repository::memory::{MemoryAuthTicketRepository, MemoryAuthTicketStore},
        },
        validate::init::{
            nonce_metadata::test::StaticAuthNonceMetadata,
            nonce_repository::memory::{MemoryAuthNonceRepository, MemoryAuthNonceStore},
            test::StaticValidateAuthNonceStruct,
        },
    },
    user::{
        kernel::init::user_repository::memory::{MemoryAuthUserRepository, MemoryAuthUserStore},
        password::{
            kernel::init::password_hasher::test::PlainPasswordHasher,
            reset::reset::init::{
                request_decoder::test::StaticResetPasswordRequestDecoder,
                reset_notifier::test::StaticResetPasswordNotifier,
                token_decoder::test::StaticResetTokenDecoder,
            },
        },
    },
};

use crate::auth::ticket::{
    encode::method::EncodeAuthTicketConfig, issue::method::IssueAuthTicketConfig,
    validate::method::AuthNonceConfig,
};

use super::action::{ResetPasswordAction, ResetPasswordMaterial};

use crate::auth::user::password::reset::reset::infra::ResetPasswordFieldsExtract;

use crate::auth::{
    ticket::kernel::data::{AuthDateTime, AuthTicketId, ExpansionLimitDuration, ExpireDuration},
    user::{
        kernel::data::{AuthUser, AuthUserExtract},
        login_id::kernel::data::LoginId,
        password::reset::kernel::data::{
            ResetToken, ResetTokenDestination, ResetTokenDestinationExtract,
        },
    },
};

#[tokio::test]
async fn success_request_token() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let material = TestStruct::standard(&store);
    let request_decoder = standard_request_decoder();

    let mut action = ResetPasswordAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "reset password notified; message-id: message-id",
        "reset password success; user: user-id (granted: [])",
        "expansion limit calculated; 2021-01-11 10:00:00 UTC",
        "issue success; ticket: ticket-id / user: user-id (granted: [])",
        "token expires calculated; ticket: 2021-01-02 10:00:00 UTC / api: 2021-01-01 10:01:00 UTC / cloudfront: 2021-01-01 10:01:00 UTC",
        "encode success",
    ]);
    assert!(result.is_ok());
}

#[tokio::test]
async fn error_match_failed_login_id() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let material = TestStruct::standard(&store);
    let request_decoder = match_failed_login_id_request_decoder();

    let mut action = ResetPasswordAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "reset password error; login id not matched",
    ]);
    assert!(result.is_err());
}

#[tokio::test]
async fn error_empty_login_id() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let material = TestStruct::standard(&store);
    let request_decoder = empty_login_id_request_decoder();

    let mut action = ResetPasswordAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "reset password error; invalid; login-id: empty",
    ]);
    assert!(result.is_err());
}

#[tokio::test]
async fn error_too_long_login_id() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let material = TestStruct::standard(&store);
    let request_decoder = too_long_login_id_request_decoder();

    let mut action = ResetPasswordAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "reset password error; invalid; login-id: too long",
    ]);
    assert!(result.is_err());
}

#[tokio::test]
async fn just_max_length_login_id() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let material = TestStruct::standard(&store);
    let request_decoder = just_max_length_login_id_request_decoder();

    let mut action = ResetPasswordAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "reset password error; login id not matched",
    ]);
    assert!(result.is_err());
}

#[tokio::test]
async fn error_empty_password() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let material = TestStruct::standard(&store);
    let request_decoder = empty_password_request_decoder();

    let mut action = ResetPasswordAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "reset password error; invalid; new-password: empty password",
    ]);
    assert!(result.is_err());
}

#[tokio::test]
async fn error_too_long_password() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let material = TestStruct::standard(&store);
    let request_decoder = too_long_password_request_decoder();

    let mut action = ResetPasswordAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "reset password error; invalid; new-password: too long password",
    ]);
    assert!(result.is_err());
}

#[tokio::test]
async fn just_max_length_password() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let material = TestStruct::standard(&store);
    let request_decoder = just_max_length_password_request_decoder();

    let mut action = ResetPasswordAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "reset password notified; message-id: message-id",
        "reset password success; user: user-id (granted: [])",
        "expansion limit calculated; 2021-01-11 10:00:00 UTC",
        "issue success; ticket: ticket-id / user: user-id (granted: [])",
        "token expires calculated; ticket: 2021-01-02 10:00:00 UTC / api: 2021-01-01 10:01:00 UTC / cloudfront: 2021-01-01 10:01:00 UTC",
        "encode success",
    ]);
    assert!(result.is_ok());
}

#[tokio::test]
async fn error_empty_reset_token() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let material = TestStruct::standard(&store);
    let request_decoder = empty_reset_token_request_decoder();

    let mut action = ResetPasswordAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "reset password error; invalid; reset-token: empty reset token",
    ]);
    assert!(result.is_err());
}

#[tokio::test]
async fn error_reset_token_expired_at_decode() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let material = TestStruct::expired_reset_token_at_decode(&store);
    let request_decoder = standard_request_decoder();

    let mut action = ResetPasswordAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "reset password error; reset token expired",
    ]);
    assert!(result.is_err());
}

#[tokio::test]
async fn error_reset_token_expired_in_store() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let material = TestStruct::expired_reset_token_in_store(&store);
    let request_decoder = standard_request_decoder();

    let mut action = ResetPasswordAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "reset password error; reset token expired",
    ]);
    assert!(result.is_err());
}

#[tokio::test]
async fn error_reset_token_discarded() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let material = TestStruct::already_reset(&store);
    let request_decoder = standard_request_decoder();

    let mut action = ResetPasswordAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "reset password error; already reset",
    ]);
    assert!(result.is_err());
}

#[tokio::test]
async fn error_reset_token_not_stored() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let material = TestStruct::no_reset_token(&store);
    let request_decoder = standard_request_decoder();

    let mut action = ResetPasswordAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "reset password error; not found",
    ]);
    assert!(result.is_err());
}

struct TestStruct<'a> {
    validate_nonce: StaticValidateAuthNonceStruct<'a>,
    issue: StaticIssueAuthTicketStruct<'a>,
    encode: StaticEncodeAuthTicketStruct<'a>,

    clock: StaticChronoAuthClock,
    reset_password_repository: MemoryAuthUserRepository<'a>,
    token_decoder: StaticResetTokenDecoder,
    reset_notifier: StaticResetPasswordNotifier,
}

impl<'a> ResetPasswordMaterial for TestStruct<'a> {
    type ValidateNonce = StaticValidateAuthNonceStruct<'a>;
    type Issue = StaticIssueAuthTicketStruct<'a>;
    type Encode = StaticEncodeAuthTicketStruct<'a>;

    type Clock = StaticChronoAuthClock;
    type ResetPasswordRepository = MemoryAuthUserRepository<'a>;
    type PasswordHasher = PlainPasswordHasher;
    type TokenDecoder = StaticResetTokenDecoder;
    type ResetNotifier = StaticResetPasswordNotifier;

    fn validate_nonce(&self) -> &Self::ValidateNonce {
        &self.validate_nonce
    }
    fn issue(&self) -> &Self::Issue {
        &self.issue
    }
    fn encode(&self) -> &Self::Encode {
        &self.encode
    }

    fn clock(&self) -> &Self::Clock {
        &self.clock
    }
    fn reset_password_repository(&self) -> &Self::ResetPasswordRepository {
        &self.reset_password_repository
    }
    fn token_decoder(&self) -> &Self::TokenDecoder {
        &self.token_decoder
    }
    fn reset_notifier(&self) -> &Self::ResetNotifier {
        &self.reset_notifier
    }
}

struct TestStore {
    nonce: MemoryAuthNonceStore,
    ticket: MemoryAuthTicketStore,
    reset_password: MemoryAuthUserStore,
}

impl TestStore {
    fn standard() -> Self {
        Self {
            nonce: MemoryAuthNonceStore::new(),
            ticket: MemoryAuthTicketStore::new(),
            reset_password: MemoryAuthUserStore::new(),
        }
    }
}

impl<'a> TestStruct<'a> {
    fn standard(store: &'a TestStore) -> Self {
        Self::new(
            store,
            standard_reset_token_repository(&store.reset_password),
            standard_reset_token_decoder(),
        )
    }
    fn expired_reset_token_at_decode(store: &'a TestStore) -> Self {
        Self::new(
            store,
            standard_reset_token_repository(&store.reset_password),
            expired_reset_token_decoder(),
        )
    }
    fn expired_reset_token_in_store(store: &'a TestStore) -> Self {
        Self::new(
            store,
            expired_reset_token_repository(&store.reset_password),
            standard_reset_token_decoder(),
        )
    }
    fn already_reset(store: &'a TestStore) -> Self {
        Self::new(
            store,
            already_reset_token_repository(&store.reset_password),
            standard_reset_token_decoder(),
        )
    }
    fn no_reset_token(store: &'a TestStore) -> Self {
        Self::new(
            store,
            no_reset_token_repository(&store.reset_password),
            standard_reset_token_decoder(),
        )
    }

    fn new(
        store: &'a TestStore,
        reset_password_repository: MemoryAuthUserRepository<'a>,
        token_decoder: StaticResetTokenDecoder,
    ) -> Self {
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
                ticket_id_generator: StaticAuthTicketIdGenerator::new(AuthTicketId::restore(
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

            clock: standard_clock(),
            reset_password_repository,
            token_decoder,
            reset_notifier: StaticResetPasswordNotifier,
        }
    }
}

const NONCE: &'static str = "nonce";
const USER_ID: &'static str = "user-id";
const LOGIN_ID: &'static str = "login-id";
const RESET_TOKEN: &'static str = "reset-token";
const EMAIL: &'static str = "email@example.com";

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

fn standard_request_decoder() -> StaticResetPasswordRequestDecoder {
    StaticResetPasswordRequestDecoder {
        fields: ResetPasswordFieldsExtract {
            reset_token: RESET_TOKEN.into(),
            login_id: LOGIN_ID.into(),
            new_password: "password".into(),
        },
    }
}
fn match_failed_login_id_request_decoder() -> StaticResetPasswordRequestDecoder {
    StaticResetPasswordRequestDecoder {
        fields: ResetPasswordFieldsExtract {
            reset_token: RESET_TOKEN.into(),
            login_id: "unknown-login-id".into(),
            new_password: "password".into(),
        },
    }
}
fn empty_login_id_request_decoder() -> StaticResetPasswordRequestDecoder {
    StaticResetPasswordRequestDecoder {
        fields: ResetPasswordFieldsExtract {
            reset_token: RESET_TOKEN.into(),
            login_id: "".into(),
            new_password: "password".into(),
        },
    }
}
fn too_long_login_id_request_decoder() -> StaticResetPasswordRequestDecoder {
    StaticResetPasswordRequestDecoder {
        fields: ResetPasswordFieldsExtract {
            reset_token: RESET_TOKEN.into(),
            login_id: vec!["a"; 100 + 1].join(""),
            new_password: "password".into(),
        },
    }
}
fn just_max_length_login_id_request_decoder() -> StaticResetPasswordRequestDecoder {
    StaticResetPasswordRequestDecoder {
        fields: ResetPasswordFieldsExtract {
            reset_token: RESET_TOKEN.into(),
            login_id: vec!["a"; 100].join(""),
            new_password: "password".into(),
        },
    }
}
fn empty_password_request_decoder() -> StaticResetPasswordRequestDecoder {
    StaticResetPasswordRequestDecoder {
        fields: ResetPasswordFieldsExtract {
            reset_token: RESET_TOKEN.into(),
            login_id: LOGIN_ID.into(),
            new_password: "".into(),
        },
    }
}
fn too_long_password_request_decoder() -> StaticResetPasswordRequestDecoder {
    StaticResetPasswordRequestDecoder {
        fields: ResetPasswordFieldsExtract {
            reset_token: RESET_TOKEN.into(),
            login_id: LOGIN_ID.into(),
            new_password: vec!["a"; 100 + 1].join(""),
        },
    }
}
fn just_max_length_password_request_decoder() -> StaticResetPasswordRequestDecoder {
    StaticResetPasswordRequestDecoder {
        fields: ResetPasswordFieldsExtract {
            reset_token: RESET_TOKEN.into(),
            login_id: LOGIN_ID.into(),
            new_password: vec!["a"; 100].join(""),
        },
    }
}
fn empty_reset_token_request_decoder() -> StaticResetPasswordRequestDecoder {
    StaticResetPasswordRequestDecoder {
        fields: ResetPasswordFieldsExtract {
            reset_token: "".into(),
            login_id: LOGIN_ID.into(),
            new_password: "password".into(),
        },
    }
}

fn standard_reset_token_decoder() -> StaticResetTokenDecoder {
    StaticResetTokenDecoder::Valid(ResetToken::restore(RESET_TOKEN.into()))
}
fn expired_reset_token_decoder() -> StaticResetTokenDecoder {
    StaticResetTokenDecoder::Expired
}

fn standard_reset_token_repository<'a>(
    store: &'a MemoryAuthUserStore,
) -> MemoryAuthUserRepository<'a> {
    let reset_token = ResetToken::restore(RESET_TOKEN.into());
    let destination =
        ResetTokenDestination::restore(ResetTokenDestinationExtract::Email(EMAIL.into()));
    let requested_at = AuthDateTime::restore(standard_now());
    let expires = requested_at.expires(&ExpireDuration::with_duration(Duration::days(1)));
    MemoryAuthUserRepository::with_user_and_reset_token(
        store,
        test_user_login_id(),
        test_user(),
        reset_token,
        destination,
        expires,
        requested_at,
        None,
    )
}
fn no_reset_token_repository<'a>(store: &'a MemoryAuthUserStore) -> MemoryAuthUserRepository<'a> {
    MemoryAuthUserRepository::new(store)
}
fn expired_reset_token_repository<'a>(
    store: &'a MemoryAuthUserStore,
) -> MemoryAuthUserRepository<'a> {
    let reset_token = ResetToken::restore(RESET_TOKEN.into());
    let destination =
        ResetTokenDestination::restore(ResetTokenDestinationExtract::Email(EMAIL.into()));
    let requested_at = AuthDateTime::restore(standard_now());
    let expires = requested_at.expires(&ExpireDuration::with_duration(Duration::days(-1)));
    MemoryAuthUserRepository::with_user_and_reset_token(
        store,
        test_user_login_id(),
        test_user(),
        reset_token,
        destination,
        expires,
        requested_at,
        None,
    )
}
fn already_reset_token_repository<'a>(
    store: &'a MemoryAuthUserStore,
) -> MemoryAuthUserRepository<'a> {
    let reset_token = ResetToken::restore(RESET_TOKEN.into());
    let destination =
        ResetTokenDestination::restore(ResetTokenDestinationExtract::Email(EMAIL.into()));
    let requested_at = AuthDateTime::restore(standard_now());
    let expires = requested_at.expires(&ExpireDuration::with_duration(Duration::days(1)));
    let reset_at = AuthDateTime::restore(standard_now() - Duration::days(1));
    MemoryAuthUserRepository::with_user_and_reset_token(
        store,
        test_user_login_id(),
        test_user(),
        reset_token,
        destination,
        expires,
        requested_at,
        Some(reset_at),
    )
}

fn test_user() -> AuthUser {
    AuthUserExtract {
        user_id: USER_ID.into(),
        granted_roles: HashSet::new(),
    }
    .restore()
}
fn test_user_login_id() -> LoginId {
    LoginId::restore(LOGIN_ID.into())
}

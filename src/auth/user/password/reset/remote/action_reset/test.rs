use std::collections::HashSet;

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
        encode::init::{
            test::StaticEncodeAuthTicketStruct,
            token_encoder::test::{StaticAuthTokenEncoder, StaticCloudfrontTokenEncoder},
        },
        issue::init::{
            id_generator::test::StaticAuthTicketIdGenerator, test::StaticIssueAuthTicketStruct,
        },
        kernel::init::{
            clock::test::StaticChronoAuthClock,
            nonce_metadata::test::StaticAuthNonceMetadata,
            ticket_repository::test::{
                MemoryAuthTicketMap, MemoryAuthTicketRepository, MemoryAuthTicketStore,
            },
        },
    },
    user::{
        password::{
            remote::kernel::init::password_repository::test::{
                MemoryAuthUserPasswordMap, MemoryAuthUserPasswordRepository,
                MemoryAuthUserPasswordStore,
            },
            reset::remote::reset::init::{
                request_decoder::test::StaticResetPasswordRequestDecoder,
                test::StaticResetPasswordStruct, token_decoder::test::StaticResetTokenDecoder,
            },
        },
        remote::kernel::init::user_repository::test::{
            MemoryAuthUserMap, MemoryAuthUserRepository, MemoryAuthUserStore,
        },
    },
};

use crate::auth::{
    ticket::remote::{
        check_nonce::infra::AuthNonceConfig, encode::infra::EncodeAuthTicketConfig,
        issue::infra::IssueAuthTicketConfig,
    },
    user::password::reset::remote::proxy_reset::infra::ResetPasswordFieldsExtract,
};

use super::action::{ResetPasswordAction, ResetPasswordMaterial};

use crate::auth::{
    ticket::remote::kernel::data::{
        AuthDateTime, AuthTicketId, ExpansionLimitDuration, ExpireDuration,
    },
    user::{
        login_id::remote::data::LoginId,
        password::remote::kernel::data::ResetToken,
        remote::kernel::data::{AuthUser, AuthUserExtract, AuthUserId},
    },
};

#[tokio::test]
async fn success_request_token() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let feature = TestFeature::standard(&store);
    let request_decoder = standard_request_decoder();

    let mut action = ResetPasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite(request_decoder).await;
    assert_state(vec![
        "reset password success; user: user-id (granted: [])",
        "expansion limit calculated; 2021-01-11 10:00:00 UTC",
        "issue success; ticket: ticket-id / user: user-id (granted: [])",
        "token expires calculated; ticket: 2021-01-02 10:00:00 UTC / api: 2021-01-01 10:01:00 UTC / cloudfront: 2021-01-01 10:01:00 UTC",
        "encode success",
    ]);
    assert!(result.is_ok());
}

#[tokio::test]
async fn success_expired_nonce() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::expired_nonce();
    let feature = TestFeature::standard(&store);
    let request_decoder = standard_request_decoder();

    let mut action = ResetPasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite(request_decoder).await;
    assert_state(vec![
        "reset password success; user: user-id (granted: [])",
        "expansion limit calculated; 2021-01-11 10:00:00 UTC",
        "issue success; ticket: ticket-id / user: user-id (granted: [])",
        "token expires calculated; ticket: 2021-01-02 10:00:00 UTC / api: 2021-01-01 10:01:00 UTC / cloudfront: 2021-01-01 10:01:00 UTC",
        "encode success",
    ]);
    assert!(result.is_ok());
}

#[tokio::test]
async fn error_conflict_nonce() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::conflict_nonce();
    let feature = TestFeature::standard(&store);
    let request_decoder = standard_request_decoder();

    let mut action = ResetPasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite(request_decoder).await;
    assert_state(vec!["reset password error; auth nonce error: conflict"]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn error_match_failed_login_id() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let feature = TestFeature::standard(&store);
    let request_decoder = match_failed_login_id_request_decoder();

    let mut action = ResetPasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite(request_decoder).await;
    assert_state(vec![
        "reset password error; invalid reset token entry: login id not matched",
    ]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn error_empty_login_id() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let feature = TestFeature::standard(&store);
    let request_decoder = empty_login_id_request_decoder();

    let mut action = ResetPasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite(request_decoder).await;
    assert_state(vec![
        "reset password error; invalid login id: empty login id",
    ]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn error_too_long_login_id() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let feature = TestFeature::standard(&store);
    let request_decoder = too_long_login_id_request_decoder();

    let mut action = ResetPasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite(request_decoder).await;
    assert_state(vec![
        "reset password error; invalid login id: too long login id",
    ]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn just_max_length_login_id() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let feature = TestFeature::standard(&store);
    let request_decoder = just_max_length_login_id_request_decoder();

    let mut action = ResetPasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite(request_decoder).await;
    assert_state(vec![
        "reset password error; invalid reset token entry: login id not matched",
    ]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn error_empty_password() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let feature = TestFeature::standard(&store);
    let request_decoder = empty_password_request_decoder();

    let mut action = ResetPasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite(request_decoder).await;
    assert_state(vec![
        "reset password error; invalid password: empty password",
    ]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn error_too_long_password() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let feature = TestFeature::standard(&store);
    let request_decoder = too_long_password_request_decoder();

    let mut action = ResetPasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite(request_decoder).await;
    assert_state(vec![
        "reset password error; invalid password: too long password",
    ]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn just_max_length_password() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let feature = TestFeature::standard(&store);
    let request_decoder = just_max_length_password_request_decoder();

    let mut action = ResetPasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite(request_decoder).await;
    assert_state(vec![
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
    let feature = TestFeature::standard(&store);
    let request_decoder = empty_reset_token_request_decoder();

    let mut action = ResetPasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite(request_decoder).await;
    assert_state(vec![
        "reset password error; invalid reset token: empty reset token",
    ]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn error_reset_token_expired_when_decode() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let feature = TestFeature::expired_reset_token(&store);
    let request_decoder = standard_request_decoder();

    let mut action = ResetPasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite(request_decoder).await;
    assert_state(vec!["reset password error; reset token expired"]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn error_reset_token_expired_in_store() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::expired_reset_token();
    let feature = TestFeature::standard(&store);
    let request_decoder = standard_request_decoder();

    let mut action = ResetPasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite(request_decoder).await;
    assert_state(vec![
        "reset password error; invalid reset token entry: reset token expired",
    ]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn error_reset_token_discarded() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::discarded_reset_token();
    let feature = TestFeature::standard(&store);
    let request_decoder = standard_request_decoder();

    let mut action = ResetPasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite(request_decoder).await;
    assert_state(vec![
        "reset password error; invalid reset token entry: already reset",
    ]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn error_password_not_stored() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::password_not_stored();
    let feature = TestFeature::standard(&store);
    let request_decoder = standard_request_decoder();

    let mut action = ResetPasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite(request_decoder).await;
    assert_state(vec![
        "reset password error; invalid reset token entry: reset token entry not found",
    ]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn error_user_not_stored() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::user_not_stored();
    let feature = TestFeature::standard(&store);
    let request_decoder = standard_request_decoder();

    let mut action = ResetPasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite(request_decoder).await;
    assert_state(vec!["reset password error; user not found"]);
    assert!(!result.is_ok());
}

struct TestFeature<'a> {
    reset: StaticResetPasswordStruct<'a>,
    issue: StaticIssueAuthTicketStruct<'a>,
    encode: StaticEncodeAuthTicketStruct<'a>,
}

impl<'a> ResetPasswordMaterial for TestFeature<'a> {
    type Reset = StaticResetPasswordStruct<'a>;
    type Issue = StaticIssueAuthTicketStruct<'a>;
    type Encode = StaticEncodeAuthTicketStruct<'a>;

    fn reset(&self) -> &Self::Reset {
        &self.reset
    }
    fn issue(&self) -> &Self::Issue {
        &self.issue
    }
    fn encode(&self) -> &Self::Encode {
        &self.encode
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
    fn password_not_stored() -> Self {
        Self {
            nonce: standard_nonce_store(),
            ticket: standard_ticket_store(),
            password: not_stored_password_store(),
            user: standard_user_store(),
        }
    }
    fn expired_reset_token() -> Self {
        Self {
            nonce: standard_nonce_store(),
            ticket: standard_ticket_store(),
            password: expired_reset_token_password_store(),
            user: standard_user_store(),
        }
    }
    fn discarded_reset_token() -> Self {
        Self {
            nonce: standard_nonce_store(),
            ticket: standard_ticket_store(),
            password: discarded_reset_token_password_store(),
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

impl<'a> TestFeature<'a> {
    fn standard(store: &'a TestStore) -> Self {
        Self::with_token_decoder(store, standard_reset_token_decoder())
    }
    fn expired_reset_token(store: &'a TestStore) -> Self {
        Self::with_token_decoder(store, expired_reset_token_decoder())
    }
    fn with_token_decoder(store: &'a TestStore, token_decoder: StaticResetTokenDecoder) -> Self {
        Self {
            reset: StaticResetPasswordStruct {
                check_nonce_infra: StaticCheckAuthNonceStruct {
                    config: standard_nonce_config(),
                    clock: standard_clock(),
                    nonce_metadata: standard_nonce_metadata(),
                    nonce_repository: MemoryAuthNonceRepository::new(&store.nonce),
                },
                clock: standard_clock(),
                user_repository: MemoryAuthUserRepository::new(&store.user),
                password_repository: MemoryAuthUserPasswordRepository::new(&store.password),
                token_decoder,
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
        }
    }
}

const NONCE: &'static str = "nonce";
const USER_ID: &'static str = "user-id";
const LOGIN_ID: &'static str = "login-id";
const RESET_TOKEN: &'static str = "reset-token";

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
            password: "password".into(),
        },
    }
}
fn match_failed_login_id_request_decoder() -> StaticResetPasswordRequestDecoder {
    StaticResetPasswordRequestDecoder {
        fields: ResetPasswordFieldsExtract {
            reset_token: RESET_TOKEN.into(),
            login_id: "unknown-login-id".into(),
            password: "password".into(),
        },
    }
}
fn empty_login_id_request_decoder() -> StaticResetPasswordRequestDecoder {
    StaticResetPasswordRequestDecoder {
        fields: ResetPasswordFieldsExtract {
            reset_token: RESET_TOKEN.into(),
            login_id: "".into(),
            password: "password".into(),
        },
    }
}
fn too_long_login_id_request_decoder() -> StaticResetPasswordRequestDecoder {
    StaticResetPasswordRequestDecoder {
        fields: ResetPasswordFieldsExtract {
            reset_token: RESET_TOKEN.into(),
            login_id: vec!["a"; 100 + 1].join(""),
            password: "password".into(),
        },
    }
}
fn just_max_length_login_id_request_decoder() -> StaticResetPasswordRequestDecoder {
    StaticResetPasswordRequestDecoder {
        fields: ResetPasswordFieldsExtract {
            reset_token: RESET_TOKEN.into(),
            login_id: vec!["a"; 100].join(""),
            password: "password".into(),
        },
    }
}
fn empty_password_request_decoder() -> StaticResetPasswordRequestDecoder {
    StaticResetPasswordRequestDecoder {
        fields: ResetPasswordFieldsExtract {
            reset_token: RESET_TOKEN.into(),
            login_id: LOGIN_ID.into(),
            password: "".into(),
        },
    }
}
fn too_long_password_request_decoder() -> StaticResetPasswordRequestDecoder {
    StaticResetPasswordRequestDecoder {
        fields: ResetPasswordFieldsExtract {
            reset_token: RESET_TOKEN.into(),
            login_id: LOGIN_ID.into(),
            password: vec!["a"; 100 + 1].join(""),
        },
    }
}
fn just_max_length_password_request_decoder() -> StaticResetPasswordRequestDecoder {
    StaticResetPasswordRequestDecoder {
        fields: ResetPasswordFieldsExtract {
            reset_token: RESET_TOKEN.into(),
            login_id: LOGIN_ID.into(),
            password: vec!["a"; 100].join(""),
        },
    }
}
fn empty_reset_token_request_decoder() -> StaticResetPasswordRequestDecoder {
    StaticResetPasswordRequestDecoder {
        fields: ResetPasswordFieldsExtract {
            reset_token: "".into(),
            login_id: LOGIN_ID.into(),
            password: "password".into(),
        },
    }
}

fn standard_reset_token_decoder() -> StaticResetTokenDecoder {
    StaticResetTokenDecoder::Valid(ResetToken::new(RESET_TOKEN.into()))
}
fn expired_reset_token_decoder() -> StaticResetTokenDecoder {
    StaticResetTokenDecoder::Expired
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
    let reset_token = ResetToken::new(RESET_TOKEN.into());
    let expires = AuthDateTime::restore(standard_now())
        .expires(&ExpireDuration::with_duration(Duration::days(1)));
    MemoryAuthUserPasswordMap::with_reset_token(
        test_user_login_id(),
        test_user_id(),
        reset_token,
        expires,
        None,
    )
    .to_store()
}
fn not_stored_password_store() -> MemoryAuthUserPasswordStore {
    MemoryAuthUserPasswordMap::new().to_store()
}
fn expired_reset_token_password_store() -> MemoryAuthUserPasswordStore {
    let reset_token = ResetToken::new(RESET_TOKEN.into());
    let expires = AuthDateTime::restore(standard_now())
        .expires(&ExpireDuration::with_duration(Duration::days(-1)));
    MemoryAuthUserPasswordMap::with_reset_token(
        test_user_login_id(),
        test_user_id(),
        reset_token,
        expires,
        None,
    )
    .to_store()
}
fn discarded_reset_token_password_store() -> MemoryAuthUserPasswordStore {
    let reset_token = ResetToken::new(RESET_TOKEN.into());
    let expires = AuthDateTime::restore(standard_now())
        .expires(&ExpireDuration::with_duration(Duration::days(1)));
    let discarded_at = AuthDateTime::restore(standard_now() - Duration::days(1));
    MemoryAuthUserPasswordMap::with_reset_token(
        test_user_login_id(),
        test_user_id(),
        reset_token,
        expires,
        Some(discarded_at),
    )
    .to_store()
}

fn standard_user_store() -> MemoryAuthUserStore {
    MemoryAuthUserMap::with_user(test_user()).to_store()
}
fn not_stored_user_store() -> MemoryAuthUserStore {
    MemoryAuthUserMap::new().to_store()
}

fn test_user() -> AuthUser {
    AuthUserExtract {
        user_id: USER_ID.into(),
        granted_roles: HashSet::new(),
    }
    .restore()
}
fn test_user_id() -> AuthUserId {
    AuthUserId::restore(USER_ID.into())
}
fn test_user_login_id() -> LoginId {
    LoginId::restore(LOGIN_ID.into())
}
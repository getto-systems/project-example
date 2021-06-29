use std::collections::HashSet;

use chrono::{DateTime, Duration, TimeZone, Utc};

use getto_application_test::ActionTestRunner;

use crate::auth::{
    auth_ticket::_api::{
        encode::init::test::{
            StaticAuthTokenEncoder, StaticEncodeAuthTicketStruct, StaticEncodeMessenger,
        },
        issue::init::test::{StaticAuthTicketIdGenerator, StaticIssueAuthTicketStruct},
        kernel::init::test::{
            MemoryAuthNonceMap, MemoryAuthNonceRepository, MemoryAuthNonceStore,
            MemoryAuthTicketMap, MemoryAuthTicketRepository, MemoryAuthTicketStore,
            StaticAuthNonceHeader, StaticCheckAuthNonceStruct, StaticChronoAuthClock,
        },
    },
    auth_user::_api::kernel::init::test::{
        MemoryAuthUserMap, MemoryAuthUserRepository, MemoryAuthUserStore,
    },
    password::reset::_api::reset::init::test::StaticResetPasswordStruct,
};

use crate::auth::{
    auth_ticket::_api::{
        encode::infra::EncodeAuthTicketConfig, issue::infra::IssueAuthTicketConfig,
        kernel::infra::AuthNonceConfig,
    },
    password::{
        _api::kernel::infra::password_repository::{
            MemoryAuthUserPasswordMap, MemoryAuthUserPasswordRepository,
            MemoryAuthUserPasswordStore,
        },
        reset::_api::reset::infra::{
            messenger::test::StaticResetPasswordMessenger,
            token_decoder::test::StaticResetTokenDecoder, ResetPasswordFieldsExtract,
        },
    },
};

use super::action::{ResetPasswordAction, ResetPasswordMaterial};

use crate::auth::{
    auth_ticket::_api::kernel::data::{
        AuthDateTime, AuthNonceValue, AuthTicketId, ExpansionLimitDuration, ExpireDuration,
    },
    auth_user::_api::kernel::data::{AuthUser, AuthUserExtract, AuthUserId},
    login_id::_api::data::LoginId,
    password::_api::kernel::data::ResetToken,
};

#[test]
fn success_request_token() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let feature = TestFeature::standard(&store);

    let mut action = ResetPasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite();
    assert_state(vec![
        "reset password success; user: user-id (granted: [])",
        "issue success; ticket: ticket-id / user: user-id (granted: [])",
        "token expires calculated; ticket: 2021-01-02 10:00:00 UTC / api: 2021-01-01 10:01:00 UTC / cdn: 2021-01-01 10:01:00 UTC",
        "encode success",
    ]);
    assert!(result.is_ok());
}

#[test]
fn success_expired_nonce() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::expired_nonce();
    let feature = TestFeature::standard(&store);

    let mut action = ResetPasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite();
    assert_state(vec![
        "reset password success; user: user-id (granted: [])",
        "issue success; ticket: ticket-id / user: user-id (granted: [])",
        "token expires calculated; ticket: 2021-01-02 10:00:00 UTC / api: 2021-01-01 10:01:00 UTC / cdn: 2021-01-01 10:01:00 UTC",
        "encode success",
    ]);
    assert!(result.is_ok());
}

#[test]
fn error_conflict_nonce() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::conflict_nonce();
    let feature = TestFeature::standard(&store);

    let mut action = ResetPasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite();
    assert_state(vec!["reset password error; auth nonce error: conflict"]);
    assert!(!result.is_ok());
}

#[test]
fn error_match_failed_login_id() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let feature = TestFeature::match_failed_login_id(&store);

    let mut action = ResetPasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite();
    assert_state(vec!["reset password error; invalid login id"]);
    assert!(!result.is_ok());
}

#[test]
fn error_empty_login_id() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let feature = TestFeature::empty_login_id(&store);

    let mut action = ResetPasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite();
    assert_state(vec!["reset password error; empty login id"]);
    assert!(!result.is_ok());
}

#[test]
fn error_too_long_login_id() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let feature = TestFeature::too_long_login_id(&store);

    let mut action = ResetPasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite();
    assert_state(vec!["reset password error; too long login id"]);
    assert!(!result.is_ok());
}

#[test]
fn just_max_length_login_id() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let feature = TestFeature::just_max_length_login_id(&store);

    let mut action = ResetPasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite();
    assert_state(vec!["reset password error; invalid login id"]);
    assert!(!result.is_ok());
}

#[test]
fn error_empty_password() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let feature = TestFeature::empty_password(&store);

    let mut action = ResetPasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite();
    assert_state(vec!["reset password error; empty password"]);
    assert!(!result.is_ok());
}

#[test]
fn error_too_long_password() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let feature = TestFeature::too_long_password(&store);

    let mut action = ResetPasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite();
    assert_state(vec!["reset password error; too long password"]);
    assert!(!result.is_ok());
}

#[test]
fn just_max_length_password() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let feature = TestFeature::just_max_length_password(&store);

    let mut action = ResetPasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite();
    assert_state(vec![
        "reset password success; user: user-id (granted: [])",
        "issue success; ticket: ticket-id / user: user-id (granted: [])",
        "token expires calculated; ticket: 2021-01-02 10:00:00 UTC / api: 2021-01-01 10:01:00 UTC / cdn: 2021-01-01 10:01:00 UTC",
        "encode success",
    ]);
    assert!(result.is_ok());
}

#[test]
fn error_empty_reset_token() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let feature = TestFeature::empty_reset_token(&store);

    let mut action = ResetPasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite();
    assert_state(vec!["reset password error; empty reset token"]);
    assert!(!result.is_ok());
}

#[test]
fn error_reset_token_expired_when_decode() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let feature = TestFeature::expired_reset_token(&store);

    let mut action = ResetPasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite();
    assert_state(vec!["reset password error; reset token expired"]);
    assert!(!result.is_ok());
}

#[test]
fn error_reset_token_expired_in_store() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::expired_reset_token();
    let feature = TestFeature::standard(&store);

    let mut action = ResetPasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite();
    assert_state(vec!["reset password error; reset token expired"]);
    assert!(!result.is_ok());
}

#[test]
fn error_reset_token_discarded() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::discarded_reset_token();
    let feature = TestFeature::standard(&store);

    let mut action = ResetPasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite();
    assert_state(vec!["reset password error; already reset"]);
    assert!(!result.is_ok());
}

#[test]
fn error_password_not_stored() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::password_not_stored();
    let feature = TestFeature::standard(&store);

    let mut action = ResetPasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite();
    assert_state(vec!["reset password error; reset token not found"]);
    assert!(!result.is_ok());
}

#[test]
fn error_user_not_stored() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::user_not_stored();
    let feature = TestFeature::standard(&store);

    let mut action = ResetPasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite();
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
        Self::with_messenger(store, standard_messenger(), standard_reset_token_decoder())
    }
    fn match_failed_login_id(store: &'a TestStore) -> Self {
        Self::with_messenger(
            store,
            match_failed_login_id_messenger(),
            standard_reset_token_decoder(),
        )
    }
    fn empty_login_id(store: &'a TestStore) -> Self {
        Self::with_messenger(
            store,
            empty_login_id_messenger(),
            standard_reset_token_decoder(),
        )
    }
    fn too_long_login_id(store: &'a TestStore) -> Self {
        Self::with_messenger(
            store,
            too_long_login_id_messenger(),
            standard_reset_token_decoder(),
        )
    }
    fn just_max_length_login_id(store: &'a TestStore) -> Self {
        Self::with_messenger(
            store,
            just_max_length_login_id_messenger(),
            standard_reset_token_decoder(),
        )
    }
    fn empty_password(store: &'a TestStore) -> Self {
        Self::with_messenger(
            store,
            empty_password_messenger(),
            standard_reset_token_decoder(),
        )
    }
    fn too_long_password(store: &'a TestStore) -> Self {
        Self::with_messenger(
            store,
            too_long_password_messenger(),
            standard_reset_token_decoder(),
        )
    }
    fn just_max_length_password(store: &'a TestStore) -> Self {
        Self::with_messenger(
            store,
            just_max_length_password_messenger(),
            standard_reset_token_decoder(),
        )
    }
    fn empty_reset_token(store: &'a TestStore) -> Self {
        Self::with_messenger(
            store,
            empty_reset_token_messenger(),
            standard_reset_token_decoder(),
        )
    }
    fn expired_reset_token(store: &'a TestStore) -> Self {
        Self::with_messenger(store, standard_messenger(), expired_reset_token_decoder())
    }
    fn with_messenger(
        store: &'a TestStore,
        messenger: StaticResetPasswordMessenger,
        token_decoder: StaticResetTokenDecoder,
    ) -> Self {
        Self {
            reset: StaticResetPasswordStruct {
                check_nonce_infra: StaticCheckAuthNonceStruct {
                    config: standard_nonce_config(),
                    clock: standard_clock(),
                    nonce_header: standard_nonce_header(),
                    nonce_repository: MemoryAuthNonceRepository::new(&store.nonce),
                },
                clock: standard_clock(),
                password_repository: MemoryAuthUserPasswordRepository::new(&store.password),
                user_repository: MemoryAuthUserRepository::new(&store.user),
                token_decoder,
                messenger,
            },
            issue: StaticIssueAuthTicketStruct {
                config: standard_issue_config(),
                clock: standard_clock(),
                ticket_repository: MemoryAuthTicketRepository::new(&store.ticket),
                ticket_id_generator: StaticAuthTicketIdGenerator::new(AuthTicketId::new(
                    "ticket-id".into(),
                )),
            },
            encode: StaticEncodeAuthTicketStruct {
                config: standard_encode_config(),
                clock: standard_clock(),
                ticket_repository: MemoryAuthTicketRepository::new(&store.ticket),
                ticket_encoder: StaticAuthTokenEncoder::new(),
                api_encoder: StaticAuthTokenEncoder::new(),
                cdn_encoder: StaticAuthTokenEncoder::new(),
                messenger: StaticEncodeMessenger::new(),
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
        cdn_expires: ExpireDuration::with_duration(Duration::minutes(1)),
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

fn standard_nonce_header() -> StaticAuthNonceHeader {
    StaticAuthNonceHeader::Valid(AuthNonceValue::new(NONCE.into()))
}

fn standard_messenger() -> StaticResetPasswordMessenger {
    StaticResetPasswordMessenger::new(ResetPasswordFieldsExtract {
        login_id: LOGIN_ID.into(),
        password: "password".into(),
        reset_token: RESET_TOKEN.into(),
    })
}
fn match_failed_login_id_messenger() -> StaticResetPasswordMessenger {
    StaticResetPasswordMessenger::new(ResetPasswordFieldsExtract {
        login_id: "unknown-login-id".into(),
        password: "password".into(),
        reset_token: RESET_TOKEN.into(),
    })
}
fn empty_login_id_messenger() -> StaticResetPasswordMessenger {
    StaticResetPasswordMessenger::new(ResetPasswordFieldsExtract {
        login_id: "".into(),
        password: "password".into(),
        reset_token: RESET_TOKEN.into(),
    })
}
fn too_long_login_id_messenger() -> StaticResetPasswordMessenger {
    StaticResetPasswordMessenger::new(ResetPasswordFieldsExtract {
        login_id: vec!["a"; 100 + 1].join(""),
        password: "password".into(),
        reset_token: RESET_TOKEN.into(),
    })
}
fn just_max_length_login_id_messenger() -> StaticResetPasswordMessenger {
    StaticResetPasswordMessenger::new(ResetPasswordFieldsExtract {
        login_id: vec!["a"; 100].join(""),
        password: "password".into(),
        reset_token: RESET_TOKEN.into(),
    })
}
fn empty_password_messenger() -> StaticResetPasswordMessenger {
    StaticResetPasswordMessenger::new(ResetPasswordFieldsExtract {
        login_id: LOGIN_ID.into(),
        password: "".into(),
        reset_token: RESET_TOKEN.into(),
    })
}
fn too_long_password_messenger() -> StaticResetPasswordMessenger {
    StaticResetPasswordMessenger::new(ResetPasswordFieldsExtract {
        login_id: LOGIN_ID.into(),
        password: vec!["a"; 100 + 1].join(""),
        reset_token: RESET_TOKEN.into(),
    })
}
fn just_max_length_password_messenger() -> StaticResetPasswordMessenger {
    StaticResetPasswordMessenger::new(ResetPasswordFieldsExtract {
        login_id: LOGIN_ID.into(),
        password: vec!["a"; 100].join(""),
        reset_token: RESET_TOKEN.into(),
    })
}
fn empty_reset_token_messenger() -> StaticResetPasswordMessenger {
    StaticResetPasswordMessenger::new(ResetPasswordFieldsExtract {
        login_id: LOGIN_ID.into(),
        password: "password".into(),
        reset_token: "".into(),
    })
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
    let expires = AuthDateTime::from_now(standard_now())
        .expires(&ExpireDuration::with_duration(Duration::days(-1)));
    MemoryAuthNonceMap::with_nonce(NONCE.into(), expires).to_store()
}
fn conflict_nonce_store() -> MemoryAuthNonceStore {
    let expires = AuthDateTime::from_now(standard_now())
        .expires(&ExpireDuration::with_duration(Duration::days(1)));
    MemoryAuthNonceMap::with_nonce(NONCE.into(), expires).to_store()
}

fn standard_ticket_store() -> MemoryAuthTicketStore {
    MemoryAuthTicketMap::new().to_store()
}

fn standard_password_store() -> MemoryAuthUserPasswordStore {
    let reset_token = ResetToken::new(RESET_TOKEN.into());
    let expires = AuthDateTime::from_now(standard_now())
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
    let expires = AuthDateTime::from_now(standard_now())
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
    let expires = AuthDateTime::from_now(standard_now())
        .expires(&ExpireDuration::with_duration(Duration::days(1)));
    let discarded_at = AuthDateTime::from_now(standard_now() - Duration::days(1));
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
    .into()
}
fn test_user_id() -> AuthUserId {
    AuthUserId::new(USER_ID.to_string())
}
fn test_user_login_id() -> LoginId {
    LoginId::validate(LOGIN_ID.to_string()).unwrap()
}

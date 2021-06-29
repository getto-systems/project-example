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
    password::_api::authenticate::init::test::StaticAuthenticatePasswordStruct,
};

use crate::auth::{
    auth_ticket::_api::{
        encode::infra::EncodeAuthTicketConfig, issue::infra::IssueAuthTicketConfig,
        kernel::infra::AuthNonceConfig,
    },
    auth_user::_api::kernel::infra::user_repository::{
        MemoryAuthUserMap, MemoryAuthUserRepository, MemoryAuthUserStore,
    },
    password::_api::{
        authenticate::infra::{
            messenger::test::StaticAuthenticatePasswordMessenger, AuthenticatePasswordFieldsExtract,
        },
        kernel::infra::{
            password_repository::{
                MemoryAuthUserPasswordMap, MemoryAuthUserPasswordRepository,
                MemoryAuthUserPasswordStore,
            },
            HashedPassword,
        },
    },
};

use super::action::AuthenticatePasswordAction;
use super::action::AuthenticatePasswordMaterial;

use crate::auth::{
    auth_ticket::_api::kernel::data::{
        AuthDateTime, AuthNonceValue, AuthTicketId, ExpansionLimitDuration, ExpireDuration,
    },
    auth_user::_api::kernel::data::{AuthUser, AuthUserExtract},
    login_id::_api::data::LoginId,
};

#[test]
fn success_authenticate() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let feature = TestFeature::standard(&store);

    let mut action = AuthenticatePasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite();
    assert_state(vec![
        "authenticate password success; user: test-user-id (granted: [something])",
        "issue success; ticket: ticket-id / user: test-user-id (granted: [something])",
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

    let mut action = AuthenticatePasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite();
    assert_state(vec![
        "authenticate password success; user: test-user-id (granted: [something])",
        "issue success; ticket: ticket-id / user: test-user-id (granted: [something])",
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

    let mut action = AuthenticatePasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite();
    assert_state(vec![
        "authenticate password error; auth nonce error: conflict",
    ]);
    assert!(!result.is_ok());
}

#[test]
fn error_empty_login_id() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let feature = TestFeature::empty_login_id(&store);

    let mut action = AuthenticatePasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite();
    assert_state(vec!["authenticate password error; empty login id"]);
    assert!(!result.is_ok());
}

#[test]
fn error_too_long_login_id() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let feature = TestFeature::too_long_login_id(&store);

    let mut action = AuthenticatePasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite();
    assert_state(vec!["authenticate password error; too long login id"]);
    assert!(!result.is_ok());
}

#[test]
fn just_max_length_login_id() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let feature = TestFeature::just_max_length_login_id(&store);

    let mut action = AuthenticatePasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite();
    assert_state(vec!["authenticate password error; user not found"]);
    assert!(!result.is_ok());
}

#[test]
fn error_empty_password() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let feature = TestFeature::empty_password(&store);

    let mut action = AuthenticatePasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite();
    assert_state(vec!["authenticate password error; empty password"]);
    assert!(!result.is_ok());
}

#[test]
fn error_too_long_password() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let feature = TestFeature::too_long_password(&store);

    let mut action = AuthenticatePasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite();
    assert_state(vec!["authenticate password error; too long password"]);
    assert!(!result.is_ok());
}

#[test]
fn just_max_length_password() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let feature = TestFeature::just_max_length_password(&store);

    let mut action = AuthenticatePasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite();
    assert_state(vec!["authenticate password error; password not matched"]);
    assert!(!result.is_ok());
}

#[test]
fn error_failed_to_match_password() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::match_fail_password();
    let feature = TestFeature::standard(&store);

    let mut action = AuthenticatePasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite();
    assert_state(vec!["authenticate password error; password not matched"]);
    assert!(!result.is_ok());
}

#[test]
fn error_password_not_stored() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::password_not_stored();
    let feature = TestFeature::standard(&store);

    let mut action = AuthenticatePasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite();
    assert_state(vec!["authenticate password error; user not found"]);
    assert!(!result.is_ok());
}

#[test]
fn error_user_not_stored() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::user_not_stored();
    let feature = TestFeature::standard(&store);

    let mut action = AuthenticatePasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite();
    assert_state(vec!["authenticate password error; user not found"]);
    assert!(!result.is_ok());
}

struct TestFeature<'a> {
    authenticate: StaticAuthenticatePasswordStruct<'a>,
    issue: StaticIssueAuthTicketStruct<'a>,
    encode: StaticEncodeAuthTicketStruct<'a>,
}

impl<'a> AuthenticatePasswordMaterial for TestFeature<'a> {
    type Authenticate = StaticAuthenticatePasswordStruct<'a>;
    type Issue = StaticIssueAuthTicketStruct<'a>;
    type Encode = StaticEncodeAuthTicketStruct<'a>;

    fn authenticate(&self) -> &Self::Authenticate {
        &self.authenticate
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

impl<'a> TestFeature<'a> {
    fn standard(store: &'a TestStore) -> Self {
        Self::with_messenger(store, standard_messenger())
    }
    fn empty_login_id(store: &'a TestStore) -> Self {
        Self::with_messenger(store, empty_login_id_messenger())
    }
    fn too_long_login_id(store: &'a TestStore) -> Self {
        Self::with_messenger(store, too_long_login_id_messenger())
    }
    fn just_max_length_login_id(store: &'a TestStore) -> Self {
        Self::with_messenger(store, just_max_length_login_id_messenger())
    }
    fn empty_password(store: &'a TestStore) -> Self {
        Self::with_messenger(store, empty_password_messenger())
    }
    fn too_long_password(store: &'a TestStore) -> Self {
        Self::with_messenger(store, too_long_password_messenger())
    }
    fn just_max_length_password(store: &'a TestStore) -> Self {
        Self::with_messenger(store, just_max_length_password_messenger())
    }
    fn with_messenger(
        store: &'a TestStore,
        messenger: StaticAuthenticatePasswordMessenger,
    ) -> Self {
        Self {
            authenticate: StaticAuthenticatePasswordStruct {
                check_nonce_infra: StaticCheckAuthNonceStruct {
                    config: standard_nonce_config(),
                    clock: standard_clock(),
                    nonce_header: standard_nonce_header(),
                    nonce_repository: MemoryAuthNonceRepository::new(&store.nonce),
                },
                clock: standard_clock(),
                password_repository: MemoryAuthUserPasswordRepository::new(&store.password),
                user_repository: MemoryAuthUserRepository::new(&store.user),
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

fn standard_messenger() -> StaticAuthenticatePasswordMessenger {
    StaticAuthenticatePasswordMessenger::new(AuthenticatePasswordFieldsExtract {
        login_id: "login-id".into(),
        password: "password".into(),
    })
}
fn empty_login_id_messenger() -> StaticAuthenticatePasswordMessenger {
    StaticAuthenticatePasswordMessenger::new(AuthenticatePasswordFieldsExtract {
        login_id: "".into(),
        password: "password".into(),
    })
}
fn too_long_login_id_messenger() -> StaticAuthenticatePasswordMessenger {
    StaticAuthenticatePasswordMessenger::new(AuthenticatePasswordFieldsExtract {
        login_id: vec!["a"; 100 + 1].join(""),
        password: "password".into(),
    })
}
fn just_max_length_login_id_messenger() -> StaticAuthenticatePasswordMessenger {
    StaticAuthenticatePasswordMessenger::new(AuthenticatePasswordFieldsExtract {
        login_id: vec!["a"; 100].join(""),
        password: "password".into(),
    })
}
fn empty_password_messenger() -> StaticAuthenticatePasswordMessenger {
    StaticAuthenticatePasswordMessenger::new(AuthenticatePasswordFieldsExtract {
        login_id: "login-id".into(),
        password: "".into(),
    })
}
fn too_long_password_messenger() -> StaticAuthenticatePasswordMessenger {
    StaticAuthenticatePasswordMessenger::new(AuthenticatePasswordFieldsExtract {
        login_id: "login-id".into(),
        password: vec!["a"; 100 + 1].join(""),
    })
}
fn just_max_length_password_messenger() -> StaticAuthenticatePasswordMessenger {
    StaticAuthenticatePasswordMessenger::new(AuthenticatePasswordFieldsExtract {
        login_id: "login-id".into(),
        password: vec!["a"; 100].join(""),
    })
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
    .into()
}
fn test_user_login_id() -> LoginId {
    LoginId::validate(LOGIN_ID.to_string()).unwrap()
}
fn test_user_password() -> HashedPassword {
    HashedPassword::new(PASSWORD.into())
}
fn another_password() -> HashedPassword {
    HashedPassword::new(ANOTHER_PASSWORD.into())
}

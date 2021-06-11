use std::collections::HashSet;

use chrono::{DateTime, Duration, TimeZone, Utc};

use getto_application_test::ActionTestRunner;

use crate::auth::{
    auth_ticket::_api::{
        encode::init::test::{StaticEncodeAuthTicketParam, StaticEncodeAuthTicketStruct},
        issue::init::test::StaticIssueAuthTicketStruct,
    },
    password::_api::authenticate::init::test::{
        StaticAuthenticatePasswordParam, StaticAuthenticatePasswordStruct,
    },
};

use crate::auth::{
    auth_ticket::_api::{
        encode::infra::EncodeAuthTicketConfig,
        issue::infra::{id_generator::test::StaticAuthTicketIdGenerator, IssueAuthTicketConfig},
        kernel::infra::{
            clock::test::StaticChronoAuthClock, nonce_header::test::StaticAuthNonceHeader,
            nonce_repository::MemoryAuthNonceMap, nonce_repository::MemoryAuthNonceRepository,
            nonce_repository::MemoryAuthNonceStore, ticket_repository::MemoryAuthTicketMap,
            ticket_repository::MemoryAuthTicketRepository,
            ticket_repository::MemoryAuthTicketStore, AuthNonceConfig,
        },
    },
    auth_user::_api::kernel::infra::user_repository::{
        MemoryAuthUserMap, MemoryAuthUserRepository, MemoryAuthUserStore,
    },
    password::_api::authenticate::infra::{
        messenger::test::StaticAuthenticateMessenger,
        password_repository::{
            MemoryAuthUserPasswordMap, MemoryAuthUserPasswordRepository,
            MemoryAuthUserPasswordStore,
        },
        AuthenticatePasswordFieldsExtract, HashedPassword,
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
    let feature = TestFeature::new(&store);

    let mut action = AuthenticatePasswordAction::with_material(feature);
    action.subscribe(handler);

    assert!(action.ignite().is_ok());
    assert_state(vec![
        "authenticate success; user: test-user-id (granted: [something])",
        "issue success; ticket: ticket-id / user: test-user-id (granted: [something])",
        "token expires calculated; ticket: 2021-01-02 10:00:00 UTC / api: 2021-01-01 10:01:00 UTC / cdn: 2021-01-01 10:01:00 UTC",
        "encode success",
    ])
}

#[test]
fn success_expired_nonce() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::expired_nonce();
    let feature = TestFeature::new(&store);

    let mut action = AuthenticatePasswordAction::with_material(feature);
    action.subscribe(handler);

    assert!(action.ignite().is_ok());
    assert_state(vec![
        "authenticate success; user: test-user-id (granted: [something])",
        "issue success; ticket: ticket-id / user: test-user-id (granted: [something])",
        "token expires calculated; ticket: 2021-01-02 10:00:00 UTC / api: 2021-01-01 10:01:00 UTC / cdn: 2021-01-01 10:01:00 UTC",
        "encode success",
    ])
}

#[test]
fn error_conflict_nonce() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::conflict_nonce();
    let feature = TestFeature::new(&store);

    let mut action = AuthenticatePasswordAction::with_material(feature);
    action.subscribe(handler);

    assert!(!action.ignite().is_ok());
    assert_state(vec![
        "authenticate error; auth nonce error: conflict",
    ])
}

#[test]
fn error_invalid_password() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::invalid_password();
    let feature = TestFeature::new(&store);

    let mut action = AuthenticatePasswordAction::with_material(feature);
    action.subscribe(handler);

    assert!(!action.ignite().is_ok());
    assert_state(vec![
        "authenticate error; password not match",
    ])
}

#[test]
fn error_no_password() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::no_password();
    let feature = TestFeature::new(&store);

    let mut action = AuthenticatePasswordAction::with_material(feature);
    action.subscribe(handler);

    assert!(!action.ignite().is_ok());
    assert_state(vec![
        "authenticate error; password not match",
    ])
}

#[test]
fn error_no_user() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::no_user();
    let feature = TestFeature::new(&store);

    let mut action = AuthenticatePasswordAction::with_material(feature);
    action.subscribe(handler);

    assert!(!action.ignite().is_ok());
    assert_state(vec![
        "authenticate error; user not found",
    ])
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
    fn invalid_password() -> Self {
        Self {
            nonce: standard_nonce_store(),
            ticket: standard_ticket_store(),
            password: invalid_password_store(),
            user: standard_user_store(),
        }
    }
    fn no_password() -> Self {
        Self {
            nonce: standard_nonce_store(),
            ticket: standard_ticket_store(),
            password: no_password_store(),
            user: standard_user_store(),
        }
    }
    fn no_user() -> Self {
        Self {
            nonce: standard_nonce_store(),
            ticket: standard_ticket_store(),
            password: standard_password_store(),
            user: no_user_store(),
        }
    }
}

impl<'a> TestFeature<'a> {
    fn new(store: &'a TestStore) -> Self {
        Self {
            authenticate: StaticAuthenticatePasswordStruct::new(StaticAuthenticatePasswordParam {
                nonce_config: standard_nonce_config(),
                clock: standard_clock(),
                nonce_header: standard_nonce_header(),
                nonce_repository: MemoryAuthNonceRepository::new(&store.nonce),
                password_repository: MemoryAuthUserPasswordRepository::new(&store.password),
                user_repository: MemoryAuthUserRepository::new(&store.user),
                messenger: standard_messenger(),
            }),
            issue: StaticIssueAuthTicketStruct {
                config: standard_issue_config(),
                clock: standard_clock(),
                ticket_repository: MemoryAuthTicketRepository::new(&store.ticket),
                ticket_id_generator: StaticAuthTicketIdGenerator::new(AuthTicketId::new(
                    "ticket-id".into(),
                )),
            },
            encode: StaticEncodeAuthTicketStruct::new(StaticEncodeAuthTicketParam {
                config: standard_encode_config(),
                clock: standard_clock(),
                ticket_repository: MemoryAuthTicketRepository::new(&store.ticket),
            }),
        }
    }
}

const NONCE: &'static str = "nonce";
const LOGIN_ID: &'static str = "login-id";
const PASSWORD: &'static str = "password";
const INVALID_PASSWORD: &'static str = "invalid-password";

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

fn standard_messenger() -> StaticAuthenticateMessenger {
    StaticAuthenticateMessenger::new(AuthenticatePasswordFieldsExtract {
        login_id: "login-id".into(),
        password: "password".into(),
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
fn invalid_password_store() -> MemoryAuthUserPasswordStore {
    MemoryAuthUserPasswordMap::with_password(
        test_user_login_id(),
        test_user(),
        invalid_password(),
    )
    .to_store()
}
fn no_password_store() -> MemoryAuthUserPasswordStore {
    MemoryAuthUserPasswordMap::new().to_store()
}

fn standard_user_store() -> MemoryAuthUserStore {
    MemoryAuthUserMap::with_user(test_user()).to_store()
}
fn no_user_store() -> MemoryAuthUserStore {
    MemoryAuthUserMap::new().to_store()
}

fn test_user() -> AuthUser {
    let mut granted_roles = HashSet::new();
    granted_roles.insert("something".into());

    AuthUser::from_extract(AuthUserExtract {
        id: "test-user-id".into(),
        granted_roles,
    })
}
fn test_user_login_id() -> LoginId {
    LoginId::validate(LOGIN_ID.to_string()).unwrap()
}
fn test_user_password() -> HashedPassword {
    HashedPassword::new(PASSWORD.into())
}
fn invalid_password() -> HashedPassword {
    HashedPassword::new(INVALID_PASSWORD.into())
}

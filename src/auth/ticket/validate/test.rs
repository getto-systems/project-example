use std::collections::HashSet;

use getto_application_test::ActionTestRunner;

use chrono::{DateTime, Duration, TimeZone, Utc};

use crate::auth::ticket::{
    kernel::remote::init::clock::test::StaticChronoAuthClock,
    validate::init::{
        nonce_metadata::test::StaticAuthNonceMetadata,
        nonce_repository::test::{
            MemoryAuthNonceMap, MemoryAuthNonceRepository, MemoryAuthNonceStore,
        },
        request_decoder::test::StaticValidateApiTokenRequestDecoder,
        test::{StaticValidateAuthNonceStruct, StaticValidateAuthTokenStruct},
        token_decoder::test::StaticAuthTokenDecoder,
        token_metadata::test::StaticAuthTokenMetadata,
    },
};

use crate::auth::ticket::validate::method::AuthNonceConfig;

use super::action::ValidateApiTokenAction;

use crate::auth::{
    ticket::kernel::remote::data::{AuthDateTime, AuthTicketExtract, ExpireDuration},
    user::kernel::data::RequireAuthRoles,
};

#[tokio::test]
async fn success_allow_for_any_role() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let material = TestStruct::standard(&store);
    let request_decoder = allow_any_role_request_decoder();

    let mut action = ValidateApiTokenAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "validate success; ticket: ticket-id / user: something-role-user-id (granted: [something])",
        "validate api token success; user: something-role-user-id (granted: [something])",
    ]);
    assert!(result.is_ok());
}

#[tokio::test]
async fn success_allow_for_something_role() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let material = TestStruct::standard(&store);
    let request_decoder = allow_something_role_request_decoder();

    let mut action = ValidateApiTokenAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "validate success; ticket: ticket-id / user: something-role-user-id (granted: [something])",
        "validate api token success; user: something-role-user-id (granted: [something])",
    ]);
    assert!(result.is_ok());
}

#[tokio::test]
async fn error_allow_for_something_role_but_not_granted() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let material = TestStruct::no_granted_roles(&store);
    let request_decoder = allow_something_role_request_decoder();

    let mut action = ValidateApiTokenAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "validate error; user permission denied; granted: [], require: any [something]",
    ]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn error_token_expired() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let material = TestStruct::token_expired(&store);
    let request_decoder = allow_something_role_request_decoder();

    let mut action = ValidateApiTokenAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "validate error; token expired",
    ]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn success_expired_nonce() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::expired_nonce();
    let material = TestStruct::standard(&store);
    let request_decoder = allow_something_role_request_decoder();

    let mut action = ValidateApiTokenAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "validate success; ticket: ticket-id / user: something-role-user-id (granted: [something])",
        "validate api token success; user: something-role-user-id (granted: [something])",
    ]);
    assert!(result.is_ok());
}

#[tokio::test]
async fn error_conflict_nonce() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::conflict_nonce();
    let material = TestStruct::standard(&store);
    let request_decoder = allow_something_role_request_decoder();

    let mut action = ValidateApiTokenAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce error; conflict",
    ]);
    assert!(!result.is_ok());
}

struct TestStruct;

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

impl TestStruct {
    fn standard<'a>(store: &'a TestStore) -> StaticValidateAuthTokenStruct<'a> {
        Self::with_token_validator(store, standard_token_decoder())
    }
    fn no_granted_roles<'a>(store: &'a TestStore) -> StaticValidateAuthTokenStruct<'a> {
        Self::with_token_validator(store, no_granted_roles_token_decoder())
    }
    fn token_expired<'a>(store: &'a TestStore) -> StaticValidateAuthTokenStruct<'a> {
        Self::with_token_validator(store, expired_token_decoder())
    }

    fn with_token_validator<'a>(
        store: &'a TestStore,
        token_decoder: StaticAuthTokenDecoder,
    ) -> StaticValidateAuthTokenStruct<'a> {
        StaticValidateAuthTokenStruct {
            validate_nonce: StaticValidateAuthNonceStruct {
                config: standard_nonce_config(),
                clock: standard_clock(),
                nonce_metadata: standard_nonce_header(),
                nonce_repository: MemoryAuthNonceRepository::new(&store.nonce),
            },
            token_metadata: standard_token_header(),
            token_decoder,
        }
    }
}

const NONCE: &'static str = "nonce";
const TICKET_ID: &'static str = "ticket-id";

fn allow_any_role_request_decoder() -> StaticValidateApiTokenRequestDecoder {
    StaticValidateApiTokenRequestDecoder {
        require_roles: RequireAuthRoles::Nothing,
    }
}
fn allow_something_role_request_decoder() -> StaticValidateApiTokenRequestDecoder {
    StaticValidateApiTokenRequestDecoder {
        require_roles: RequireAuthRoles::has_any(&["something"]),
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
    let mut granted_roles = HashSet::new();
    granted_roles.insert("something".into());

    StaticAuthTokenDecoder::Valid(AuthTicketExtract {
        ticket_id: TICKET_ID.into(),
        user_id: "something-role-user-id".into(),
        granted_roles,
    })
}
fn no_granted_roles_token_decoder() -> StaticAuthTokenDecoder {
    StaticAuthTokenDecoder::Valid(AuthTicketExtract {
        ticket_id: TICKET_ID.into(),
        user_id: "no-role-user-id".into(),
        granted_roles: HashSet::new(),
    })
}
fn expired_token_decoder() -> StaticAuthTokenDecoder {
    StaticAuthTokenDecoder::Expired
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

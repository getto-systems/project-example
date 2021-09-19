use std::collections::HashSet;

use getto_application_test::ActionTestRunner;

use chrono::{DateTime, Duration, TimeZone, Utc};

use crate::auth::auth_ticket::{
    _auth::{
        discard::init::test::StaticDiscardAuthTicketStruct,
        kernel::init::{
            clock::test::StaticChronoAuthClock,
            nonce_repository::test::{
                MemoryAuthNonceMap, MemoryAuthNonceRepository, MemoryAuthNonceStore,
            },
            test::StaticCheckAuthNonceStruct,
            ticket_repository::test::{
                MemoryAuthTicketMap, MemoryAuthTicketRepository, MemoryAuthTicketStore,
            },
        },
        validate::init::{
            test::StaticValidateAuthTokenStruct, token_decoder::test::StaticAuthTokenDecoder,
        },
    },
    _common::kernel::init::{
        nonce_metadata::test::StaticAuthNonceMetadata,
        token_metadata::test::StaticAuthTokenMetadata,
    },
};

use crate::auth::auth_ticket::_auth::kernel::infra::AuthNonceConfig;

use super::action::{LogoutAction, LogoutMaterial};

use crate::auth::auth_ticket::_auth::kernel::data::{
    AuthDateTime, AuthTicketExtract, AuthTicketId, ExpansionLimitDuration, ExpireDuration,
};

#[tokio::test]
async fn success_logout() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let feature = TestFeature::standard(&store);

    let mut action = LogoutAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "validate success; ticket: ticket-id / user: user-id (granted: [])",
        "discard success",
    ]);
    assert!(result.is_ok());
}

#[tokio::test]
async fn success_expired_nonce() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::expired_nonce();
    let feature = TestFeature::standard(&store);

    let mut action = LogoutAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "validate success; ticket: ticket-id / user: user-id (granted: [])",
        "discard success",
    ]);
    assert!(result.is_ok());
}

#[tokio::test]
async fn error_conflict_nonce() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::conflict_nonce();
    let feature = TestFeature::standard(&store);

    let mut action = LogoutAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec!["validate error; auth nonce error: conflict"]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn error_no_ticket() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::no_ticket();
    let feature = TestFeature::standard(&store);

    let mut action = LogoutAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "validate success; ticket: ticket-id / user: user-id (granted: [])",
        "discard success",
    ]);
    assert!(result.is_ok());
}

struct TestFeature<'a> {
    validate: StaticValidateAuthTokenStruct<'a>,
    discard: StaticDiscardAuthTicketStruct<'a>,
}

impl<'a> LogoutMaterial for TestFeature<'a> {
    type Validate = StaticValidateAuthTokenStruct<'a>;
    type Discard = StaticDiscardAuthTicketStruct<'a>;

    fn validate(&self) -> &Self::Validate {
        &self.validate
    }
    fn discard(&self) -> &Self::Discard {
        &self.discard
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
    fn no_ticket() -> Self {
        Self {
            nonce: standard_nonce_store(),
            ticket: no_ticket_store(),
        }
    }
}

impl<'a> TestFeature<'a> {
    fn standard(store: &'a TestStore) -> Self {
        Self {
            validate: StaticValidateAuthTokenStruct {
                check_nonce_infra: StaticCheckAuthNonceStruct {
                    config: standard_nonce_config(),
                    clock: standard_clock(),
                    nonce_metadata: standard_nonce_metadata(),
                    nonce_repository: MemoryAuthNonceRepository::new(&store.nonce),
                },
                token_metadata: standard_token_metadata(),
                token_decoder: standard_token_validator(),
            },
            discard: StaticDiscardAuthTicketStruct {
                clock: standard_clock(),
                ticket_repository: MemoryAuthTicketRepository::new(&store.ticket),
            },
        }
    }
}

const NONCE: &'static str = "nonce";
const TICKET_ID: &'static str = "ticket-id";

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

fn standard_nonce_metadata() -> StaticAuthNonceMetadata {
    StaticAuthNonceMetadata::new(NONCE.into())
}
fn standard_token_metadata() -> StaticAuthTokenMetadata {
    StaticAuthTokenMetadata::new("TOKEN".into())
}

fn standard_token_validator() -> StaticAuthTokenDecoder {
    StaticAuthTokenDecoder::Valid(
        AuthTicketExtract {
            ticket_id: TICKET_ID.into(),
            user_id: "user-id".into(),
            granted_roles: HashSet::new(),
        }
        .restore(),
    )
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
    let limit = AuthDateTime::restore(standard_now())
        .expansion_limit(&ExpansionLimitDuration::with_duration(Duration::days(10)));
    MemoryAuthTicketMap::with_ticket(AuthTicketId::new(TICKET_ID.into()), limit).to_store()
}
fn no_ticket_store() -> MemoryAuthTicketStore {
    MemoryAuthTicketMap::new().to_store()
}

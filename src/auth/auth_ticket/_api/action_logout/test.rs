use std::collections::HashSet;

use getto_application_test::ActionTestRunner;

use chrono::{DateTime, Duration, TimeZone, Utc};

use crate::auth::auth_ticket::_api::{
    discard::init::test::StaticDiscardAuthTicketStruct,
    kernel::init::test::{
        MemoryAuthNonceMap, MemoryAuthNonceRepository, MemoryAuthNonceStore, MemoryAuthTicketMap,
        MemoryAuthTicketRepository, MemoryAuthTicketStore, StaticAuthNonceHeader,
        StaticAuthTicketStruct, StaticCheckAuthNonceStruct, StaticChronoAuthClock,
    },
    validate::init::test::{
        StaticAuthTokenDecoder, StaticAuthTokenHeader, StaticValidateAuthTokenStruct,
    },
};

use crate::auth::auth_ticket::_api::{
    kernel::infra::AuthNonceConfig, validate::infra::ValidateAuthTokenConfig,
};

use super::action::{LogoutAction, LogoutMaterial};

use crate::auth::auth_ticket::_api::kernel::data::{
    AuthDateTime, AuthNonceValue, AuthTicketExtract, AuthTicketId, AuthTokenValue,
    ExpansionLimitDuration, ExpireDuration,
};
use crate::auth::auth_user::_api::kernel::data::RequireAuthRoles;

#[test]
fn success_logout() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let feature = TestFeature::standard(&store);

    let mut action = LogoutAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite();
    assert_state(vec![
        "validate success; ticket: ticket-id / user: user-id (granted: [])",
        "discard success",
    ]);
    assert!(result.is_ok());
}

#[test]
fn success_expired_nonce() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::expired_nonce();
    let feature = TestFeature::standard(&store);

    let mut action = LogoutAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite();
    assert_state(vec![
        "validate success; ticket: ticket-id / user: user-id (granted: [])",
        "discard success",
    ]);
    assert!(result.is_ok());
}

#[test]
fn success_limited_ticket() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::limited_ticket();
    let feature = TestFeature::standard(&store);

    let mut action = LogoutAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite();
    assert_state(vec![
        "validate success; ticket: ticket-id / user: user-id (granted: [])",
        "discard success",
    ]);
    assert!(result.is_ok());
}

#[test]
fn error_conflict_nonce() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::conflict_nonce();
    let feature = TestFeature::standard(&store);

    let mut action = LogoutAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite();
    assert_state(vec!["validate error; auth nonce error: conflict"]);
    assert!(!result.is_ok());
}

#[test]
fn error_no_ticket() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::no_ticket();
    let feature = TestFeature::standard(&store);

    let mut action = LogoutAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite();
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
    fn limited_ticket() -> Self {
        Self {
            nonce: standard_nonce_store(),
            ticket: limited_ticket_store(),
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
                    nonce_header: standard_nonce_header(),
                    nonce_repository: MemoryAuthNonceRepository::new(&store.nonce),
                },
                config: ValidateAuthTokenConfig {
                    require_roles: RequireAuthRoles::Nothing,
                },
                clock: standard_clock(),
                token_header: standard_token_header(),
                ticket_repository: MemoryAuthTicketRepository::new(&store.ticket),
                token_validator: standard_token_validator(),
            },
            discard: StaticDiscardAuthTicketStruct {
                ticket_infra: StaticAuthTicketStruct {
                    clock: standard_clock(),
                    ticket_repository: MemoryAuthTicketRepository::new(&store.ticket),
                },
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

fn standard_nonce_header() -> StaticAuthNonceHeader {
    StaticAuthNonceHeader::Valid(AuthNonceValue::new(NONCE.into()))
}
fn standard_token_header() -> StaticAuthTokenHeader {
    StaticAuthTokenHeader::Valid(AuthTokenValue::new("TOKEN".into()))
}

fn standard_token_validator() -> StaticAuthTokenDecoder {
    StaticAuthTokenDecoder::Valid(
        AuthTicketExtract {
            ticket_id: TICKET_ID.into(),
            user_id: "user-id".into(),
            granted_roles: HashSet::new(),
        }
        .into(),
    )
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
    let limit = AuthDateTime::from_now(standard_now())
        .limit(&ExpansionLimitDuration::with_duration(Duration::days(10)));
    MemoryAuthTicketMap::with_ticket(AuthTicketId::new(TICKET_ID.into()), limit).to_store()
}
fn limited_ticket_store() -> MemoryAuthTicketStore {
    let limit = AuthDateTime::from_now(standard_now())
        .limit(&ExpansionLimitDuration::with_duration(Duration::hours(1)));
    MemoryAuthTicketMap::with_ticket(AuthTicketId::new(TICKET_ID.into()), limit).to_store()
}
fn no_ticket_store() -> MemoryAuthTicketStore {
    MemoryAuthTicketMap::new().to_store()
}

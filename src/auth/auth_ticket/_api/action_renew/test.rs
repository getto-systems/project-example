use std::collections::HashSet;

use getto_application_test::ActionTestRunner;

use chrono::{DateTime, Duration, TimeZone, Utc};

use crate::auth::auth_ticket::_api::{
    encode::init::test::{
        StaticAuthTokenEncoder, StaticEncodeAuthTicketStruct, StaticEncodeMessenger,
    },
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
    encode::infra::EncodeAuthTicketConfig, kernel::infra::AuthNonceConfig,
    validate::infra::ValidateAuthTokenConfig,
};

use super::action::{RenewAuthTicketAction, RenewAuthTicketMaterial};

use crate::auth::auth_ticket::_api::kernel::data::{
    AuthDateTime, AuthNonceValue, AuthTicketExtract, AuthTicketId, AuthTokenValue,
    ExpansionLimitDuration, ExpireDuration,
};
use crate::auth::auth_user::_api::kernel::data::RequireAuthRoles;

#[tokio::test]
async fn success_allow_for_any_role() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let feature = TestFeature::allow_for_any_role(&store);

    let mut action = RenewAuthTicketAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "validate success; ticket: ticket-id / user: something-role-user-id (granted: [something])",
        "token expires calculated; ticket: 2021-01-02 10:00:00 UTC / api: 2021-01-01 10:01:00 UTC / cdn: 2021-01-01 10:01:00 UTC",
        "encode success",
    ]);
    assert!(result.is_ok());
}

#[tokio::test]
async fn success_allow_for_something_role() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let feature = TestFeature::allow_for_something_role(&store);

    let mut action = RenewAuthTicketAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "validate success; ticket: ticket-id / user: something-role-user-id (granted: [something])",
        "token expires calculated; ticket: 2021-01-02 10:00:00 UTC / api: 2021-01-01 10:01:00 UTC / cdn: 2021-01-01 10:01:00 UTC",
        "encode success",
    ]);
    assert!(result.is_ok());
}

#[tokio::test]
async fn error_allow_for_something_role_but_not_granted() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let feature = TestFeature::allow_for_something_role_but_not_granted(&store);

    let mut action = RenewAuthTicketAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "validate error; auth token error: user permission denied: granted: [], required: any [something]",
    ]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn error_token_expired() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let feature = TestFeature::token_expired(&store);

    let mut action = RenewAuthTicketAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec!["validate error; auth token error: token expired"]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn success_expired_nonce() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::expired_nonce();
    let feature = TestFeature::allow_for_any_role(&store);

    let mut action = RenewAuthTicketAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "validate success; ticket: ticket-id / user: something-role-user-id (granted: [something])",
        "token expires calculated; ticket: 2021-01-02 10:00:00 UTC / api: 2021-01-01 10:01:00 UTC / cdn: 2021-01-01 10:01:00 UTC",
        "encode success",
    ]);
    assert!(result.is_ok());
}

#[tokio::test]
async fn success_limited_ticket() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::limited_ticket();
    let feature = TestFeature::allow_for_any_role(&store);

    let mut action = RenewAuthTicketAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "validate success; ticket: ticket-id / user: something-role-user-id (granted: [something])",
        "token expires calculated; ticket: 2021-01-01 11:00:00 UTC / api: 2021-01-01 10:01:00 UTC / cdn: 2021-01-01 10:01:00 UTC",
        "encode success",
    ]);
    assert!(result.is_ok());
}

#[tokio::test]
async fn error_conflict_nonce() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::conflict_nonce();
    let feature = TestFeature::allow_for_any_role(&store);

    let mut action = RenewAuthTicketAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec!["validate error; auth nonce error: conflict"]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn error_no_ticket() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::no_ticket();
    let feature = TestFeature::allow_for_any_role(&store);

    let mut action = RenewAuthTicketAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "validate success; ticket: ticket-id / user: something-role-user-id (granted: [something])",
        "encode error; ticket data not found",
    ]);
    assert!(!result.is_ok());
}

struct TestFeature<'a> {
    validate: StaticValidateAuthTokenStruct<'a>,
    encode: StaticEncodeAuthTicketStruct<'a>,
}

impl<'a> RenewAuthTicketMaterial for TestFeature<'a> {
    type Validate = StaticValidateAuthTokenStruct<'a>;
    type Encode = StaticEncodeAuthTicketStruct<'a>;

    fn validate(&self) -> &Self::Validate {
        &self.validate
    }
    fn encode(&self) -> &Self::Encode {
        &self.encode
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
    fn allow_for_any_role(store: &'a TestStore) -> Self {
        Self::with_require_roles_and_validator(
            store,
            RequireAuthRoles::Nothing,
            standard_token_decoder(),
        )
    }
    fn allow_for_something_role(store: &'a TestStore) -> Self {
        Self::with_require_roles_and_validator(
            store,
            RequireAuthRoles::has_any(&["something"]),
            standard_token_decoder(),
        )
    }
    fn allow_for_something_role_but_not_granted(store: &'a TestStore) -> Self {
        Self::with_require_roles_and_validator(
            store,
            RequireAuthRoles::has_any(&["something"]),
            no_granted_roles_token_decoder(),
        )
    }
    fn token_expired(store: &'a TestStore) -> Self {
        Self::with_require_roles_and_validator(
            store,
            RequireAuthRoles::Nothing,
            expired_token_decoder(),
        )
    }

    fn with_require_roles_and_validator(
        store: &'a TestStore,
        require_roles: RequireAuthRoles,
        token_validator: StaticAuthTokenDecoder,
    ) -> Self {
        Self {
            validate: StaticValidateAuthTokenStruct {
                check_nonce_infra: StaticCheckAuthNonceStruct {
                    config: standard_nonce_config(),
                    clock: standard_clock(),
                    nonce_header: standard_nonce_header(),
                    nonce_repository: MemoryAuthNonceRepository::new(&store.nonce),
                },
                ticket_infra: standard_ticket_infra(store),
                config: ValidateAuthTokenConfig { require_roles },
                token_header: standard_token_header(),
                token_validator,
            },
            encode: StaticEncodeAuthTicketStruct {
                ticket_infra: standard_ticket_infra(store),
                ticket_encoder: StaticAuthTokenEncoder::new(),
                api_encoder: StaticAuthTokenEncoder::new(),
                cdn_encoder: StaticAuthTokenEncoder::new(),
                messenger: StaticEncodeMessenger::new(),
                config: standard_encode_config(),
            },
        }
    }
}

fn standard_ticket_infra<'a>(store: &'a TestStore) -> StaticAuthTicketStruct<'a> {
    StaticAuthTicketStruct {
        clock: standard_clock(),
        ticket_repository: MemoryAuthTicketRepository::new(&store.ticket),
    }
}

const NONCE: &'static str = "nonce";
const TICKET_ID: &'static str = "ticket-id";

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

fn standard_token_decoder() -> StaticAuthTokenDecoder {
    let mut granted_roles = HashSet::new();
    granted_roles.insert("something".into());

    StaticAuthTokenDecoder::Valid(
        AuthTicketExtract {
            ticket_id: TICKET_ID.into(),
            user_id: "something-role-user-id".into(),
            granted_roles,
        }
        .into(),
    )
}
fn no_granted_roles_token_decoder() -> StaticAuthTokenDecoder {
    StaticAuthTokenDecoder::Valid(
        AuthTicketExtract {
            ticket_id: TICKET_ID.into(),
            user_id: "no-role-user-id".into(),
            granted_roles: HashSet::new(),
        }
        .into(),
    )
}
fn expired_token_decoder() -> StaticAuthTokenDecoder {
    StaticAuthTokenDecoder::Expired
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
        .expansion_limit(&ExpansionLimitDuration::with_duration(Duration::days(10)));
    MemoryAuthTicketMap::with_ticket(AuthTicketId::new(TICKET_ID.into()), limit).to_store()
}
fn limited_ticket_store() -> MemoryAuthTicketStore {
    let limit = AuthDateTime::from_now(standard_now())
        .expansion_limit(&ExpansionLimitDuration::with_duration(Duration::hours(1)));
    MemoryAuthTicketMap::with_ticket(AuthTicketId::new(TICKET_ID.into()), limit).to_store()
}
fn no_ticket_store() -> MemoryAuthTicketStore {
    MemoryAuthTicketMap::new().to_store()
}

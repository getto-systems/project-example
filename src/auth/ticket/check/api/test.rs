use std::collections::HashSet;

use getto_application_test::ActionTestRunner;

use chrono::{DateTime, Duration, TimeZone, Utc};

use crate::auth::ticket::{
    encode::init::{
        test::StaticEncodeAuthTicketStruct,
        token_encoder::test::{StaticAuthTokenEncoder, StaticCloudfrontTokenEncoder},
    },
    kernel::{
        data::AuthTicket,
        init::{
            clock::test::StaticChronoAuthClock,
            ticket_repository::memory::{MemoryAuthTicketRepository, MemoryAuthTicketStore},
        },
    },
    validate::init::{
        nonce_metadata::test::StaticAuthNonceMetadata,
        nonce_repository::memory::MemoryAuthNonceRepository,
        test::{StaticValidateAuthNonceStruct, StaticValidateAuthTokenStruct},
        token_decoder::test::StaticAuthTokenDecoder,
        token_metadata::test::StaticAuthTokenMetadata,
    },
};

use crate::auth::ticket::{
    encode::method::EncodeAuthTicketConfig, validate::method::AuthNonceConfig,
};

use super::action::{CheckAuthTicketAction, CheckAuthTicketMaterial};

use crate::auth::ticket::kernel::data::{
    AuthDateTime, AuthTicketExtract, ExpansionLimitDuration, ExpireDuration,
};

#[tokio::test]
async fn success() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::new();
    let material = TestStruct::standard(&store);

    let mut action = CheckAuthTicketAction::with_material(material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "validate success; ticket: ticket-id / user: user-role-user-id (granted: [user])",
        "token expires calculated; ticket: 2021-01-02 10:00:00 UTC / api: 2021-01-01 10:01:00 UTC / cloudfront: 2021-01-01 10:01:00 UTC",
        "encode success",
    ]);
    assert!(result.is_ok());
}

#[tokio::test]
async fn error_token_expired() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::new();
    let material = TestStruct::token_expired(&store);

    let mut action = CheckAuthTicketAction::with_material(material);
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
async fn success_limited_ticket() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::new();
    let material = TestStruct::limited_ticket(&store);

    let mut action = CheckAuthTicketAction::with_material(material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "validate success; ticket: ticket-id / user: user-role-user-id (granted: [user])",
        "token expires calculated; ticket: 2021-01-01 11:00:00 UTC / api: 2021-01-01 10:01:00 UTC / cloudfront: 2021-01-01 10:01:00 UTC",
        "encode success",
    ]);
    assert!(result.is_ok());
}

#[tokio::test]
async fn error_no_ticket() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::new();
    let material = TestStruct::no_ticket(&store);

    let mut action = CheckAuthTicketAction::with_material(material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "validate success; ticket: ticket-id / user: user-role-user-id (granted: [user])",
        "encode error; ticket data not found",
    ]);
    assert!(!result.is_ok());
}

struct TestStruct<'a> {
    validate: StaticValidateAuthTokenStruct,
    encode: StaticEncodeAuthTicketStruct<'a>,
}

impl<'a> CheckAuthTicketMaterial for TestStruct<'a> {
    type Validate = StaticValidateAuthTokenStruct;
    type Encode = StaticEncodeAuthTicketStruct<'a>;

    fn validate(&self) -> &Self::Validate {
        &self.validate
    }
    fn encode(&self) -> &Self::Encode {
        &self.encode
    }
}

struct TestStore {
    ticket: MemoryAuthTicketStore,
}

impl TestStore {
    fn new() -> Self {
        Self {
            ticket: MemoryAuthTicketStore::new(),
        }
    }
}

impl<'a> TestStruct<'a> {
    fn standard(store: &'a TestStore) -> Self {
        Self::new(
            standard_ticket_repository(&store.ticket),
            standard_token_decoder(),
        )
    }
    fn token_expired(store: &'a TestStore) -> Self {
        Self::new(
            standard_ticket_repository(&store.ticket),
            expired_token_decoder(),
        )
    }
    fn limited_ticket(store: &'a TestStore) -> Self {
        Self::new(
            limited_ticket_repository(&store.ticket),
            standard_token_decoder(),
        )
    }
    fn no_ticket(store: &'a TestStore) -> Self {
        Self::new(
            no_ticket_repository(&store.ticket),
            standard_token_decoder(),
        )
    }

    fn new(
        ticket_repository: MemoryAuthTicketRepository<'a>,
        token_validator: StaticAuthTokenDecoder,
    ) -> Self {
        Self {
            validate: StaticValidateAuthTokenStruct {
                validate_nonce: StaticValidateAuthNonceStruct {
                    config: standard_nonce_config(),
                    clock: standard_clock(),
                    nonce_metadata: standard_nonce_header(),
                    nonce_repository: standard_nonce_repository(),
                },
                token_metadata: standard_token_header(),
                token_decoder: token_validator,
            },
            encode: StaticEncodeAuthTicketStruct {
                clock: standard_clock(),
                ticket_repository,
                ticket_encoder: StaticAuthTokenEncoder,
                api_encoder: StaticAuthTokenEncoder,
                cloudfront_encoder: StaticCloudfrontTokenEncoder,
                config: standard_encode_config(),
            },
        }
    }
}

const NONCE: &'static str = "nonce";
const TICKET_ID: &'static str = "ticket-id";
const USER_ID: &'static str = "user-id";

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
    granted_roles.insert("user".into());

    StaticAuthTokenDecoder::Valid(AuthTicketExtract {
        ticket_id: TICKET_ID.into(),
        user_id: "user-role-user-id".into(),
        granted_roles,
    })
}
fn expired_token_decoder() -> StaticAuthTokenDecoder {
    StaticAuthTokenDecoder::Expired
}

fn standard_nonce_repository() -> MemoryAuthNonceRepository {
    MemoryAuthNonceRepository::new()
}

fn standard_ticket_repository<'a>(
    store: &'a MemoryAuthTicketStore,
) -> MemoryAuthTicketRepository<'a> {
    let issued_at = AuthDateTime::restore(standard_now());
    let limit =
        issued_at.expansion_limit(&ExpansionLimitDuration::with_duration(Duration::days(10)));
    MemoryAuthTicketRepository::with_ticket(store, test_ticket(), limit, issued_at)
}
fn limited_ticket_repository<'a>(
    store: &'a MemoryAuthTicketStore,
) -> MemoryAuthTicketRepository<'a> {
    let issued_at = AuthDateTime::restore(standard_now());
    let limit =
        issued_at.expansion_limit(&ExpansionLimitDuration::with_duration(Duration::hours(1)));
    MemoryAuthTicketRepository::with_ticket(store, test_ticket(), limit, issued_at)
}
fn no_ticket_repository<'a>(store: &'a MemoryAuthTicketStore) -> MemoryAuthTicketRepository<'a> {
    MemoryAuthTicketRepository::new(store)
}

fn test_ticket() -> AuthTicket {
    AuthTicket::restore(AuthTicketExtract {
        ticket_id: TICKET_ID.into(),
        user_id: USER_ID.into(),
        granted_roles: HashSet::new(),
    })
}

use std::collections::HashSet;

use getto_application_test::ActionTestRunner;

use chrono::{DateTime, Duration, TimeZone, Utc};

use crate::auth::ticket::{
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

use crate::auth::ticket::validate::method::AuthNonceConfig;

use super::action::{LogoutAction, LogoutMaterial};

use crate::auth::ticket::kernel::data::{
    AuthDateTime, AuthTicketExtract, ExpansionLimitDuration, ExpireDuration,
};

#[tokio::test]
async fn success_logout() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::new();
    let material = TestStruct::standard(&store);

    let mut action = LogoutAction::with_material(material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "validate success; ticket: ticket-id / user: user-id (granted: [])",
        "logout success",
    ]);
    assert!(result.is_ok());
}

#[tokio::test]
async fn error_no_ticket() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::new();
    let material = TestStruct::no_ticket(&store);

    let mut action = LogoutAction::with_material(material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "validate success; ticket: ticket-id / user: user-id (granted: [])",
        "logout success",
    ]);
    assert!(result.is_ok());
}

struct TestStruct<'a> {
    validate: StaticValidateAuthTokenStruct,
    ticket_repository: MemoryAuthTicketRepository<'a>,
}

impl<'a> LogoutMaterial for TestStruct<'a> {
    type ValidateInfra = StaticValidateAuthTokenStruct;
    type TicketRepository = MemoryAuthTicketRepository<'a>;

    fn validate(&self) -> &Self::ValidateInfra {
        &self.validate
    }
    fn ticket_repository(&self) -> &Self::TicketRepository {
        &self.ticket_repository
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
        Self::new(standard_ticket_repository(&store.ticket))
    }
    fn no_ticket(store: &'a TestStore) -> Self {
        Self::new(no_ticket_repository(&store.ticket))
    }
    fn new(ticket_repository: MemoryAuthTicketRepository<'a>) -> Self {
        Self {
            validate: StaticValidateAuthTokenStruct {
                validate_nonce: StaticValidateAuthNonceStruct {
                    config: standard_nonce_config(),
                    clock: standard_clock(),
                    nonce_metadata: standard_nonce_metadata(),
                    nonce_repository: standard_nonce_repository(),
                },
                token_metadata: standard_token_metadata(),
                token_decoder: standard_token_validator(),
            },
            ticket_repository,
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
    StaticAuthTokenDecoder::Valid(AuthTicketExtract {
        ticket_id: TICKET_ID.into(),
        user_id: "user-id".into(),
        granted_roles: HashSet::new(),
    })
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

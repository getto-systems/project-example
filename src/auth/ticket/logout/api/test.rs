use getto_application_test::ApplicationActionStateHolder;
use pretty_assertions::assert_eq;

use chrono::{DateTime, Duration, TimeZone, Utc};

use crate::auth::ticket::{
    authenticate::init::test::StaticAuthenticateWithTokenInfra,
    kernel::init::{
        request::test::StaticAuthenticateToken,
        ticket_repository::memory::{MemoryAuthTicketRepository, MemoryAuthTicketStore},
        token::authenticate::decoder::test::StaticAuthenticateTokenDecoder,
    },
    logout::init::test::StaticLogoutMaterial,
};

use crate::auth::ticket::logout::action::LogoutAction;

use crate::auth::{
    kernel::data::{AuthDateTime, ExpansionLimitDuration},
    ticket::kernel::data::AuthTicket,
};

#[tokio::test]
async fn info() {
    let store = TestStore::new();
    let material = StaticLogoutMaterial {
        authenticate_with_token: standard_authenticate_with_token_infra(),
        ticket_repository: standard_ticket_repository(&store),
    };

    let action = LogoutAction::with_material(material);

    assert_eq!(action.info.name(), "auth.ticket.logout");
}

#[tokio::test]
async fn success_logout() {
    let holder = ApplicationActionStateHolder::new();

    let store = TestStore::new();
    let material = StaticLogoutMaterial {
        authenticate_with_token: standard_authenticate_with_token_infra(),
        ticket_repository: standard_ticket_repository(&store),
    };

    let mut action = LogoutAction::with_material(material);
    action.subscribe(holder.handler());

    let result = action.ignite(StaticAuthenticateToken).await;
    assert_eq!(
        holder.extract(),
        vec![
            "authenticate with token success; ticket: ticket-id / user-id: user-id (granted: [])",
            "logout success",
        ],
    );
    assert!(result.is_ok());
}

#[tokio::test]
async fn error_no_ticket() {
    let holder = ApplicationActionStateHolder::new();

    let store = TestStore::new();
    let material = StaticLogoutMaterial {
        authenticate_with_token: standard_authenticate_with_token_infra(),
        ticket_repository: no_ticket_repository(&store),
    };

    let mut action = LogoutAction::with_material(material);
    action.subscribe(holder.handler());

    let result = action.ignite(StaticAuthenticateToken).await;
    assert_eq!(
        holder.extract(),
        vec![
            "authenticate with token success; ticket: ticket-id / user-id: user-id (granted: [])",
            "logout success",
        ],
    );
    assert!(result.is_ok());
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

fn standard_now() -> DateTime<Utc> {
    Utc.with_ymd_and_hms(2021, 1, 1, 10, 0, 0).latest().unwrap()
}

fn standard_authenticate_with_token_infra() -> StaticAuthenticateWithTokenInfra {
    StaticAuthenticateWithTokenInfra {
        token_decoder: StaticAuthenticateTokenDecoder::Valid(stored_ticket()),
    }
}

fn standard_ticket_repository<'a>(store: &'a TestStore) -> MemoryAuthTicketRepository<'a> {
    let issued_at = AuthDateTime::restore(standard_now());
    let limit =
        issued_at.expansion_limit(&ExpansionLimitDuration::with_duration(Duration::days(10)));
    MemoryAuthTicketRepository::with_ticket(&store.ticket, stored_ticket(), limit, issued_at)
}
fn no_ticket_repository<'a>(store: &'a TestStore) -> MemoryAuthTicketRepository<'a> {
    MemoryAuthTicketRepository::new(&store.ticket)
}

fn stored_ticket() -> AuthTicket {
    AuthTicket::standard()
}

use getto_application_test::ApplicationActionStateHolder;
use pretty_assertions::assert_eq;

use chrono::{DateTime, Duration, TimeZone, Utc};

use crate::x_content::permission::AuthPermission;

use crate::auth::{
    kernel::init::clock::test::StaticChronoAuthClock,
    ticket::{
        authenticate::init::test::{
            StaticAuthenticateWithTokenInfra, StaticAuthenticateWithTokenMaterial,
        },
        encode::init::test::StaticEncodeAuthTokenInfra,
        kernel::init::{
            request::test::StaticAuthenticateToken,
            ticket_repository::memory::{MemoryAuthTicketRepository, MemoryAuthTicketStore},
            token::authenticate::decoder::test::StaticAuthenticateTokenDecoder,
        },
    },
};

use crate::auth::ticket::authenticate::action::AuthenticateWithTokenAction;

use crate::auth::ticket::encode::infra::EncodeAuthTokenConfig;

use crate::auth::{
    kernel::data::{AuthDateTime, ExpansionLimitDuration, ExpireDuration},
    ticket::kernel::data::{AuthPermissionGranted, AuthTicket, AuthTicketAttrs, AuthTicketId},
    user::kernel::data::AuthUserId,
};

#[tokio::test]
async fn info() {
    let store = TestStore::new();
    let material = StaticAuthenticateWithTokenMaterial {
        authenticate_with_token: StaticAuthenticateWithTokenInfra {
            token_decoder: standard_token_decoder(),
        },
        encode: standard_encode_infra(new_ticket_repository(&store, Duration::days(10))),
    };

    let action = AuthenticateWithTokenAction::with_material(material);

    assert_eq!(action.info.name(), "auth.ticket.authenticate");
}

#[tokio::test]
async fn success() {
    let holder = ApplicationActionStateHolder::new();

    let store = TestStore::new();
    let material = StaticAuthenticateWithTokenMaterial {
        authenticate_with_token: StaticAuthenticateWithTokenInfra {
            token_decoder: standard_token_decoder(),
        },
        encode: standard_encode_infra(new_ticket_repository(&store, Duration::days(10))),
    };

    let mut action = AuthenticateWithTokenAction::with_material(material);
    action.subscribe(holder.handler());

    let result = action.ignite(StaticAuthenticateToken).await;
    assert_eq!(
        holder.extract(),
        vec![
            "authenticate with token success; ticket: ticket-id / user-id: user-id (granted: [auth-user])",
            "token expires calculated; authenticate: 2021-01-02 10:00:00 UTC / authorize: 2021-01-01 10:01:00 UTC / cdn: 2021-01-01 10:01:00 UTC",
            "encode auth-token success",
        ],
    );
    assert!(result.is_ok());
}

#[tokio::test]
async fn error_token_expired() {
    let holder = ApplicationActionStateHolder::new();

    let store = TestStore::new();
    let material = StaticAuthenticateWithTokenMaterial {
        authenticate_with_token: StaticAuthenticateWithTokenInfra {
            token_decoder: expired_token_decoder(),
        },
        encode: standard_encode_infra(new_ticket_repository(&store, Duration::days(10))),
    };

    let mut action = AuthenticateWithTokenAction::with_material(material);
    action.subscribe(holder.handler());

    let result = action.ignite(StaticAuthenticateToken).await;
    assert_eq!(
        holder.extract(),
        vec!["authenticate with token error; token expired"],
    );
    assert!(result.is_err());
}

#[tokio::test]
async fn success_limited_ticket() {
    let holder = ApplicationActionStateHolder::new();

    let store = TestStore::new();
    let material = StaticAuthenticateWithTokenMaterial {
        authenticate_with_token: StaticAuthenticateWithTokenInfra {
            token_decoder: standard_token_decoder(),
        },
        encode: standard_encode_infra(new_ticket_repository(&store, Duration::hours(1))),
    };

    let mut action = AuthenticateWithTokenAction::with_material(material);
    action.subscribe(holder.handler());

    let result = action.ignite(StaticAuthenticateToken).await;
    assert_eq!(
        holder.extract(),
        vec![
            "authenticate with token success; ticket: ticket-id / user-id: user-id (granted: [auth-user])",
            "token expires calculated; authenticate: 2021-01-01 11:00:00 UTC / authorize: 2021-01-01 10:01:00 UTC / cdn: 2021-01-01 10:01:00 UTC",
            "encode auth-token success",
        ],
    );
    assert!(result.is_ok());
}

#[tokio::test]
async fn error_no_ticket() {
    let holder = ApplicationActionStateHolder::new();

    let store = TestStore::new();
    let material = StaticAuthenticateWithTokenMaterial {
        authenticate_with_token: StaticAuthenticateWithTokenInfra {
            token_decoder: standard_token_decoder(),
        },
        encode: standard_encode_infra(no_ticket_repository(&store)),
    };

    let mut action = AuthenticateWithTokenAction::with_material(material);
    action.subscribe(holder.handler());

    let result = action.ignite(StaticAuthenticateToken).await;
    assert_eq!(
        holder.extract(),
        vec![
            "authenticate with token success; ticket: ticket-id / user-id: user-id (granted: [auth-user])",
            "encode auth-token error; ticket data not found",
        ],
    );
    assert!(result.is_err());
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

fn standard_encode_infra<'a>(
    ticket_repository: MemoryAuthTicketRepository<'a>,
) -> StaticEncodeAuthTokenInfra<'a> {
    StaticEncodeAuthTokenInfra::standard(
        standard_clock(),
        ticket_repository,
        standard_encode_config(),
    )
}

fn standard_encode_config() -> EncodeAuthTokenConfig {
    EncodeAuthTokenConfig {
        authenticate_expires: ExpireDuration::with_duration(Duration::days(1)),
        authorize_expires: ExpireDuration::with_duration(Duration::minutes(1)),
        cdn_expires: ExpireDuration::with_duration(Duration::minutes(1)),
    }
}

fn standard_now() -> DateTime<Utc> {
    Utc.with_ymd_and_hms(2021, 1, 1, 10, 0, 0).latest().unwrap()
}
fn standard_clock() -> StaticChronoAuthClock {
    StaticChronoAuthClock::new(standard_now())
}

fn standard_token_decoder() -> StaticAuthenticateTokenDecoder {
    StaticAuthenticateTokenDecoder::Valid(stored_ticket())
}
fn expired_token_decoder() -> StaticAuthenticateTokenDecoder {
    StaticAuthenticateTokenDecoder::Expired
}

fn new_ticket_repository<'a>(
    store: &'a TestStore,
    duration: Duration,
) -> MemoryAuthTicketRepository<'a> {
    let issued_at = AuthDateTime::restore(standard_now());
    let limit = issued_at.expansion_limit(&ExpansionLimitDuration::with_duration(duration));
    MemoryAuthTicketRepository::with_ticket(&store.ticket, stored_ticket(), limit, issued_at)
}
fn no_ticket_repository<'a>(store: &'a TestStore) -> MemoryAuthTicketRepository<'a> {
    MemoryAuthTicketRepository::new(&store.ticket)
}

fn stored_ticket() -> AuthTicket {
    AuthTicket {
        ticket_id: AuthTicketId::restore("ticket-id".to_owned()),
        attrs: AuthTicketAttrs {
            user_id: AuthUserId::restore("user-id".to_owned()),
            granted: AuthPermissionGranted::restore(
                vec![AuthPermission::AuthUser].into_iter().collect(),
            ),
        },
    }
}

use getto_application_test::ApplicationActionStateHolder;
use pretty_assertions::assert_eq;

use chrono::{DateTime, Duration, TimeZone, Utc};

use crate::auth::{
    kernel::init::clock::test::StaticChronoAuthClock,
    ticket::{
        authorize::init::test::{
            StaticAuthorizeFields, StaticAuthorizeWithTokenInfra,
            StaticClarifyAuthorizeTokenMaterial,
        },
        kernel::init::{
            ticket_repository::memory::{MemoryAuthTicketRepository, MemoryAuthTicketStore},
            token::authorize::decoder::test::StaticAuthorizeTokenDecoder,
        },
    },
    user::kernel::init::user_repository::memory::{MemoryAuthUserRepository, MemoryAuthUserStore},
};

use crate::x_content::permission::AuthPermission;

use crate::auth::ticket::authorize::action::ClarifyAuthorizeTokenAction;

use crate::auth::ticket::authorize::infra::AuthorizeFields;

use crate::auth::{
    kernel::data::{AuthDateTime, ExpansionLimitDuration},
    ticket::{
        authorize::data::ValidateAuthorizeFieldsError,
        kernel::data::{
            AuthPermissionGranted, AuthPermissionRequired, AuthTicket, AuthTicketAttrs,
            AuthTicketId, AuthorizeToken, ValidateAuthorizeTokenError,
        },
    },
    user::{kernel::data::AuthUserId, login_id::kernel::data::LoginId},
};

#[tokio::test]
async fn info() {
    let store = TestStore::new();
    let material = StaticClarifyAuthorizeTokenMaterial {
        authorize_with_token: StaticAuthorizeWithTokenInfra {
            token_decoder: standard_token_decoder(),
        },
        clock: standard_clock(),
        ticket_repository: new_ticket_repository(&store, Duration::days(10)),
        user_repository: standard_user_repository(&store),
    };

    let action = ClarifyAuthorizeTokenAction::with_material(material);

    assert_eq!(action.info.name(), "auth.ticket.authorize.clarify");
}

#[tokio::test]
async fn success() {
    let holder = ApplicationActionStateHolder::new();

    let store = TestStore::new();
    let material = StaticClarifyAuthorizeTokenMaterial {
        authorize_with_token: StaticAuthorizeWithTokenInfra {
            token_decoder: standard_token_decoder(),
        },
        clock: standard_clock(),
        ticket_repository: new_ticket_repository(&store, Duration::days(10)),
        user_repository: standard_user_repository(&store),
    };
    let fields = StaticAuthorizeFields::Valid(standard_authorize_fields());

    let mut action = ClarifyAuthorizeTokenAction::with_material(material);
    action.subscribe(holder.handler());

    let result = action.ignite(fields).await;
    assert_eq!(
        holder.extract(),
        vec![
            "authorize with token success",
            "clarify authorize-token success; user-id: user-id (granted: [auth-user])",
        ]
    );
    assert!(result.is_ok());
}

#[tokio::test]
async fn error_invalid_token() {
    let holder = ApplicationActionStateHolder::new();

    let store = TestStore::new();
    let material = StaticClarifyAuthorizeTokenMaterial {
        authorize_with_token: StaticAuthorizeWithTokenInfra {
            token_decoder: standard_token_decoder(),
        },
        clock: standard_clock(),
        ticket_repository: new_ticket_repository(&store, Duration::days(10)),
        user_repository: standard_user_repository(&store),
    };
    let fields = StaticAuthorizeFields::Invalid(ValidateAuthorizeFieldsError::Token(
        ValidateAuthorizeTokenError::NotFound,
    ));

    let mut action = ClarifyAuthorizeTokenAction::with_material(material);
    action.subscribe(holder.handler());

    let result = action.ignite(fields).await;
    assert_eq!(
        holder.extract(),
        vec!["authorize with token error; invalid token: data not found"]
    );
    assert!(result.is_err());
}

#[tokio::test]
async fn error_token_expired() {
    let holder = ApplicationActionStateHolder::new();

    let store = TestStore::new();
    let material = StaticClarifyAuthorizeTokenMaterial {
        authorize_with_token: StaticAuthorizeWithTokenInfra {
            token_decoder: expired_token_decoder(),
        },
        clock: standard_clock(),
        ticket_repository: new_ticket_repository(&store, Duration::days(10)),
        user_repository: standard_user_repository(&store),
    };
    let fields = StaticAuthorizeFields::Valid(standard_authorize_fields());

    let mut action = ClarifyAuthorizeTokenAction::with_material(material);
    action.subscribe(holder.handler());

    let result = action.ignite(fields).await;
    assert_eq!(
        holder.extract(),
        vec!["authorize with token error; token expired"]
    );
    assert!(result.is_err());
}

#[tokio::test]
async fn error_no_ticket() {
    let holder = ApplicationActionStateHolder::new();

    let store = TestStore::new();
    let material = StaticClarifyAuthorizeTokenMaterial {
        authorize_with_token: StaticAuthorizeWithTokenInfra {
            token_decoder: standard_token_decoder(),
        },
        clock: standard_clock(),
        ticket_repository: no_ticket_repository(&store),
        user_repository: standard_user_repository(&store),
    };
    let fields = StaticAuthorizeFields::Valid(standard_authorize_fields());

    let mut action = ClarifyAuthorizeTokenAction::with_material(material);
    action.subscribe(holder.handler());

    let result = action.ignite(fields).await;
    assert_eq!(
        holder.extract(),
        vec![
            "authorize with token success",
            "clarify authorize-token error; ticket not found",
        ]
    );
    assert!(result.is_err());
}

#[tokio::test]
async fn error_expired_ticket() {
    let holder = ApplicationActionStateHolder::new();

    let store = TestStore::new();
    let material = StaticClarifyAuthorizeTokenMaterial {
        authorize_with_token: StaticAuthorizeWithTokenInfra {
            token_decoder: standard_token_decoder(),
        },
        clock: standard_clock(),
        ticket_repository: new_ticket_repository(&store, Duration::days(-1)),
        user_repository: standard_user_repository(&store),
    };
    let fields = StaticAuthorizeFields::Valid(standard_authorize_fields());

    let mut action = ClarifyAuthorizeTokenAction::with_material(material);
    action.subscribe(holder.handler());

    let result = action.ignite(fields).await;
    assert_eq!(
        holder.extract(),
        vec![
            "authorize with token success",
            "clarify authorize-token error; ticket has expired",
        ]
    );
    assert!(result.is_err());
}

#[tokio::test]
async fn error_no_user() {
    let holder = ApplicationActionStateHolder::new();

    let store = TestStore::new();
    let material = StaticClarifyAuthorizeTokenMaterial {
        authorize_with_token: StaticAuthorizeWithTokenInfra {
            token_decoder: standard_token_decoder(),
        },
        clock: standard_clock(),
        ticket_repository: new_ticket_repository(&store, Duration::days(10)),
        user_repository: no_user_repository(&store),
    };
    let fields = StaticAuthorizeFields::Valid(standard_authorize_fields());

    let mut action = ClarifyAuthorizeTokenAction::with_material(material);
    action.subscribe(holder.handler());

    let result = action.ignite(fields).await;
    assert_eq!(
        holder.extract(),
        vec![
            "authorize with token success",
            "clarify authorize-token error; user not found",
        ]
    );
    assert!(result.is_err());
}

#[tokio::test]
async fn error_permission_denied() {
    let holder = ApplicationActionStateHolder::new();

    let store = TestStore::new();
    let material = StaticClarifyAuthorizeTokenMaterial {
        authorize_with_token: StaticAuthorizeWithTokenInfra {
            token_decoder: standard_token_decoder(),
        },
        clock: standard_clock(),
        ticket_repository: new_ticket_repository(&store, Duration::days(10)),
        user_repository: denied_user_repository(&store),
    };
    let fields = StaticAuthorizeFields::Valid(standard_authorize_fields());

    let mut action = ClarifyAuthorizeTokenAction::with_material(material);
    action.subscribe(holder.handler());

    let result = action.ignite(fields).await;
    assert_eq!(
        holder.extract(),
        vec![
            "authorize with token success",
            "clarify authorize-token error; permission denied; granted: [], require: some [auth-user]",
        ]
    );
    assert!(result.is_err());
}

struct TestStore {
    ticket: MemoryAuthTicketStore,
    user: MemoryAuthUserStore,
}

impl TestStore {
    fn new() -> Self {
        Self {
            ticket: MemoryAuthTicketStore::new(),
            user: MemoryAuthUserStore::new(),
        }
    }
}

fn standard_now() -> DateTime<Utc> {
    Utc.with_ymd_and_hms(2021, 1, 1, 10, 0, 0).latest().unwrap()
}
fn standard_clock() -> StaticChronoAuthClock {
    StaticChronoAuthClock::new(standard_now())
}

fn standard_token_decoder() -> StaticAuthorizeTokenDecoder {
    StaticAuthorizeTokenDecoder::Valid(stored_ticket())
}
fn expired_token_decoder() -> StaticAuthorizeTokenDecoder {
    StaticAuthorizeTokenDecoder::Expired
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

fn standard_user_repository<'a>(store: &'a TestStore) -> MemoryAuthUserRepository<'a> {
    MemoryAuthUserRepository::with_user_and_permission(
        &store.user,
        stored_user_id(),
        stored_login_id(),
        stored_permission_granted(),
    )
}
fn denied_user_repository<'a>(store: &'a TestStore) -> MemoryAuthUserRepository<'a> {
    MemoryAuthUserRepository::with_user_and_permission(
        &store.user,
        stored_user_id(),
        stored_login_id(),
        Default::default(),
    )
}
fn no_user_repository<'a>(store: &'a TestStore) -> MemoryAuthUserRepository<'a> {
    MemoryAuthUserRepository::new(&store.user)
}

fn stored_ticket() -> AuthTicket {
    AuthTicket {
        ticket_id: AuthTicketId::restore("ticket-id".to_owned()),
        attrs: AuthTicketAttrs {
            user_id: stored_user_id(),
            granted: stored_permission_granted(),
        },
    }
}
fn stored_user_id() -> AuthUserId {
    AuthUserId::restore("user-id".to_owned())
}
fn stored_login_id() -> LoginId {
    LoginId::restore("login-id".to_owned())
}

fn standard_authorize_fields() -> AuthorizeFields {
    AuthorizeFields {
        token: AuthorizeToken::restore("TOKEN".to_owned()),
        required: AuthPermissionRequired::HasSome(
            vec![AuthPermission::AuthUser].into_iter().collect(),
        ),
    }
}

fn stored_permission_granted() -> AuthPermissionGranted {
    AuthPermissionGranted::restore(vec![AuthPermission::AuthUser].into_iter().collect())
}

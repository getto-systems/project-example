use chrono::{DateTime, Duration, TimeZone, Utc};
use pretty_assertions::assert_eq;

use getto_application_test::ApplicationActionStateHolder;

use crate::auth::{
    kernel::init::clock::test::StaticChronoAuthClock,
    ticket::{
        encode::init::test::StaticEncodeAuthTokenInfra,
        issue::init::test::{StaticAuthTicketIdGenerator, StaticIssueAuthTicketInfra},
        kernel::init::ticket_repository::memory::{
            MemoryAuthTicketRepository, MemoryAuthTicketStore,
        },
    },
    user::{
        kernel::init::user_repository::memory::{MemoryAuthUserRepository, MemoryAuthUserStore},
        password::authenticate::init::test::{
            StaticAuthenticateWithPasswordFields, StaticAuthenticateWithPasswordMaterial,
        },
    },
};

use crate::auth::user::password::authenticate::action::AuthenticateWithPasswordAction;

use crate::auth::{
    ticket::{encode::infra::EncodeAuthTokenConfig, issue::infra::IssueAuthTicketConfig},
    user::password::{
        authenticate::infra::AuthenticateWithPasswordFields,
        kernel::infra::{HashedPassword, PlainPassword},
    },
};

use crate::{
    auth::{
        kernel::data::{ExpansionLimitDuration, ExpireDuration},
        ticket::kernel::data::{AuthPermissionGranted, AuthTicketId},
        user::{
            kernel::data::{AuthUser, AuthUserId},
            login_id::kernel::data::{LoginId, ValidateLoginIdError},
            password::authenticate::data::ValidateAuthenticateWithPasswordFieldsError,
        },
    },
    common::api::validate::data::ValidateTextError,
};

#[tokio::test]
async fn info() {
    let store = TestStore::standard();
    let material = StaticAuthenticateWithPasswordMaterial {
        issue: standard_issue_infra(&store),
        encode: standard_encode_infra(&store),
        password_repository: standard_password_repository(&store),
    };

    let action = AuthenticateWithPasswordAction::with_material(material);

    assert_eq!(action.info.name(), "auth.user.password.authenticate");
}

#[tokio::test]
async fn success_authenticate() {
    let holder = ApplicationActionStateHolder::new();

    let store = TestStore::standard();
    let material = StaticAuthenticateWithPasswordMaterial {
        issue: standard_issue_infra(&store),
        encode: standard_encode_infra(&store),
        password_repository: standard_password_repository(&store),
    };
    let fields = StaticAuthenticateWithPasswordFields::Valid(AuthenticateWithPasswordFields {
        login_id: stored_login_id(),
        plain_password: stored_plain_password(),
    });

    let mut action = AuthenticateWithPasswordAction::with_material(material);
    action.subscribe(holder.handler());

    let result = action.ignite(fields).await;
    assert_eq!(
        holder.extract(),
        vec![
            "authenticate with password success; user-id: test-user-id (granted: [])",
            "expansion limit calculated; 2021-01-11 10:00:00 UTC",
            "issue auth-ticket success; ticket: ticket-id / user-id: test-user-id (granted: [])",
            "token expires calculated; authenticate: 2021-01-02 10:00:00 UTC / authorize: 2021-01-01 10:01:00 UTC / cdn: 2021-01-01 10:01:00 UTC",
            "encode auth-token success",
        ],
    );
    assert!(result.is_ok());
}

#[tokio::test]
async fn error_invalid_login_id() {
    let holder = ApplicationActionStateHolder::new();

    let store = TestStore::standard();
    let material = StaticAuthenticateWithPasswordMaterial {
        issue: standard_issue_infra(&store),
        encode: standard_encode_infra(&store),
        password_repository: standard_password_repository(&store),
    };
    let fields = StaticAuthenticateWithPasswordFields::Invalid(
        ValidateAuthenticateWithPasswordFieldsError::InvalidLoginId(ValidateLoginIdError::LoginId(
            ValidateTextError::TooLong,
        )),
    );

    let mut action = AuthenticateWithPasswordAction::with_material(material);
    action.subscribe(holder.handler());

    let result = action.ignite(fields).await;
    assert_eq!(
        holder.extract(),
        vec!["authenticate with password error; invalid; login-id: too long"],
    );
    assert!(result.is_err());
}

#[tokio::test]
async fn error_failed_to_match_password() {
    let holder = ApplicationActionStateHolder::new();

    let store = TestStore::standard();
    let material = StaticAuthenticateWithPasswordMaterial {
        issue: standard_issue_infra(&store),
        encode: standard_encode_infra(&store),
        password_repository: standard_password_repository(&store),
    };
    let fields = StaticAuthenticateWithPasswordFields::Valid(AuthenticateWithPasswordFields {
        login_id: stored_login_id(),
        plain_password: PlainPassword::restore("INVALID-PASSWORD".to_owned()),
    });

    let mut action = AuthenticateWithPasswordAction::with_material(material);
    action.subscribe(holder.handler());

    let result = action.ignite(fields).await;
    assert_eq!(
        holder.extract(),
        vec!["authenticate with password error; password not matched",],
    );
    assert!(result.is_err());
}

#[tokio::test]
async fn error_no_user() {
    let holder = ApplicationActionStateHolder::new();

    let store = TestStore::standard();
    let material = StaticAuthenticateWithPasswordMaterial {
        issue: standard_issue_infra(&store),
        encode: standard_encode_infra(&store),
        password_repository: no_user_password_repository(&store),
    };
    let fields = StaticAuthenticateWithPasswordFields::Valid(AuthenticateWithPasswordFields {
        login_id: stored_login_id(),
        plain_password: stored_plain_password(),
    });

    let mut action = AuthenticateWithPasswordAction::with_material(material);
    action.subscribe(holder.handler());

    let result = action.ignite(fields).await;
    assert_eq!(
        holder.extract(),
        vec!["authenticate with password error; not found"],
    );
    assert!(result.is_err());
}

struct TestStore {
    ticket: MemoryAuthTicketStore,
    password: MemoryAuthUserStore,
}

impl TestStore {
    fn standard() -> Self {
        Self {
            ticket: MemoryAuthTicketStore::new(),
            password: MemoryAuthUserStore::new(),
        }
    }
}

fn standard_issue_infra<'a>(store: &'a TestStore) -> StaticIssueAuthTicketInfra<'a> {
    StaticIssueAuthTicketInfra {
        clock: standard_clock(),
        ticket_repository: MemoryAuthTicketRepository::new(&store.ticket),
        ticket_id_generator: StaticAuthTicketIdGenerator::new(AuthTicketId::restore(
            "ticket-id".into(),
        )),
        config: standard_issue_config(),
    }
}

fn standard_encode_infra<'a>(store: &'a TestStore) -> StaticEncodeAuthTokenInfra<'a> {
    StaticEncodeAuthTokenInfra::standard(
        standard_clock(),
        MemoryAuthTicketRepository::new(&store.ticket),
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
fn standard_issue_config() -> IssueAuthTicketConfig {
    IssueAuthTicketConfig {
        authenticate_expansion_limit: ExpansionLimitDuration::with_duration(Duration::days(10)),
    }
}

fn standard_now() -> DateTime<Utc> {
    Utc.with_ymd_and_hms(2021, 1, 1, 10, 0, 0).latest().unwrap()
}
fn standard_clock() -> StaticChronoAuthClock {
    StaticChronoAuthClock::new(standard_now())
}

fn standard_password_repository<'a>(store: &'a TestStore) -> MemoryAuthUserRepository<'a> {
    MemoryAuthUserRepository::with_user_and_password(
        &store.password,
        stored_login_id(),
        stored_user(),
        stored_hashed_password(),
        vec![],
    )
}
fn no_user_password_repository<'a>(store: &'a TestStore) -> MemoryAuthUserRepository<'a> {
    MemoryAuthUserRepository::new(&store.password)
}

fn stored_login_id() -> LoginId {
    LoginId::restore("login-id".to_owned())
}
fn stored_hashed_password() -> HashedPassword {
    HashedPassword::restore(stored_password())
}
fn stored_plain_password() -> PlainPassword {
    PlainPassword::restore(stored_password())
}
fn stored_password() -> String {
    "password".to_owned()
}

fn stored_user() -> AuthUser {
    AuthUser {
        user_id: AuthUserId::restore("test-user-id".to_owned()),
        granted: AuthPermissionGranted::default(),
    }
}

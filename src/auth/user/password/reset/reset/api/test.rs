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
        password::reset::{
            kernel::init::token::decoder::test::StaticResetTokenDecoder,
            reset::init::test::{
                StaticResetPasswordFields, StaticResetPasswordMaterial, StaticResetPasswordNotifier,
            },
        },
    },
};

use crate::auth::user::password::reset::reset::action::ResetPasswordAction;

use crate::auth::{
    ticket::{encode::infra::EncodeAuthTokenConfig, issue::infra::IssueAuthTicketConfig},
    user::password::{kernel::infra::PlainPassword, reset::reset::infra::ResetPasswordFields},
};

use crate::{
    auth::{
        kernel::data::{AuthDateTime, ExpansionLimitDuration, ExpireDuration},
        ticket::kernel::data::{AuthPermissionGranted, AuthTicketId},
        user::{
            kernel::data::{AuthUser, AuthUserId},
            login_id::kernel::data::{LoginId, ValidateLoginIdError},
            password::reset::{
                kernel::data::{
                    ResetPasswordId, ResetPasswordToken, ResetPasswordTokenDestination,
                    ResetPasswordTokenDestinationEmail,
                },
                reset::data::ValidateResetPasswordFieldsError,
            },
        },
    },
    common::api::validate::data::ValidateTextError,
};

#[tokio::test]
async fn info() {
    let store = TestStore::standard();
    let material = StaticResetPasswordMaterial {
        issue: standard_issue_infra(&store),
        encode: standard_encode_infra(&store),
        clock: standard_clock(),
        reset_password_repository: standard_reset_token_repository(&store),
        token_decoder: StaticResetTokenDecoder::Valid(stored_reset_token()),
        reset_notifier: StaticResetPasswordNotifier,
    };

    let action = ResetPasswordAction::with_material(material);

    assert_eq!(action.info.name(), "auth.user.password.reset");
}

#[tokio::test]
async fn success_reset_password() {
    let holder = ApplicationActionStateHolder::new();

    let store = TestStore::standard();
    let material = StaticResetPasswordMaterial {
        issue: standard_issue_infra(&store),
        encode: standard_encode_infra(&store),
        clock: standard_clock(),
        reset_password_repository: standard_reset_token_repository(&store),
        token_decoder: StaticResetTokenDecoder::Valid(stored_reset_token()),
        reset_notifier: StaticResetPasswordNotifier,
    };
    let fields = StaticResetPasswordFields::Valid(ResetPasswordFields {
        reset_token: ResetPasswordToken::restore("TOKEN".to_owned()),
        login_id: stored_login_id(),
        new_password: PlainPassword::restore("new-password".to_owned()),
    });

    let mut action = ResetPasswordAction::with_material(material);
    action.subscribe(holder.handler());

    let result = action.ignite(fields).await;
    assert_eq!(
        holder.extract(),
        vec![
            "reset password notified; message-id: message-id",
            "reset password success; user-id: user-id (granted: [])",
            "expansion limit calculated; 2021-01-11 10:00:00 UTC",
            "issue auth-ticket success; ticket: ticket-id / user-id: user-id (granted: [])",
            "token expires calculated; authenticate: 2021-01-02 10:00:00 UTC / authorize: 2021-01-01 10:01:00 UTC / cdn: 2021-01-01 10:01:00 UTC",
            "encode auth-token success",
        ],
    );
    assert!(result.is_ok());
}

#[tokio::test]
async fn error_match_failed_login_id() {
    let holder = ApplicationActionStateHolder::new();

    let store = TestStore::standard();
    let material = StaticResetPasswordMaterial {
        issue: standard_issue_infra(&store),
        encode: standard_encode_infra(&store),
        clock: standard_clock(),
        reset_password_repository: standard_reset_token_repository(&store),
        token_decoder: StaticResetTokenDecoder::Valid(stored_reset_token()),
        reset_notifier: StaticResetPasswordNotifier,
    };
    let fields = StaticResetPasswordFields::Valid(ResetPasswordFields {
        reset_token: ResetPasswordToken::restore("TOKEN".to_owned()),
        login_id: LoginId::restore("INVALID-LOGIN-ID".to_owned()),
        new_password: PlainPassword::restore("new-password".to_owned()),
    });

    let mut action = ResetPasswordAction::with_material(material);
    action.subscribe(holder.handler());

    let result = action.ignite(fields).await;
    assert_eq!(
        holder.extract(),
        vec!["reset password error; login id not matched"],
    );
    assert!(result.is_err());
}

#[tokio::test]
async fn error_invalid_login_id() {
    let holder = ApplicationActionStateHolder::new();

    let store = TestStore::standard();
    let material = StaticResetPasswordMaterial {
        issue: standard_issue_infra(&store),
        encode: standard_encode_infra(&store),
        clock: standard_clock(),
        reset_password_repository: standard_reset_token_repository(&store),
        token_decoder: StaticResetTokenDecoder::Valid(stored_reset_token()),
        reset_notifier: StaticResetPasswordNotifier,
    };
    let fields =
        StaticResetPasswordFields::Invalid(ValidateResetPasswordFieldsError::InvalidLoginId(
            ValidateLoginIdError::LoginId(ValidateTextError::Empty),
        ));

    let mut action = ResetPasswordAction::with_material(material);
    action.subscribe(holder.handler());

    let result = action.ignite(fields).await;
    assert_eq!(
        holder.extract(),
        vec!["reset password error; invalid; login-id: empty"],
    );
    assert!(result.is_err());
}

#[tokio::test]
async fn error_reset_token_expired_at_decode() {
    let holder = ApplicationActionStateHolder::new();

    let store = TestStore::standard();
    let material = StaticResetPasswordMaterial {
        issue: standard_issue_infra(&store),
        encode: standard_encode_infra(&store),
        clock: standard_clock(),
        reset_password_repository: standard_reset_token_repository(&store),
        token_decoder: StaticResetTokenDecoder::Expired,
        reset_notifier: StaticResetPasswordNotifier,
    };
    let fields = StaticResetPasswordFields::Valid(ResetPasswordFields {
        reset_token: ResetPasswordToken::restore("TOKEN".to_owned()),
        login_id: stored_login_id(),
        new_password: PlainPassword::restore("new-password".to_owned()),
    });

    let mut action = ResetPasswordAction::with_material(material);
    action.subscribe(holder.handler());

    let result = action.ignite(fields).await;
    assert_eq!(
        holder.extract(),
        vec!["reset password error; reset token expired"],
    );
    assert!(result.is_err());
}

#[tokio::test]
async fn error_reset_token_expired_in_store() {
    let holder = ApplicationActionStateHolder::new();

    let store = TestStore::standard();
    let material = StaticResetPasswordMaterial {
        issue: standard_issue_infra(&store),
        encode: standard_encode_infra(&store),
        clock: standard_clock(),
        reset_password_repository: expired_reset_token_repository(&store),
        token_decoder: StaticResetTokenDecoder::Valid(stored_reset_token()),
        reset_notifier: StaticResetPasswordNotifier,
    };
    let fields = StaticResetPasswordFields::Valid(ResetPasswordFields {
        reset_token: ResetPasswordToken::restore("TOKEN".to_owned()),
        login_id: stored_login_id(),
        new_password: PlainPassword::restore("new-password".to_owned()),
    });

    let mut action = ResetPasswordAction::with_material(material);
    action.subscribe(holder.handler());

    let result = action.ignite(fields).await;
    assert_eq!(
        holder.extract(),
        vec!["reset password error; reset token expired"],
    );
    assert!(result.is_err());
}

#[tokio::test]
async fn error_already_reset() {
    let holder = ApplicationActionStateHolder::new();

    let store = TestStore::standard();
    let material = StaticResetPasswordMaterial {
        issue: standard_issue_infra(&store),
        encode: standard_encode_infra(&store),
        clock: standard_clock(),
        reset_password_repository: already_reset_token_repository(&store),
        token_decoder: StaticResetTokenDecoder::Valid(stored_reset_token()),
        reset_notifier: StaticResetPasswordNotifier,
    };
    let fields = StaticResetPasswordFields::Valid(ResetPasswordFields {
        reset_token: ResetPasswordToken::restore("TOKEN".to_owned()),
        login_id: stored_login_id(),
        new_password: PlainPassword::restore("new-password".to_owned()),
    });

    let mut action = ResetPasswordAction::with_material(material);
    action.subscribe(holder.handler());

    let result = action.ignite(fields).await;
    assert_eq!(
        holder.extract(),
        vec!["reset password error; already reset"],
    );
    assert!(result.is_err());
}

#[tokio::test]
async fn error_reset_token_not_stored() {
    let holder = ApplicationActionStateHolder::new();

    let store = TestStore::standard();
    let material = StaticResetPasswordMaterial {
        issue: standard_issue_infra(&store),
        encode: standard_encode_infra(&store),
        clock: standard_clock(),
        reset_password_repository: no_reset_token_repository(&store),
        token_decoder: StaticResetTokenDecoder::Valid(stored_reset_token()),
        reset_notifier: StaticResetPasswordNotifier,
    };
    let fields = StaticResetPasswordFields::Valid(ResetPasswordFields {
        reset_token: ResetPasswordToken::restore("TOKEN".to_owned()),
        login_id: stored_login_id(),
        new_password: PlainPassword::restore("new-password".to_owned()),
    });

    let mut action = ResetPasswordAction::with_material(material);
    action.subscribe(holder.handler());

    let result = action.ignite(fields).await;
    assert_eq!(holder.extract(), vec!["reset password error; not found"]);
    assert!(result.is_err());
}

struct TestStore {
    ticket: MemoryAuthTicketStore,
    reset_password: MemoryAuthUserStore,
}

impl TestStore {
    fn standard() -> Self {
        Self {
            ticket: MemoryAuthTicketStore::new(),
            reset_password: MemoryAuthUserStore::new(),
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

fn standard_reset_token_repository<'a>(store: &'a TestStore) -> MemoryAuthUserRepository<'a> {
    let requested_at = AuthDateTime::restore(standard_now());
    let expires = requested_at.expires(&ExpireDuration::with_duration(Duration::days(1)));
    MemoryAuthUserRepository::with_user_and_reset_token(
        &store.reset_password,
        stored_login_id(),
        test_user(),
        stored_reset_token(),
        stored_destination(),
        expires,
        requested_at,
        None,
    )
}
fn no_reset_token_repository<'a>(store: &'a TestStore) -> MemoryAuthUserRepository<'a> {
    MemoryAuthUserRepository::new(&store.reset_password)
}
fn expired_reset_token_repository<'a>(store: &'a TestStore) -> MemoryAuthUserRepository<'a> {
    let requested_at = AuthDateTime::restore(standard_now());
    let expires = requested_at.expires(&ExpireDuration::with_duration(Duration::days(-1)));
    MemoryAuthUserRepository::with_user_and_reset_token(
        &store.reset_password,
        stored_login_id(),
        test_user(),
        stored_reset_token(),
        stored_destination(),
        expires,
        requested_at,
        None,
    )
}
fn already_reset_token_repository<'a>(store: &'a TestStore) -> MemoryAuthUserRepository<'a> {
    let requested_at = AuthDateTime::restore(standard_now());
    let expires = requested_at.expires(&ExpireDuration::with_duration(Duration::days(1)));
    let reset_at = AuthDateTime::restore(standard_now() - Duration::days(1));
    MemoryAuthUserRepository::with_user_and_reset_token(
        &store.reset_password,
        stored_login_id(),
        test_user(),
        stored_reset_token(),
        stored_destination(),
        expires,
        requested_at,
        Some(reset_at),
    )
}

fn test_user() -> AuthUser {
    AuthUser {
        user_id: AuthUserId::restore("user-id".to_owned()),
        granted: AuthPermissionGranted::default(),
    }
}
fn stored_login_id() -> LoginId {
    LoginId::restore("login-id".to_owned())
}
fn stored_reset_token() -> ResetPasswordId {
    ResetPasswordId::restore("RESET-TOKEN".to_owned())
}
fn stored_destination() -> ResetPasswordTokenDestination {
    ResetPasswordTokenDestination::Email(ResetPasswordTokenDestinationEmail::restore(
        "user@example.com".to_owned(),
    ))
}

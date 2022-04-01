use std::collections::HashSet;

use chrono::{DateTime, Duration, TimeZone, Utc};
use getto_application_test::ActionTestRunner;

use crate::auth::{
    ticket::{
        kernel::init::clock::test::StaticChronoAuthClock,
        validate::init::{
            nonce_metadata::test::StaticAuthNonceMetadata,
            nonce_repository::memory::{
                MemoryAuthNonceMap, MemoryAuthNonceRepository, MemoryAuthNonceStore,
            },
            test::{StaticValidateAuthNonceStruct, StaticValidateAuthTokenStruct},
            token_decoder::test::StaticAuthTokenDecoder,
            token_metadata::test::StaticAuthTokenMetadata,
        },
    },
    user::{
        kernel::init::user_repository::memory::{
            MemoryAuthUserMap, MemoryAuthUserRepository, MemoryAuthUserStore,
        },
        password::reset::token_destination::change::init::request_decoder::test::StaticChangeResetTokenDestinationRequestDecoder,
    },
};

use crate::auth::user::password::reset::token_destination::change::action::{
    ChangeResetTokenDestinationAction, ChangeResetTokenDestinationMaterial,
};

use crate::auth::ticket::validate::method::AuthNonceConfig;

use crate::auth::user::password::reset::token_destination::change::infra::ChangeResetTokenDestinationFields;

use crate::auth::{
    ticket::kernel::data::{AuthDateTime, AuthTicketExtract, ExpireDuration},
    user::{
        kernel::data::AuthUserId,
        login_id::kernel::data::LoginId,
        password::reset::kernel::data::{ResetTokenDestination, ResetTokenDestinationExtract},
    },
};

#[tokio::test]
async fn success_change_destination() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let material = TestStruct::standard(&store);
    let request_decoder = standard_request_decoder();

    let mut action = ChangeResetTokenDestinationAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "validate success; ticket: ticket-id / user: user-id (granted: [])",
        "change reset token destination success; reset token destination: email(user@example.com)",
    ]);
    assert!(result.is_ok());
}

#[tokio::test]
async fn success_expired_nonce() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::expired_nonce();
    let material = TestStruct::standard(&store);
    let request_decoder = standard_request_decoder();

    let mut action = ChangeResetTokenDestinationAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "validate success; ticket: ticket-id / user: user-id (granted: [])",
        "change reset token destination success; reset token destination: email(user@example.com)",
    ]);
    assert!(result.is_ok());
}

#[tokio::test]
async fn error_conflict_nonce() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::conflict_nonce();
    let material = TestStruct::standard(&store);
    let request_decoder = standard_request_decoder();

    let mut action = ChangeResetTokenDestinationAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce error; conflict",
    ]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn error_conflict_changes() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let material = TestStruct::standard(&store);
    let request_decoder = conflict_request_decoder();

    let mut action = ChangeResetTokenDestinationAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "validate success; ticket: ticket-id / user: user-id (granted: [])",
        "change reset token destination error; changes conflicted",
    ]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn error_not_found() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let material = TestStruct::standard(&store);
    let request_decoder = not_found_request_decoder();

    let mut action = ChangeResetTokenDestinationAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "validate success; ticket: ticket-id / user: user-id (granted: [])",
        "change reset token destination error; not found",
    ]);
    assert!(!result.is_ok());
}

struct TestStruct<'a> {
    validate: StaticValidateAuthTokenStruct<'a>,
    destination_repository: MemoryAuthUserRepository<'a>,
}

impl<'a> ChangeResetTokenDestinationMaterial for TestStruct<'a> {
    type Validate = StaticValidateAuthTokenStruct<'a>;

    type DestinationRepository = MemoryAuthUserRepository<'a>;

    fn validate(&self) -> &Self::Validate {
        &self.validate
    }
    fn destination_repository(&self) -> &Self::DestinationRepository {
        &self.destination_repository
    }
}

struct TestStore {
    nonce: MemoryAuthNonceStore,
    user: MemoryAuthUserStore,
}

impl TestStore {
    fn standard() -> Self {
        Self {
            nonce: standard_nonce_store(),
            user: standard_login_id_store(),
        }
    }
    fn expired_nonce() -> Self {
        Self {
            nonce: expired_nonce_store(),
            user: standard_login_id_store(),
        }
    }
    fn conflict_nonce() -> Self {
        Self {
            nonce: conflict_nonce_store(),
            user: standard_login_id_store(),
        }
    }
}

impl<'a> TestStruct<'a> {
    fn standard(store: &'a TestStore) -> Self {
        Self {
            validate: StaticValidateAuthTokenStruct {
                validate_nonce: StaticValidateAuthNonceStruct {
                    config: standard_nonce_config(),
                    clock: standard_clock(),
                    nonce_metadata: standard_nonce_header(),
                    nonce_repository: MemoryAuthNonceRepository::new(&store.nonce),
                },
                token_metadata: standard_token_header(),
                token_decoder: standard_token_decoder(),
            },
            destination_repository: MemoryAuthUserRepository::new(&store.user),
        }
    }
}

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

fn standard_nonce_header() -> StaticAuthNonceMetadata {
    StaticAuthNonceMetadata::new(NONCE.into())
}
fn standard_token_header() -> StaticAuthTokenMetadata {
    StaticAuthTokenMetadata::new("TOKEN".into())
}

fn standard_token_decoder() -> StaticAuthTokenDecoder {
    StaticAuthTokenDecoder::Valid(AuthTicketExtract {
        ticket_id: TICKET_ID.into(),
        user_id: USER_ID.into(),
        granted_roles: HashSet::new(),
    })
}

const NONCE: &'static str = "nonce";
const TICKET_ID: &'static str = "ticket-id";
const USER_ID: &'static str = "user-id";
const LOGIN_ID: &'static str = "login-id";

fn standard_request_decoder() -> StaticChangeResetTokenDestinationRequestDecoder {
    StaticChangeResetTokenDestinationRequestDecoder::Valid(ChangeResetTokenDestinationFields {
        login_id: LoginId::restore(LOGIN_ID.into()),
        from: ResetTokenDestination::None,
        to: ResetTokenDestination::restore(ResetTokenDestinationExtract::Email(
            "user@example.com".into(),
        )),
    })
}
fn conflict_request_decoder() -> StaticChangeResetTokenDestinationRequestDecoder {
    StaticChangeResetTokenDestinationRequestDecoder::Valid(ChangeResetTokenDestinationFields {
        login_id: LoginId::restore(LOGIN_ID.into()),
        from: ResetTokenDestination::restore(ResetTokenDestinationExtract::Email(
            "user@example.com".into(),
        )),
        to: ResetTokenDestination::restore(ResetTokenDestinationExtract::Email(
            "user@example.com".into(),
        )),
    })
}
fn not_found_request_decoder() -> StaticChangeResetTokenDestinationRequestDecoder {
    StaticChangeResetTokenDestinationRequestDecoder::Valid(ChangeResetTokenDestinationFields {
        login_id: LoginId::restore("unknown-user".into()),
        from: ResetTokenDestination::None,
        to: ResetTokenDestination::restore(ResetTokenDestinationExtract::Email(
            "user@example.com".into(),
        )),
    })
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

fn standard_login_id_store() -> MemoryAuthUserStore {
    MemoryAuthUserMap::with_user_id_and_destination(
        test_login_id(),
        test_user_id(),
        test_destination(),
    )
    .to_store()
}

fn test_login_id() -> LoginId {
    LoginId::restore(LOGIN_ID.into())
}
fn test_user_id() -> AuthUserId {
    AuthUserId::restore(USER_ID.into())
}
fn test_destination() -> ResetTokenDestination {
    ResetTokenDestination::None
}

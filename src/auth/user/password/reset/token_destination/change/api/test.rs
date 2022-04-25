use std::collections::HashSet;

use chrono::{DateTime, Duration, TimeZone, Utc};
use getto_application_test::ActionTestRunner;

use crate::auth::{
    ticket::{
        kernel::init::clock::test::StaticChronoAuthClock,
        validate::init::{
            nonce_metadata::test::StaticAuthNonceMetadata,
            nonce_repository::memory::{MemoryAuthNonceRepository, MemoryAuthNonceStore},
            test::{StaticValidateAuthNonceStruct, StaticValidateAuthTokenStruct},
            token_decoder::test::StaticAuthTokenDecoder,
            token_metadata::test::StaticAuthTokenMetadata,
        },
    },
    user::{
        kernel::init::user_repository::memory::{MemoryAuthUserRepository, MemoryAuthUserStore},
        password::reset::token_destination::change::init::request_decoder::test::StaticChangeResetTokenDestinationRequestDecoder,
    },
};

use crate::auth::user::password::reset::token_destination::change::action::{
    ChangeResetTokenDestinationAction, ChangeResetTokenDestinationMaterial,
};

use crate::auth::ticket::validate::method::AuthNonceConfig;

use crate::auth::user::password::reset::token_destination::change::infra::ChangeResetTokenDestinationFieldsExtract;

use crate::auth::{
    ticket::kernel::data::{AuthTicketExtract, ExpireDuration},
    user::{
        kernel::data::AuthUserId,
        login_id::kernel::data::LoginId,
        password::reset::kernel::data::{ResetTokenDestination, ResetTokenDestinationExtract},
    },
};

#[tokio::test]
async fn success_change_destination() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::new();
    let material = TestStruct::standard(&store);
    let request_decoder = standard_request_decoder();

    let mut action = ChangeResetTokenDestinationAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "validate success; ticket: ticket-id / user: user-id (granted: [user])",
        "change reset token destination success",
    ]);
    assert!(result.is_ok());
}

#[tokio::test]
async fn permission_denied() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::new();
    let material = TestStruct::not_permitted(&store);
    let request_decoder = standard_request_decoder();

    let mut action = ChangeResetTokenDestinationAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "validate success; ticket: ticket-id / user: user-id (granted: [])",
        "user permission denied; granted: [], require: any [user]",
    ]);
    assert!(result.is_err());
}

#[tokio::test]
async fn error_conflict_changes() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::new();
    let material = TestStruct::standard(&store);
    let request_decoder = conflict_request_decoder();

    let mut action = ChangeResetTokenDestinationAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "validate success; ticket: ticket-id / user: user-id (granted: [user])",
        "change reset token destination error; changes conflicted",
    ]);
    assert!(result.is_err());
}

#[tokio::test]
async fn error_not_found() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::new();
    let material = TestStruct::standard(&store);
    let request_decoder = not_found_request_decoder();

    let mut action = ChangeResetTokenDestinationAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "validate success; ticket: ticket-id / user: user-id (granted: [user])",
        "change reset token destination error; not found",
    ]);
    assert!(result.is_err());
}

#[tokio::test]
async fn error_invalid_email() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::new();
    let material = TestStruct::standard(&store);
    let request_decoder = invalid_email_request_decoder();

    let mut action = ChangeResetTokenDestinationAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "validate success; ticket: ticket-id / user: user-id (granted: [user])",
        "change reset token destination error; invalid; to: invalid email",
    ]);
    assert!(result.is_err());
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
    destination: MemoryAuthUserStore,
}

impl TestStore {
    fn new() -> Self {
        Self {
            nonce: MemoryAuthNonceStore::new(),
            destination: MemoryAuthUserStore::new(),
        }
    }
}

impl<'a> TestStruct<'a> {
    fn standard(store: &'a TestStore) -> Self {
        Self::new(store, standard_token_decoder())
    }
    fn not_permitted(store: &'a TestStore) -> Self {
        Self::new(store, not_permitted_token_decoder())
    }
    fn new(store: &'a TestStore, token_decoder: StaticAuthTokenDecoder) -> Self {
        Self {
            validate: StaticValidateAuthTokenStruct {
                validate_nonce: StaticValidateAuthNonceStruct {
                    config: standard_nonce_config(),
                    clock: standard_clock(),
                    nonce_metadata: standard_nonce_header(),
                    nonce_repository: MemoryAuthNonceRepository::new(&store.nonce),
                },
                token_metadata: standard_token_header(),
                token_decoder,
            },
            destination_repository: standard_destination_repository(&store.destination),
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
        granted_roles: standard_granted_roles(),
    })
}
fn not_permitted_token_decoder() -> StaticAuthTokenDecoder {
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
    StaticChangeResetTokenDestinationRequestDecoder::Valid(
        ChangeResetTokenDestinationFieldsExtract {
            login_id: LOGIN_ID.into(),
            from: Some(ResetTokenDestinationExtract::None),
            to: Some(ResetTokenDestinationExtract::Email(
                "user@example.com".into(),
            )),
        },
    )
}
fn conflict_request_decoder() -> StaticChangeResetTokenDestinationRequestDecoder {
    StaticChangeResetTokenDestinationRequestDecoder::Valid(
        ChangeResetTokenDestinationFieldsExtract {
            login_id: LOGIN_ID.into(),
            from: Some(ResetTokenDestinationExtract::Email(
                "user@example.com".into(),
            )),
            to: Some(ResetTokenDestinationExtract::Email(
                "user@example.com".into(),
            )),
        },
    )
}
fn not_found_request_decoder() -> StaticChangeResetTokenDestinationRequestDecoder {
    StaticChangeResetTokenDestinationRequestDecoder::Valid(
        ChangeResetTokenDestinationFieldsExtract {
            login_id: "unknown-user".into(),
            from: Some(ResetTokenDestinationExtract::None),
            to: Some(ResetTokenDestinationExtract::Email(
                "user@example.com".into(),
            )),
        },
    )
}
fn invalid_email_request_decoder() -> StaticChangeResetTokenDestinationRequestDecoder {
    StaticChangeResetTokenDestinationRequestDecoder::Valid(
        ChangeResetTokenDestinationFieldsExtract {
            login_id: LOGIN_ID.into(),
            from: Some(ResetTokenDestinationExtract::None),
            to: Some(ResetTokenDestinationExtract::Email("invalid-email".into())),
        },
    )
}

fn standard_destination_repository<'a>(
    store: &'a MemoryAuthUserStore,
) -> MemoryAuthUserRepository<'a> {
    MemoryAuthUserRepository::with_user_id_and_destination(
        store,
        standard_login_id(),
        standard_user_id(),
        standard_destination(),
    )
}

fn standard_login_id() -> LoginId {
    LoginId::restore(LOGIN_ID.into())
}
fn standard_user_id() -> AuthUserId {
    AuthUserId::restore(USER_ID.into())
}
fn standard_destination() -> ResetTokenDestination {
    ResetTokenDestination::None
}
fn standard_granted_roles() -> HashSet<String> {
    vec!["user".to_owned()].into_iter().collect()
}

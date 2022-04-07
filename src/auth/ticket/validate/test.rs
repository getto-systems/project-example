use std::collections::HashSet;

use getto_application_test::ActionTestRunner;

use chrono::{DateTime, Duration, TimeZone, Utc};

use crate::auth::ticket::{
    kernel::{data::AuthNonce, init::clock::test::StaticChronoAuthClock},
    validate::init::{
        nonce_metadata::test::StaticAuthNonceMetadata,
        nonce_repository::memory::MemoryAuthNonceRepository,
        request_decoder::test::StaticValidateApiTokenRequestDecoder,
        test::{StaticValidateAuthNonceStruct, StaticValidateAuthTokenStruct},
        token_decoder::test::StaticAuthTokenDecoder,
        token_metadata::test::StaticAuthTokenMetadata,
    },
};

use crate::auth::ticket::validate::method::AuthNonceConfig;

use super::action::ValidateApiTokenAction;

use crate::auth::{
    ticket::kernel::data::{AuthDateTime, AuthTicketExtract, ExpireDuration},
    user::kernel::data::RequireAuthRoles,
};

#[tokio::test]
async fn success_allow_for_any_role() {
    let (handler, assert_state) = ActionTestRunner::new();

    let material = TestStruct::standard();
    let request_decoder = allow_any_role_request_decoder();

    let mut action = ValidateApiTokenAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "validate success; ticket: ticket-id / user: user-role-user-id (granted: [user])",
        "validate api token success; user: user-role-user-id (granted: [user])",
    ]);
    assert!(result.is_ok());
}

#[tokio::test]
async fn success_allow_for_user_role() {
    let (handler, assert_state) = ActionTestRunner::new();

    let material = TestStruct::standard();
    let request_decoder = allow_user_role_request_decoder();

    let mut action = ValidateApiTokenAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "validate success; ticket: ticket-id / user: user-role-user-id (granted: [user])",
        "validate api token success; user: user-role-user-id (granted: [user])",
    ]);
    assert!(result.is_ok());
}

#[tokio::test]
async fn error_allow_for_user_role_but_not_granted() {
    let (handler, assert_state) = ActionTestRunner::new();

    let material = TestStruct::no_granted_roles();
    let request_decoder = allow_user_role_request_decoder();

    let mut action = ValidateApiTokenAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "validate success; ticket: ticket-id / user: no-role-user-id (granted: [])",
        "user permission denied; granted: [], require: any [user]",
    ]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn error_token_expired() {
    let (handler, assert_state) = ActionTestRunner::new();

    let material = TestStruct::token_expired();
    let request_decoder = allow_user_role_request_decoder();

    let mut action = ValidateApiTokenAction::with_material(request_decoder, material);
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
async fn error_conflict_nonce() {
    let (handler, assert_state) = ActionTestRunner::new();

    let material = TestStruct::conflict_nonce();
    let request_decoder = allow_user_role_request_decoder();

    let mut action = ValidateApiTokenAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce error; conflict",
    ]);
    assert!(!result.is_ok());
}

struct TestStruct;

impl TestStruct {
    fn standard() -> StaticValidateAuthTokenStruct {
        Self::with_token_validator(standard_nonce_repository(), standard_token_decoder())
    }
    fn conflict_nonce() -> StaticValidateAuthTokenStruct {
        Self::with_token_validator(conflict_nonce_repository(), standard_token_decoder())
    }
    fn no_granted_roles() -> StaticValidateAuthTokenStruct {
        Self::with_token_validator(
            standard_nonce_repository(),
            no_granted_roles_token_decoder(),
        )
    }
    fn token_expired() -> StaticValidateAuthTokenStruct {
        Self::with_token_validator(standard_nonce_repository(), expired_token_decoder())
    }

    fn with_token_validator(
        nonce_repository: MemoryAuthNonceRepository,
        token_decoder: StaticAuthTokenDecoder,
    ) -> StaticValidateAuthTokenStruct {
        StaticValidateAuthTokenStruct {
            validate_nonce: StaticValidateAuthNonceStruct {
                config: standard_nonce_config(),
                clock: standard_clock(),
                nonce_metadata: standard_nonce_header(),
                nonce_repository,
            },
            token_metadata: standard_token_header(),
            token_decoder,
        }
    }
}

const NONCE: &'static str = "nonce";
const TICKET_ID: &'static str = "ticket-id";

fn allow_any_role_request_decoder() -> StaticValidateApiTokenRequestDecoder {
    StaticValidateApiTokenRequestDecoder {
        require_roles: RequireAuthRoles::Nothing,
    }
}
fn allow_user_role_request_decoder() -> StaticValidateApiTokenRequestDecoder {
    StaticValidateApiTokenRequestDecoder {
        require_roles: RequireAuthRoles::restore_has_any(vec!["user"]),
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
    let mut granted_roles = HashSet::new();
    granted_roles.insert("user".into());

    StaticAuthTokenDecoder::Valid(AuthTicketExtract {
        ticket_id: TICKET_ID.into(),
        user_id: "user-role-user-id".into(),
        granted_roles,
    })
}
fn no_granted_roles_token_decoder() -> StaticAuthTokenDecoder {
    StaticAuthTokenDecoder::Valid(AuthTicketExtract {
        ticket_id: TICKET_ID.into(),
        user_id: "no-role-user-id".into(),
        granted_roles: HashSet::new(),
    })
}
fn expired_token_decoder() -> StaticAuthTokenDecoder {
    StaticAuthTokenDecoder::Expired
}

fn standard_nonce_repository() -> MemoryAuthNonceRepository {
    MemoryAuthNonceRepository::new()
}
fn conflict_nonce_repository() -> MemoryAuthNonceRepository {
    let registered_at = AuthDateTime::restore(standard_now());
    let expires = registered_at.expires(&ExpireDuration::with_duration(Duration::days(1)));
    MemoryAuthNonceRepository::with_nonce(test_nonce(), expires, registered_at)
}

fn test_nonce() -> AuthNonce {
    AuthNonce::restore(NONCE.into())
}

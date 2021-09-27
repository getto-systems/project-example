use std::collections::HashSet;

use chrono::{DateTime, Duration, TimeZone, Utc};
use getto_application_test::ActionTestRunner;

use crate::auth::{
    ticket::remote::{
        check_nonce::init::{
            nonce_repository::test::{
                MemoryAuthNonceMap, MemoryAuthNonceRepository, MemoryAuthNonceStore,
            },
            test::StaticCheckAuthNonceStruct,
        },
        kernel::init::{
            clock::test::StaticChronoAuthClock, nonce_metadata::test::StaticAuthNonceMetadata,
            token_decoder::test::StaticAuthTokenDecoder,
            token_metadata::test::StaticAuthTokenMetadata,
        },
        validate::init::test::StaticValidateAuthTokenStruct,
    },
    password::remote::{
        change::init::{
            request_decoder::test::StaticChangePasswordRequestDecoder,
            test::StaticChangePasswordStruct,
        },
        kernel::init::password_repository::test::{
            MemoryAuthUserPasswordMap, MemoryAuthUserPasswordRepository,
            MemoryAuthUserPasswordStore,
        },
    },
};

use crate::auth::{
    ticket::remote::check_nonce::infra::AuthNonceConfig,
    password::remote::{
        kernel::infra::HashedPassword, proxy_change::infra::ChangePasswordFieldsExtract,
    },
};

use super::action::{ChangePasswordAction, ChangePasswordMaterial};

use crate::auth::{
    ticket::remote::kernel::data::{AuthDateTime, AuthTicketExtract, ExpireDuration},
    auth_user::remote::kernel::data::{AuthUser, AuthUserExtract},
    login_id::remote::data::LoginId,
};

#[tokio::test]
async fn success_change() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let feature = TestFeature::standard(&store);
    let request_decoder = standard_request_decoder();

    let mut action = ChangePasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite(request_decoder).await;
    assert_state(vec![
        "validate success; ticket: ticket-id / user: user-id (granted: [])",
        "change password success",
    ]);
    assert!(result.is_ok());
}

#[tokio::test]
async fn success_expired_nonce() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::expired_nonce();
    let feature = TestFeature::standard(&store);
    let request_decoder = standard_request_decoder();

    let mut action = ChangePasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite(request_decoder).await;
    assert_state(vec![
        "validate success; ticket: ticket-id / user: user-id (granted: [])",
        "change password success",
    ]);
    assert!(result.is_ok());
}

#[tokio::test]
async fn error_conflict_nonce() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::conflict_nonce();
    let feature = TestFeature::standard(&store);
    let request_decoder = standard_request_decoder();

    let mut action = ChangePasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite(request_decoder).await;
    assert_state(vec!["validate error; auth nonce error: conflict"]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn error_empty_current_password() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let feature = TestFeature::standard(&store);
    let request_decoder = empty_current_password_request_decoder();

    let mut action = ChangePasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite(request_decoder).await;
    assert_state(vec![
        "validate success; ticket: ticket-id / user: user-id (granted: [])",
        "change password error; invalid current password: empty password",
    ]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn error_too_long_current_password() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let feature = TestFeature::standard(&store);
    let request_decoder = too_long_current_password_request_decoder();

    let mut action = ChangePasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite(request_decoder).await;
    assert_state(vec![
        "validate success; ticket: ticket-id / user: user-id (granted: [])",
        "change password error; invalid current password: too long password",
    ]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn just_max_length_current_password() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let feature = TestFeature::standard(&store);
    let request_decoder = just_max_length_current_password_request_decoder();

    let mut action = ChangePasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite(request_decoder).await;
    assert_state(vec![
        "validate success; ticket: ticket-id / user: user-id (granted: [])",
        "change password error; password not matched",
    ]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn error_empty_new_password() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let feature = TestFeature::standard(&store);
    let request_decoder = empty_new_password_request_decoder();

    let mut action = ChangePasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite(request_decoder).await;
    assert_state(vec![
        "validate success; ticket: ticket-id / user: user-id (granted: [])",
        "change password error; invalid new password: empty password",
    ]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn error_too_long_new_password() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let feature = TestFeature::standard(&store);
    let request_decoder = too_long_new_password_request_decoder();

    let mut action = ChangePasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite(request_decoder).await;
    assert_state(vec![
        "validate success; ticket: ticket-id / user: user-id (granted: [])",
        "change password error; invalid new password: too long password",
    ]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn just_max_length_new_password() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let feature = TestFeature::standard(&store);
    let request_decoder = just_max_length_new_password_request_decoder();

    let mut action = ChangePasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite(request_decoder).await;
    assert_state(vec![
        "validate success; ticket: ticket-id / user: user-id (granted: [])",
        "change password success",
    ]);
    assert!(result.is_ok());
}

#[tokio::test]
async fn error_failed_to_match_password() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::match_fail_password();
    let feature = TestFeature::standard(&store);
    let request_decoder = standard_request_decoder();

    let mut action = ChangePasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite(request_decoder).await;
    assert_state(vec![
        "validate success; ticket: ticket-id / user: user-id (granted: [])",
        "change password error; password not matched",
    ]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn error_password_not_stored() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::password_not_stored();
    let feature = TestFeature::standard(&store);
    let request_decoder = standard_request_decoder();

    let mut action = ChangePasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite(request_decoder).await;
    assert_state(vec![
        "validate success; ticket: ticket-id / user: user-id (granted: [])",
        "change password error; password not found",
    ]);
    assert!(!result.is_ok());
}

struct TestFeature<'a> {
    change: StaticChangePasswordStruct<'a>,
}

impl<'a> ChangePasswordMaterial for TestFeature<'a> {
    type Change = StaticChangePasswordStruct<'a>;

    fn change(&self) -> &Self::Change {
        &self.change
    }
}

struct TestStore {
    nonce: MemoryAuthNonceStore,
    password: MemoryAuthUserPasswordStore,
}

impl TestStore {
    fn standard() -> Self {
        Self {
            nonce: standard_nonce_store(),
            password: standard_password_store(),
        }
    }
    fn match_fail_password() -> Self {
        Self {
            nonce: standard_nonce_store(),
            password: match_fail_password_store(),
        }
    }
    fn password_not_stored() -> Self {
        Self {
            nonce: standard_nonce_store(),
            password: not_stored_password_store(),
        }
    }
    fn expired_nonce() -> Self {
        Self {
            nonce: expired_nonce_store(),
            password: standard_password_store(),
        }
    }
    fn conflict_nonce() -> Self {
        Self {
            nonce: conflict_nonce_store(),
            password: standard_password_store(),
        }
    }
}

impl<'a> TestFeature<'a> {
    fn standard(store: &'a TestStore) -> Self {
        Self {
            change: StaticChangePasswordStruct {
                validate_infra: StaticValidateAuthTokenStruct {
                    check_nonce_infra: StaticCheckAuthNonceStruct {
                        config: standard_nonce_config(),
                        clock: standard_clock(),
                        nonce_metadata: standard_nonce_header(),
                        nonce_repository: MemoryAuthNonceRepository::new(&store.nonce),
                    },
                    token_metadata: standard_token_header(),
                    token_decoder: standard_token_decoder(),
                },
                password_repository: MemoryAuthUserPasswordRepository::new(&store.password),
            },
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
const PASSWORD: &'static str = "current-password";
const ANOTHER_PASSWORD: &'static str = "another-password";

fn standard_request_decoder() -> StaticChangePasswordRequestDecoder {
    StaticChangePasswordRequestDecoder::Valid(ChangePasswordFieldsExtract {
        current_password: "current-password".into(),
        new_password: "new-password".into(),
    })
}
fn empty_current_password_request_decoder() -> StaticChangePasswordRequestDecoder {
    StaticChangePasswordRequestDecoder::Valid(ChangePasswordFieldsExtract {
        current_password: "".into(),
        new_password: "new-password".into(),
    })
}
fn too_long_current_password_request_decoder() -> StaticChangePasswordRequestDecoder {
    StaticChangePasswordRequestDecoder::Valid(ChangePasswordFieldsExtract {
        current_password: vec!["a"; 100 + 1].join(""),
        new_password: "new-password".into(),
    })
}
fn just_max_length_current_password_request_decoder() -> StaticChangePasswordRequestDecoder {
    StaticChangePasswordRequestDecoder::Valid(ChangePasswordFieldsExtract {
        current_password: vec!["a"; 100].join(""),
        new_password: "new-password".into(),
    })
}
fn empty_new_password_request_decoder() -> StaticChangePasswordRequestDecoder {
    StaticChangePasswordRequestDecoder::Valid(ChangePasswordFieldsExtract {
        current_password: "current-password".into(),
        new_password: "".into(),
    })
}
fn too_long_new_password_request_decoder() -> StaticChangePasswordRequestDecoder {
    StaticChangePasswordRequestDecoder::Valid(ChangePasswordFieldsExtract {
        current_password: "current-password".into(),
        new_password: vec!["a"; 100 + 1].join(""),
    })
}
fn just_max_length_new_password_request_decoder() -> StaticChangePasswordRequestDecoder {
    StaticChangePasswordRequestDecoder::Valid(ChangePasswordFieldsExtract {
        current_password: "current-password".into(),
        new_password: vec!["a"; 100].join(""),
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

fn standard_password_store() -> MemoryAuthUserPasswordStore {
    MemoryAuthUserPasswordMap::with_password(
        test_user_login_id(),
        test_user(),
        test_user_password(),
    )
    .to_store()
}
fn match_fail_password_store() -> MemoryAuthUserPasswordStore {
    MemoryAuthUserPasswordMap::with_password(test_user_login_id(), test_user(), another_password())
        .to_store()
}
fn not_stored_password_store() -> MemoryAuthUserPasswordStore {
    MemoryAuthUserPasswordMap::new().to_store()
}

fn test_user() -> AuthUser {
    let mut granted_roles = HashSet::new();
    granted_roles.insert("something".into());

    AuthUserExtract {
        user_id: USER_ID.into(),
        granted_roles,
    }
    .restore()
}
fn test_user_login_id() -> LoginId {
    LoginId::restore(LOGIN_ID.into())
}
fn test_user_password() -> HashedPassword {
    HashedPassword::restore(PASSWORD.into())
}
fn another_password() -> HashedPassword {
    HashedPassword::restore(ANOTHER_PASSWORD.into())
}

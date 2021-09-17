use std::collections::HashSet;

use getto_application_test::ActionTestRunner;

use crate::auth::password::_auth::{
    change::init::{
        request_decoder::test::StaticChangePasswordRequestDecoder, test::StaticChangePasswordStruct,
    },
    kernel::init::password_repository::test::{
        MemoryAuthUserPasswordMap, MemoryAuthUserPasswordRepository, MemoryAuthUserPasswordStore,
    },
};

use crate::auth::password::{
    _auth::kernel::infra::HashedPassword, _common::change::infra::ChangePasswordFieldsExtract,
};

use super::action::{ChangePasswordAction, ChangePasswordMaterial};

use crate::auth::{
    auth_user::_common::kernel::data::{AuthUser, AuthUserExtract, AuthUserId},
    login_id::_auth::data::LoginId,
};

#[tokio::test]
async fn success_change() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let feature = TestFeature::new(&store);
    let request_decoder = standard_request_decoder();

    let mut action = ChangePasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite(request_decoder).await;
    assert_state(vec!["change password success"]);
    assert!(result.is_ok());
}

#[tokio::test]
async fn error_empty_current_password() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let feature = TestFeature::new(&store);
    let request_decoder = empty_current_password_request_decoder();

    let mut action = ChangePasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite(request_decoder).await;
    assert_state(vec![
        "change password error; invalid current password: empty password",
    ]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn error_too_long_current_password() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let feature = TestFeature::new(&store);
    let request_decoder = too_long_current_password_request_decoder();

    let mut action = ChangePasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite(request_decoder).await;
    assert_state(vec![
        "change password error; invalid current password: too long password",
    ]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn just_max_length_current_password() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let feature = TestFeature::new(&store);
    let request_decoder = just_max_length_current_password_request_decoder();

    let mut action = ChangePasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite(request_decoder).await;
    assert_state(vec!["change password error; password not matched"]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn error_empty_new_password() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let feature = TestFeature::new(&store);
    let request_decoder = empty_new_password_request_decoder();

    let mut action = ChangePasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite(request_decoder).await;
    assert_state(vec![
        "change password error; invalid new password: empty password",
    ]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn error_too_long_new_password() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let feature = TestFeature::new(&store);
    let request_decoder = too_long_new_password_request_decoder();

    let mut action = ChangePasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite(request_decoder).await;
    assert_state(vec![
        "change password error; invalid new password: too long password",
    ]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn just_max_length_new_password() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let feature = TestFeature::new(&store);
    let request_decoder = just_max_length_new_password_request_decoder();

    let mut action = ChangePasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite(request_decoder).await;
    assert_state(vec!["change password success"]);
    assert!(result.is_ok());
}

#[tokio::test]
async fn error_failed_to_match_password() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::match_fail_password();
    let feature = TestFeature::new(&store);
    let request_decoder = standard_request_decoder();

    let mut action = ChangePasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite(request_decoder).await;
    assert_state(vec!["change password error; password not matched"]);
    assert!(!result.is_ok());
}

#[tokio::test]
async fn error_password_not_stored() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::password_not_stored();
    let feature = TestFeature::new(&store);
    let request_decoder = standard_request_decoder();

    let mut action = ChangePasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite(request_decoder).await;
    assert_state(vec!["change password error; password not found"]);
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
    password: MemoryAuthUserPasswordStore,
}

impl TestStore {
    fn standard() -> Self {
        Self {
            password: standard_password_store(),
        }
    }
    fn match_fail_password() -> Self {
        Self {
            password: match_fail_password_store(),
        }
    }
    fn password_not_stored() -> Self {
        Self {
            password: not_stored_password_store(),
        }
    }
}

impl<'a> TestFeature<'a> {
    fn new(store: &'a TestStore) -> Self {
        Self {
            change: StaticChangePasswordStruct {
                password_repository: MemoryAuthUserPasswordRepository::new(&store.password),
            },
        }
    }
}

const USER_ID: &'static str = "user-id";
const LOGIN_ID: &'static str = "login-id";
const PASSWORD: &'static str = "current-password";
const ANOTHER_PASSWORD: &'static str = "another-password";

fn standard_request_decoder() -> StaticChangePasswordRequestDecoder {
    StaticChangePasswordRequestDecoder::Valid(
        AuthUserId::restore(USER_ID.into()),
        ChangePasswordFieldsExtract {
            current_password: "current-password".into(),
            new_password: "new-password".into(),
        },
    )
}
fn empty_current_password_request_decoder() -> StaticChangePasswordRequestDecoder {
    StaticChangePasswordRequestDecoder::Valid(
        AuthUserId::restore(USER_ID.into()),
        ChangePasswordFieldsExtract {
            current_password: "".into(),
            new_password: "new-password".into(),
        },
    )
}
fn too_long_current_password_request_decoder() -> StaticChangePasswordRequestDecoder {
    StaticChangePasswordRequestDecoder::Valid(
        AuthUserId::restore(USER_ID.into()),
        ChangePasswordFieldsExtract {
            current_password: vec!["a"; 100 + 1].join(""),
            new_password: "new-password".into(),
        },
    )
}
fn just_max_length_current_password_request_decoder() -> StaticChangePasswordRequestDecoder {
    StaticChangePasswordRequestDecoder::Valid(
        AuthUserId::restore(USER_ID.into()),
        ChangePasswordFieldsExtract {
            current_password: vec!["a"; 100].join(""),
            new_password: "new-password".into(),
        },
    )
}
fn empty_new_password_request_decoder() -> StaticChangePasswordRequestDecoder {
    StaticChangePasswordRequestDecoder::Valid(
        AuthUserId::restore(USER_ID.into()),
        ChangePasswordFieldsExtract {
            current_password: "current-password".into(),
            new_password: "".into(),
        },
    )
}
fn too_long_new_password_request_decoder() -> StaticChangePasswordRequestDecoder {
    StaticChangePasswordRequestDecoder::Valid(
        AuthUserId::restore(USER_ID.into()),
        ChangePasswordFieldsExtract {
            current_password: "current-password".into(),
            new_password: vec!["a"; 100 + 1].join(""),
        },
    )
}
fn just_max_length_new_password_request_decoder() -> StaticChangePasswordRequestDecoder {
    StaticChangePasswordRequestDecoder::Valid(
        AuthUserId::restore(USER_ID.into()),
        ChangePasswordFieldsExtract {
            current_password: "current-password".into(),
            new_password: vec!["a"; 100].join(""),
        },
    )
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

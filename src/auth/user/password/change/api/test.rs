use getto_application_test::ApplicationActionStateHolder;
use pretty_assertions::assert_eq;

use crate::auth::{
    ticket::{
        authorize::init::test::StaticAuthorizeInfra,
        kernel::{data::AuthPermissionGranted, init::request::test::StaticAuthorizeToken},
    },
    user::{
        kernel::init::user_repository::memory::{MemoryAuthUserRepository, MemoryAuthUserStore},
        password::change::init::test::{StaticChangePasswordFields, StaticChangePasswordMaterial},
    },
};

use crate::auth::user::password::change::action::ChangePasswordAction;

use crate::auth::user::password::{
    change::infra::ChangePasswordFields,
    kernel::infra::{HashedPassword, PlainPassword},
};

use crate::{
    auth::user::{
        kernel::data::{AuthUser, AuthUserId},
        login_id::kernel::data::LoginId,
        password::{
            change::data::ValidateChangePasswordFieldsError, kernel::data::ValidatePasswordError,
        },
    },
    common::api::validate::data::ValidateTextError,
};

#[tokio::test]
async fn info() {
    let store = TestStore::new();
    let material = StaticChangePasswordMaterial {
        authorize: StaticAuthorizeInfra::new(operator_user_id()),
        password_repository: standard_password_repository(&store),
    };

    let action = ChangePasswordAction::with_material(material);

    let (name, required) = action.info.params();
    assert_eq!(
        format!("{}; {}", name, required),
        "auth.user.password.change; require: nothing",
    );
}

#[tokio::test]
async fn success_change() {
    let holder = ApplicationActionStateHolder::new();

    let store = TestStore::new();
    let material = StaticChangePasswordMaterial {
        authorize: StaticAuthorizeInfra::new(operator_user_id()),
        password_repository: standard_password_repository(&store),
    };
    let fields = StaticChangePasswordFields::Valid(ChangePasswordFields {
        current_password: stored_plain_password(),
        new_password: PlainPassword::restore("new-password".to_owned()),
    });

    let mut action = ChangePasswordAction::with_material(material);
    action.subscribe(holder.handler());

    let result = action.ignite(StaticAuthorizeToken, fields).await;
    assert_eq!(
        holder.extract(),
        vec![
            "try to proxy call: auth.ticket.authorize.clarify(require: nothing)",
            "proxy call success",
            "change password success",
        ],
    );
    assert!(result.is_ok());
}

#[tokio::test]
async fn error_invalid_current_password() {
    let holder = ApplicationActionStateHolder::new();

    let store = TestStore::new();
    let material = StaticChangePasswordMaterial {
        authorize: StaticAuthorizeInfra::new(operator_user_id()),
        password_repository: standard_password_repository(&store),
    };
    let fields = StaticChangePasswordFields::Invalid(
        ValidateChangePasswordFieldsError::InvalidCurrentPassword(ValidatePasswordError::Password(
            ValidateTextError::TooLong,
        )),
    );

    let mut action = ChangePasswordAction::with_material(material);
    action.subscribe(holder.handler());

    let result = action.ignite(StaticAuthorizeToken, fields).await;
    assert_eq!(
        holder.extract(),
        vec![
            "try to proxy call: auth.ticket.authorize.clarify(require: nothing)",
            "proxy call success",
            "change password error; invalid; current: password: too long",
        ],
    );
    assert!(result.is_err());
}

#[tokio::test]
async fn error_failed_to_match_password() {
    let holder = ApplicationActionStateHolder::new();

    let store = TestStore::new();
    let material = StaticChangePasswordMaterial {
        authorize: StaticAuthorizeInfra::new(operator_user_id()),
        password_repository: standard_password_repository(&store),
    };
    let fields = StaticChangePasswordFields::Valid(ChangePasswordFields {
        current_password: PlainPassword::restore("UNKNOWN-PASSWORD".to_owned()),
        new_password: PlainPassword::restore("new-password".to_owned()),
    });

    let mut action = ChangePasswordAction::with_material(material);
    action.subscribe(holder.handler());

    let result = action.ignite(StaticAuthorizeToken, fields).await;
    assert_eq!(
        holder.extract(),
        vec![
            "try to proxy call: auth.ticket.authorize.clarify(require: nothing)",
            "proxy call success",
            "change password error; password not matched",
        ],
    );
    assert!(result.is_err());
}

#[tokio::test]
async fn error_password_not_stored() {
    let holder = ApplicationActionStateHolder::new();

    let store = TestStore::new();
    let material = StaticChangePasswordMaterial {
        authorize: StaticAuthorizeInfra::new(operator_user_id()),
        password_repository: no_user_password_repository(&store),
    };
    let fields = StaticChangePasswordFields::Valid(ChangePasswordFields {
        current_password: stored_plain_password(),
        new_password: PlainPassword::restore("new-password".to_owned()),
    });

    let mut action = ChangePasswordAction::with_material(material);
    action.subscribe(holder.handler());

    let result = action.ignite(StaticAuthorizeToken, fields).await;
    assert_eq!(
        holder.extract(),
        vec![
            "try to proxy call: auth.ticket.authorize.clarify(require: nothing)",
            "proxy call success",
            "change password error; not found",
        ],
    );
    assert!(result.is_err());
}

struct TestStore {
    password: MemoryAuthUserStore,
}

impl TestStore {
    fn new() -> Self {
        Self {
            password: MemoryAuthUserStore::new(),
        }
    }
}

fn standard_password_repository<'a>(store: &'a TestStore) -> MemoryAuthUserRepository<'a> {
    MemoryAuthUserRepository::with_user_and_password(
        &store.password,
        stored_login_id(),
        stored_user(),
        stored_password(),
        vec![],
    )
}
fn no_user_password_repository<'a>(store: &'a TestStore) -> MemoryAuthUserRepository<'a> {
    MemoryAuthUserRepository::new(&store.password)
}

fn stored_user() -> AuthUser {
    AuthUser {
        user_id: AuthUserId::restore("user-id".to_owned()),
        granted: AuthPermissionGranted::default(),
    }
}
fn stored_login_id() -> LoginId {
    LoginId::restore("login-id".to_owned())
}
fn stored_plain_password() -> PlainPassword {
    PlainPassword::restore("password".to_owned())
}
fn stored_password() -> HashedPassword {
    HashedPassword::restore("password".to_owned())
}

fn operator_user_id() -> AuthUserId {
    stored_user().user_id
}

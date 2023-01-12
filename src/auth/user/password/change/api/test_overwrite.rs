use getto_application_test::ApplicationActionStateHolder;
use pretty_assertions::assert_eq;

use crate::x_content::permission::AuthPermission;

use crate::auth::{
    ticket::{
        authorize::init::test::StaticAuthorizeInfra,
        kernel::init::request::test::StaticAuthorizeToken,
    },
    user::{
        kernel::init::user_repository::memory::{MemoryAuthUserRepository, MemoryAuthUserStore},
        password::change::init::test::{
            StaticOverwritePasswordFields, StaticOverwritePasswordMaterial,
        },
    },
};

use crate::auth::user::password::change::action::OverwritePasswordAction;

use crate::auth::user::password::{
    change::infra::OverwritePasswordFields,
    kernel::infra::{HashedPassword, PlainPassword},
};

use crate::{
    auth::{
        ticket::kernel::data::AuthPermissionGranted,
        user::{
            kernel::data::{AuthUser, AuthUserId},
            login_id::kernel::data::LoginId,
            password::{
                change::data::ValidateOverwritePasswordFieldsError,
                kernel::data::ValidatePasswordError,
            },
        },
    },
    common::api::validate::data::ValidateTextError,
};

#[tokio::test]
async fn info() {
    let store = TestStore::new();
    let material = StaticOverwritePasswordMaterial {
        authorize: StaticAuthorizeInfra::new(operator_user_id()),
        password_repository: standard_password_repository(&store),
    };

    let action = OverwritePasswordAction::with_material(material);

    let (name, required) = action.info.params();
    assert_eq!(
        format!("{}; {}", name, required),
        "auth.user.password.overwrite; require: some [auth-user]",
    );
}

#[tokio::test]
async fn success_overwrite() {
    let holder = ApplicationActionStateHolder::new();

    let store = TestStore::new();
    let material = StaticOverwritePasswordMaterial {
        authorize: StaticAuthorizeInfra::new(operator_user_id()),
        password_repository: standard_password_repository(&store),
    };
    let fields = StaticOverwritePasswordFields::Valid(OverwritePasswordFields {
        login_id: stored_login_id(),
        new_password: PlainPassword::restore("new-password".to_owned()),
    });

    let mut action = OverwritePasswordAction::with_material(material);
    action.subscribe(holder.handler());

    let result = action.ignite(StaticAuthorizeToken, fields).await;
    assert_eq!(
        holder.extract(),
        vec![
            "try to proxy call: auth.ticket.authorize.clarify(require: some [auth-user])",
            "proxy call success",
            "overwrite password success",
        ],
    );
    assert!(result.is_ok());
}

#[tokio::test]
async fn error_invalid_password() {
    let holder = ApplicationActionStateHolder::new();

    let store = TestStore::new();
    let material = StaticOverwritePasswordMaterial {
        authorize: StaticAuthorizeInfra::new(operator_user_id()),
        password_repository: standard_password_repository(&store),
    };
    let fields = StaticOverwritePasswordFields::Invalid(
        ValidateOverwritePasswordFieldsError::InvalidNewPassword(ValidatePasswordError::Password(
            ValidateTextError::TooLong,
        )),
    );

    let mut action = OverwritePasswordAction::with_material(material);
    action.subscribe(holder.handler());

    let result = action.ignite(StaticAuthorizeToken, fields).await;
    assert_eq!(
        holder.extract(),
        vec![
            "try to proxy call: auth.ticket.authorize.clarify(require: some [auth-user])",
            "proxy call success",
            "overwrite password error; invalid; new-password: password: too long",
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

fn stored_user() -> AuthUser {
    AuthUser {
        user_id: AuthUserId::restore("user-id".to_owned()),
        granted: AuthPermissionGranted::restore(
            vec![AuthPermission::AuthUser].into_iter().collect(),
        ),
    }
}
fn stored_login_id() -> LoginId {
    LoginId::restore("login-id".into())
}
fn stored_password() -> HashedPassword {
    HashedPassword::restore("password".into())
}

fn operator_user_id() -> AuthUserId {
    AuthUserId::restore("operator-user-id".to_owned())
}

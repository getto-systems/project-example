use getto_application_test::ApplicationActionStateHolder;
use pretty_assertions::assert_eq;

use crate::auth::{
    ticket::{
        authorize::init::test::StaticAuthorizeInfra,
        kernel::init::request::test::StaticAuthorizeToken,
    },
    user::{
        kernel::init::user_repository::memory::{MemoryAuthUserRepository, MemoryAuthUserStore},
        login_id::change::init::test::{
            StaticOverwriteLoginIdFields, StaticOverwriteLoginIdMaterial,
        },
    },
};

use crate::auth::user::login_id::change::action::OverwriteLoginIdAction;

use crate::auth::user::{
    login_id::change::infra::OverwriteLoginIdFields, password::kernel::infra::HashedPassword,
};

use crate::{
    auth::{
        ticket::kernel::data::AuthPermissionGranted,
        user::{
            kernel::data::{AuthUser, AuthUserId},
            login_id::{
                change::data::ValidateOverwriteLoginIdFieldsError,
                kernel::data::{LoginId, ValidateLoginIdError},
            },
        },
    },
    common::api::validate::data::ValidateTextError,
};

#[tokio::test]
async fn info() {
    let store = TestStore::new();
    let material = StaticOverwriteLoginIdMaterial {
        authorize: StaticAuthorizeInfra::new(operator_user_id()),
        login_id_repository: standard_login_id_repository(&store),
    };

    let action = OverwriteLoginIdAction::with_material(material);

    let (name, required) = action.info.params();
    assert_eq!(
        format!("{}; {}", name, required),
        "auth.user.login-id.change; require: some [auth-user]",
    );
}

#[tokio::test]
async fn success_overwrite() {
    let holder = ApplicationActionStateHolder::new();

    let store = TestStore::new();
    let material = StaticOverwriteLoginIdMaterial {
        authorize: StaticAuthorizeInfra::new(operator_user_id()),
        login_id_repository: standard_login_id_repository(&store),
    };
    let fields = StaticOverwriteLoginIdFields::Valid(OverwriteLoginIdFields {
        login_id: stored_login_id(),
        new_login_id: LoginId::restore("new-login-id".to_owned()),
    });

    let mut action = OverwriteLoginIdAction::with_material(material);
    action.subscribe(holder.handler());

    let result = action.ignite(StaticAuthorizeToken, fields).await;
    assert_eq!(
        holder.extract(),
        vec![
            "try to proxy call: auth.ticket.authorize.clarify(require: some [auth-user])",
            "proxy call success",
            "overwrite login-id success",
        ],
    );
    assert!(result.is_ok());
}

#[tokio::test]
async fn error_invalid_new_login_id() {
    let holder = ApplicationActionStateHolder::new();

    let store = TestStore::new();
    let material = StaticOverwriteLoginIdMaterial {
        authorize: StaticAuthorizeInfra::new(operator_user_id()),
        login_id_repository: standard_login_id_repository(&store),
    };
    let fields = StaticOverwriteLoginIdFields::Invalid(
        ValidateOverwriteLoginIdFieldsError::InvalidNewLoginId(ValidateLoginIdError::LoginId(
            ValidateTextError::Empty,
        )),
    );

    let mut action = OverwriteLoginIdAction::with_material(material);
    action.subscribe(holder.handler());

    let result = action.ignite(StaticAuthorizeToken, fields).await;
    assert_eq!(
        holder.extract(),
        vec![
            "try to proxy call: auth.ticket.authorize.clarify(require: some [auth-user])",
            "proxy call success",
            "overwrite login-id error; invalid; new: login-id: empty",
        ],
    );
    assert!(result.is_err());
}

#[tokio::test]
async fn error_login_id_already_registered() {
    let holder = ApplicationActionStateHolder::new();

    let store = TestStore::new();
    let material = StaticOverwriteLoginIdMaterial {
        authorize: StaticAuthorizeInfra::new(operator_user_id()),
        login_id_repository: standard_login_id_repository(&store),
    };
    let fields = StaticOverwriteLoginIdFields::Valid(OverwriteLoginIdFields {
        login_id: stored_login_id(),
        new_login_id: stored_another_login_id(),
    });

    let mut action = OverwriteLoginIdAction::with_material(material);
    action.subscribe(holder.handler());

    let result = action.ignite(StaticAuthorizeToken, fields).await;
    assert_eq!(
        holder.extract(),
        vec![
            "try to proxy call: auth.ticket.authorize.clarify(require: some [auth-user])",
            "proxy call success",
            "overwrite login-id error; new login id is already registered",
        ],
    );
    assert!(result.is_err());
}

struct TestStore {
    login_id: MemoryAuthUserStore,
}

impl TestStore {
    fn new() -> Self {
        Self {
            login_id: MemoryAuthUserStore::new(),
        }
    }
}

fn standard_login_id_repository<'a>(store: &'a TestStore) -> MemoryAuthUserRepository<'a> {
    MemoryAuthUserRepository::with_user_and_password(
        &store.login_id,
        stored_login_id(),
        stored_user(),
        stored_password(),
        vec![(stored_another_login_id(), stored_another_user_id())],
    )
}

fn stored_user() -> AuthUser {
    AuthUser {
        user_id: AuthUserId::restore("user-id".to_owned()),
        granted: AuthPermissionGranted::default(),
    }
}
fn stored_login_id() -> LoginId {
    LoginId::restore("login-id".into())
}
fn stored_password() -> HashedPassword {
    HashedPassword::restore("hashed-password".into())
}

fn stored_another_user_id() -> AuthUserId {
    AuthUserId::restore("another-user-id".into())
}
fn stored_another_login_id() -> LoginId {
    LoginId::restore("another-login-id".into())
}

fn operator_user_id() -> AuthUserId {
    AuthUserId::restore("operator-user-id".to_owned())
}

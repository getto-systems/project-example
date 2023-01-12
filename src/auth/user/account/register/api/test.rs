use getto_application_test::ApplicationActionStateHolder;
use pretty_assertions::assert_eq;

use crate::auth::{
    ticket::{
        authorize::init::test::StaticAuthorizeInfra,
        kernel::init::request::test::StaticAuthorizeToken,
    },
    user::{
        account::register::init::test::{
            StaticAuthUserIdGenerator, StaticRegisterAuthUserAccountFields,
            StaticRegisterAuthUserAccountMaterial,
        },
        kernel::init::user_repository::memory::{MemoryAuthUserRepository, MemoryAuthUserStore},
    },
};

use crate::auth::user::account::register::action::RegisterAuthUserAccountAction;

use crate::auth::user::password::kernel::infra::HashedPassword;

use crate::{
    auth::{
        ticket::kernel::data::AuthPermissionGranted,
        user::{
            account::kernel::data::{
                AuthUserAccount, AuthUserAccountAttrs, AuthUserMemo, ValidateAuthUserAccountError,
            },
            kernel::data::{AuthUser, AuthUserId},
            login_id::kernel::data::LoginId,
            password::reset::kernel::data::{
                ResetPasswordTokenDestination, ResetPasswordTokenDestinationEmail,
            },
        },
    },
    common::api::validate::data::ValidateTextError,
};

#[tokio::test]
async fn info() {
    let store = TestStore::new();
    let material = StaticRegisterAuthUserAccountMaterial {
        authorize: StaticAuthorizeInfra::new(operator_user_id()),
        user_id_generator: standard_user_id_generator(),
        user_repository: standard_user_repository(&store),
    };

    let action = RegisterAuthUserAccountAction::with_material(material);

    let (name, required) = action.info.params();
    assert_eq!(
        format!("{}; {}", name, required),
        "auth.user.account.register; require: some [auth-user]"
    );
}

#[tokio::test]
async fn success_register_user() {
    let holder = ApplicationActionStateHolder::new();

    let store = TestStore::new();
    let material = StaticRegisterAuthUserAccountMaterial {
        authorize: StaticAuthorizeInfra::new(operator_user_id()),
        user_id_generator: standard_user_id_generator(),
        user_repository: standard_user_repository(&store),
    };
    let fields = StaticRegisterAuthUserAccountFields::Valid(AuthUserAccount {
        login_id: LoginId::restore("new-login-id".to_owned()),
        attrs: AuthUserAccountAttrs {
            granted: AuthPermissionGranted::default(),
            memo: AuthUserMemo::restore("memo".to_owned()),
        },
        reset_token_destination: ResetPasswordTokenDestination::Email(
            ResetPasswordTokenDestinationEmail::restore("user@example.com".to_owned()),
        ),
    });

    let mut action = RegisterAuthUserAccountAction::with_material(material);
    action.subscribe(holder.handler());

    let result = action.ignite(StaticAuthorizeToken, fields).await;
    assert_eq!(
        holder.extract(),
        vec![
            "try to proxy call: auth.ticket.authorize.clarify(require: some [auth-user])",
            "proxy call success",
            "register auth user account success",
        ],
    );
    assert!(result.is_ok());
}

#[tokio::test]
async fn error_login_id_already_registered() {
    let holder = ApplicationActionStateHolder::new();

    let store = TestStore::new();
    let material = StaticRegisterAuthUserAccountMaterial {
        authorize: StaticAuthorizeInfra::new(operator_user_id()),
        user_id_generator: standard_user_id_generator(),
        user_repository: standard_user_repository(&store),
    };
    let fields = StaticRegisterAuthUserAccountFields::Valid(AuthUserAccount {
        login_id: stored_login_id(),
        attrs: AuthUserAccountAttrs {
            granted: AuthPermissionGranted::default(),
            memo: AuthUserMemo::restore("memo".to_owned()),
        },
        reset_token_destination: ResetPasswordTokenDestination::Email(
            ResetPasswordTokenDestinationEmail::restore("user@example.com".to_owned()),
        ),
    });

    let mut action = RegisterAuthUserAccountAction::with_material(material);
    action.subscribe(holder.handler());

    let result = action.ignite(StaticAuthorizeToken, fields).await;
    assert_eq!(
        holder.extract(),
        vec![
            "try to proxy call: auth.ticket.authorize.clarify(require: some [auth-user])",
            "proxy call success",
            "register auth user account error; login-id already registered",
        ],
    );
    assert!(result.is_err());
}

#[tokio::test]
async fn error_invalid_memo() {
    let holder = ApplicationActionStateHolder::new();

    let store = TestStore::new();
    let material = StaticRegisterAuthUserAccountMaterial {
        authorize: StaticAuthorizeInfra::new(operator_user_id()),
        user_id_generator: standard_user_id_generator(),
        user_repository: standard_user_repository(&store),
    };
    let fields = StaticRegisterAuthUserAccountFields::Invalid(ValidateAuthUserAccountError::Memo(
        ValidateTextError::TooLong,
    ));

    let mut action = RegisterAuthUserAccountAction::with_material(material);
    action.subscribe(holder.handler());

    let result = action.ignite(StaticAuthorizeToken, fields).await;
    assert_eq!(
        holder.extract(),
        vec![
            "try to proxy call: auth.ticket.authorize.clarify(require: some [auth-user])",
            "proxy call success",
            "register auth user account error; invalid; memo: too long",
        ],
    );
    assert!(result.is_err());
}

struct TestStore {
    user: MemoryAuthUserStore,
}

impl TestStore {
    fn new() -> Self {
        Self {
            user: MemoryAuthUserStore::new(),
        }
    }
}

fn standard_user_id_generator() -> StaticAuthUserIdGenerator {
    StaticAuthUserIdGenerator::new(AuthUserId::restore("new-user-id".to_owned()))
}

fn standard_user_repository<'a>(store: &'a TestStore) -> MemoryAuthUserRepository<'a> {
    MemoryAuthUserRepository::with_user_and_password(
        &store.user,
        stored_login_id(),
        stored_user(),
        stored_password(),
        vec![],
    )
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
fn stored_password() -> HashedPassword {
    HashedPassword::restore("password".to_owned())
}

fn operator_user_id() -> AuthUserId {
    AuthUserId::restore("operator-user-id".to_owned())
}

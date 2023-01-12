use getto_application_test::ApplicationActionStateHolder;
use pretty_assertions::assert_eq;

use crate::x_content::permission::AuthPermission;

use crate::auth::{
    ticket::{
        authorize::init::test::StaticAuthorizeInfra,
        kernel::init::request::test::StaticAuthorizeToken,
    },
    user::{
        account::modify::init::test::{
            StaticModifyAuthUserAccountFields, StaticModifyAuthUserAccountMaterial,
        },
        kernel::init::user_repository::memory::{MemoryAuthUserRepository, MemoryAuthUserStore},
    },
};

use crate::auth::user::account::modify::action::ModifyAuthUserAccountAction;

use crate::auth::user::account::modify::infra::ModifyAuthUserAccountFields;

use crate::{
    auth::{
        ticket::kernel::data::AuthPermissionGranted,
        user::{
            account::{
                kernel::data::{AuthUserAccountAttrs, AuthUserMemo, ValidateAuthUserAccountError},
                modify::data::ValidateModifyAuthUserAccountFieldsError,
            },
            kernel::data::AuthUserId,
            login_id::kernel::data::LoginId,
        },
    },
    common::api::validate::data::ValidateTextError,
};

#[tokio::test]
async fn info() {
    let store = TestStore::new();
    let material = StaticModifyAuthUserAccountMaterial {
        authorize: StaticAuthorizeInfra::new(operator_user_id()),
        user_repository: standard_user_repository(&store),
    };

    let action = ModifyAuthUserAccountAction::with_material(material);

    let (name, required) = action.info.params();
    assert_eq!(
        format!("{}; {}", name, required),
        "auth.user.account.modify; require: some [auth-user]",
    );
}

#[tokio::test]
async fn success_modify_user() {
    let holder = ApplicationActionStateHolder::new();

    let store = TestStore::new();
    let material = StaticModifyAuthUserAccountMaterial {
        authorize: StaticAuthorizeInfra::new(operator_user_id()),
        user_repository: standard_user_repository(&store),
    };
    let fields = StaticModifyAuthUserAccountFields::Valid(ModifyAuthUserAccountFields {
        login_id: stored_login_id(),
        from: stored_user_attrs(),
        to: AuthUserAccountAttrs {
            memo: AuthUserMemo::restore("changing-memo".to_owned()),
            ..stored_user_attrs()
        },
    });

    let mut action = ModifyAuthUserAccountAction::with_material(material);
    action.subscribe(holder.handler());

    let result = action.ignite(StaticAuthorizeToken, fields).await;
    assert_eq!(
        holder.extract(),
        vec![
            "try to proxy call: auth.ticket.authorize.clarify(require: some [auth-user])",
            "proxy call success",
            "modify auth user account success",
        ],
    );
    assert!(result.is_ok());
}

#[tokio::test]
async fn error_conflict_changes() {
    let holder = ApplicationActionStateHolder::new();

    let store = TestStore::new();
    let material = StaticModifyAuthUserAccountMaterial {
        authorize: StaticAuthorizeInfra::new(operator_user_id()),
        user_repository: standard_user_repository(&store),
    };
    let fields = StaticModifyAuthUserAccountFields::Valid(ModifyAuthUserAccountFields {
        login_id: stored_login_id(),
        from: AuthUserAccountAttrs {
            memo: AuthUserMemo::restore("conflicted-memo".to_owned()),
            ..stored_user_attrs()
        },
        to: AuthUserAccountAttrs {
            memo: AuthUserMemo::restore("changing-memo".to_owned()),
            ..stored_user_attrs()
        },
    });

    let mut action = ModifyAuthUserAccountAction::with_material(material);
    action.subscribe(holder.handler());

    let result = action.ignite(StaticAuthorizeToken, fields).await;
    assert_eq!(
        holder.extract(),
        vec![
            "try to proxy call: auth.ticket.authorize.clarify(require: some [auth-user])",
            "proxy call success",
            "modify auth user account error; changes conflicted",
        ],
    );
    assert!(result.is_err());
}

#[tokio::test]
async fn error_not_found() {
    let holder = ApplicationActionStateHolder::new();

    let store = TestStore::new();
    let material = StaticModifyAuthUserAccountMaterial {
        authorize: StaticAuthorizeInfra::new(operator_user_id()),
        user_repository: standard_user_repository(&store),
    };
    let fields = StaticModifyAuthUserAccountFields::Valid(ModifyAuthUserAccountFields {
        login_id: LoginId::restore("UNKNOWN-LOGIN-ID".to_owned()),
        from: stored_user_attrs(),
        to: AuthUserAccountAttrs {
            memo: AuthUserMemo::restore("changing-memo".to_owned()),
            ..stored_user_attrs()
        },
    });

    let mut action = ModifyAuthUserAccountAction::with_material(material);
    action.subscribe(holder.handler());

    let result = action.ignite(StaticAuthorizeToken, fields).await;
    assert_eq!(
        holder.extract(),
        vec![
            "try to proxy call: auth.ticket.authorize.clarify(require: some [auth-user])",
            "proxy call success",
            "modify auth user account error; not found",
        ],
    );
    assert!(result.is_err());
}

#[tokio::test]
async fn error_invalid_memo() {
    let holder = ApplicationActionStateHolder::new();

    let store = TestStore::new();
    let material = StaticModifyAuthUserAccountMaterial {
        authorize: StaticAuthorizeInfra::new(operator_user_id()),
        user_repository: standard_user_repository(&store),
    };
    let fields = StaticModifyAuthUserAccountFields::Invalid(
        ValidateModifyAuthUserAccountFieldsError::InvalidTo(ValidateAuthUserAccountError::Memo(
            ValidateTextError::TooLong,
        )),
    );

    let mut action = ModifyAuthUserAccountAction::with_material(material);
    action.subscribe(holder.handler());

    let result = action.ignite(StaticAuthorizeToken, fields).await;
    assert_eq!(
        holder.extract(),
        vec![
            "try to proxy call: auth.ticket.authorize.clarify(require: some [auth-user])",
            "proxy call success",
            "modify auth user account error; invalid to; memo: too long",
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

fn standard_user_repository<'a>(store: &'a TestStore) -> MemoryAuthUserRepository<'a> {
    MemoryAuthUserRepository::with_user(
        &store.user,
        stored_user_id(),
        stored_login_id(),
        stored_user_attrs(),
    )
}

fn operator_user_id() -> AuthUserId {
    AuthUserId::restore("operator-user-id".to_owned())
}
fn stored_user_id() -> AuthUserId {
    AuthUserId::restore("user-id".to_owned())
}
fn stored_user_attrs() -> AuthUserAccountAttrs {
    AuthUserAccountAttrs {
        granted: AuthPermissionGranted::restore(
            vec![AuthPermission::AuthUser].into_iter().collect(),
        ),
        memo: AuthUserMemo::restore("memo".to_owned()),
    }
}
fn stored_login_id() -> LoginId {
    LoginId::restore("login-id".into())
}

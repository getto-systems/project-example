use getto_application_test::ApplicationActionStateHolder;
use pretty_assertions::assert_eq;

use crate::auth::{
    ticket::{
        authorize::init::test::StaticAuthorizeInfra,
        kernel::init::{
            request::test::StaticAuthorizeToken,
            ticket_repository::memory::{MemoryAuthTicketRepository, MemoryAuthTicketStore},
        },
    },
    user::{
        account::unregister::init::test::{
            StaticUnregisterAuthUserAccountFields, StaticUnregisterAuthUserAccountMaterial,
        },
        kernel::init::user_repository::memory::{MemoryAuthUserRepository, MemoryAuthUserStore},
    },
};

use crate::auth::user::account::unregister::action::UnregisterAuthUserAccountAction;

use crate::auth::user::{
    account::unregister::infra::UnregisterAuthUserAccountFields,
    password::kernel::infra::HashedPassword,
};

use crate::auth::{
    ticket::kernel::data::AuthPermissionGranted,
    user::{
        kernel::data::{AuthUser, AuthUserId},
        login_id::kernel::data::LoginId,
    },
};

#[tokio::test]
async fn info() {
    let store = TestStore::new();
    let material = StaticUnregisterAuthUserAccountMaterial {
        authorize: StaticAuthorizeInfra::new(operator_user_id()),
        ticket_repository: standard_ticket_repository(&store),
        user_repository: standard_user_repository(&store),
    };

    let action = UnregisterAuthUserAccountAction::with_material(material);

    let (name, required) = action.info.params();
    assert_eq!(
        format!("{}; {}", name, required),
        "auth.user.account.unregister; require: some [auth-user]",
    );
}

#[tokio::test]
async fn success_unregister_user() {
    let holder = ApplicationActionStateHolder::new();

    let store = TestStore::new();
    let material = StaticUnregisterAuthUserAccountMaterial {
        authorize: StaticAuthorizeInfra::new(operator_user_id()),
        ticket_repository: standard_ticket_repository(&store),
        user_repository: standard_user_repository(&store),
    };
    let fields = StaticUnregisterAuthUserAccountFields::Valid(UnregisterAuthUserAccountFields {
        login_id: stored_login_id(),
    });

    let mut action = UnregisterAuthUserAccountAction::with_material(material);
    action.subscribe(holder.handler());

    let result = action.ignite(StaticAuthorizeToken, fields).await;
    assert_eq!(
        holder.extract(),
        vec![
            "try to proxy call: auth.ticket.authorize.clarify(require: some [auth-user])",
            "proxy call success",
            "unregister auth user account success",
        ],
    );
    assert!(result.is_ok());
}

struct TestStore {
    ticket: MemoryAuthTicketStore,
    user: MemoryAuthUserStore,
}

impl TestStore {
    fn new() -> Self {
        Self {
            ticket: MemoryAuthTicketStore::new(),
            user: MemoryAuthUserStore::new(),
        }
    }
}

fn standard_ticket_repository<'a>(store: &'a TestStore) -> MemoryAuthTicketRepository<'a> {
    MemoryAuthTicketRepository::new(&store.ticket)
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
    LoginId::restore("login-id".into())
}
fn stored_password() -> HashedPassword {
    HashedPassword::restore("password".into())
}

fn operator_user_id() -> AuthUserId {
    AuthUserId::restore("operator-user-id".to_owned())
}

use getto_application_test::ApplicationActionStateHolder;
use pretty_assertions::assert_eq;

use crate::auth::{
    ticket::{
        authorize::init::test::StaticAuthorizeInfra,
        kernel::init::request::test::StaticAuthorizeToken,
    },
    user::{
        account::search::init::test::{
            StaticSearchAuthUserAccountFilter, StaticSearchAuthUserAccountMaterial,
        },
        kernel::init::user_repository::memory::{MemoryAuthUserRepository, MemoryAuthUserStore},
    },
};

use crate::auth::user::account::search::action::SearchAuthUserAccountAction;

use crate::auth::user::password::kernel::infra::HashedPassword;

use crate::auth::{
    ticket::kernel::data::AuthPermissionGranted,
    user::{
        account::search::data::{SearchAuthUserAccountFilter, SearchAuthUserAccountFilterProps},
        kernel::data::{AuthUser, AuthUserId},
        login_id::kernel::data::{LoginId, SearchLoginId},
    },
};

#[tokio::test]
async fn info() {
    let store = TestStore::new();
    let material = StaticSearchAuthUserAccountMaterial {
        authorize: StaticAuthorizeInfra::new(operator_user_id()),
        user_repository: standard_user_repository(&store),
    };

    let action = SearchAuthUserAccountAction::with_material(material);

    let (name, required) = action.info.params();
    assert_eq!(
        format!("{}; {}", name, required),
        "auth.user.account.search; require: some [auth-user]",
    );
}

#[tokio::test]
async fn success_search() {
    let holder = ApplicationActionStateHolder::new();

    let store = TestStore::new();
    let material = StaticSearchAuthUserAccountMaterial {
        authorize: StaticAuthorizeInfra::new(operator_user_id()),
        user_repository: standard_user_repository(&store),
    };
    let filter = StaticSearchAuthUserAccountFilter::Valid(SearchAuthUserAccountFilter {
        offset: Default::default(),
        sort: Default::default(),
        props: SearchAuthUserAccountFilterProps {
            login_id: SearchLoginId::restore(None),
            granted: vec![],
        },
    });

    let mut action = SearchAuthUserAccountAction::with_material(material);
    action.subscribe(holder.handler());

    let result = action.ignite(StaticAuthorizeToken, filter).await;
    assert_eq!(
        holder.extract(),
        vec![
            "try to proxy call: auth.ticket.authorize.clarify(require: some [auth-user])",
            "proxy call success",
            "search user account success",
        ],
    );
    assert!(result.is_ok());
}

struct TestStore {
    search: MemoryAuthUserStore,
}

impl TestStore {
    fn new() -> Self {
        Self {
            search: MemoryAuthUserStore::new(),
        }
    }
}

fn standard_user_repository<'a>(store: &'a TestStore) -> MemoryAuthUserRepository<'a> {
    MemoryAuthUserRepository::with_user_and_password(
        &store.search,
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

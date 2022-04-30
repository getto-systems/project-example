use std::collections::HashSet;

use chrono::{DateTime, Duration, TimeZone, Utc};
use getto_application_test::ActionTestRunner;

use crate::auth::{
    ticket::{
        kernel::init::clock::test::StaticChronoAuthClock,
        validate::init::{
            nonce_metadata::test::StaticAuthNonceMetadata,
            nonce_repository::memory::{MemoryAuthNonceRepository, MemoryAuthNonceStore},
            test::{StaticAuthenticateStruct, StaticValidateAuthNonceStruct},
            token_decoder::test::StaticAuthTokenDecoder,
            token_metadata::test::StaticAuthTokenMetadata,
        },
    },
    user::{
        account::modify::{
            infra::{ModifyAuthUserAccountChangesExtract, ModifyAuthUserAccountFieldsExtract},
            init::request_decoder::test::StaticModifyAuthUserAccountRequestDecoder,
        },
        kernel::init::user_repository::memory::{MemoryAuthUserRepository, MemoryAuthUserStore},
    },
};

use crate::auth::user::account::modify::action::{
    ModifyAuthUserAccountAction, ModifyAuthUserAccountMaterial,
};

use crate::auth::ticket::validate::method::AuthNonceConfig;

use crate::auth::user::password::kernel::infra::HashedPassword;

use crate::auth::{
    ticket::kernel::data::{AuthTicketExtract, ExpireDuration},
    user::{
        account::kernel::data::AuthUserAttributesExtract,
        kernel::data::{AuthUser, AuthUserExtract, AuthUserId},
        login_id::kernel::data::LoginId,
    },
};

#[tokio::test]
async fn success_modify_user() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::new();
    let material = TestStruct::standard(&store);
    let request_decoder = standard_request_decoder();

    let mut action = ModifyAuthUserAccountAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "authenticate success; ticket: ticket-id / user: user-id (granted: [user])",
        "modify auth user account success",
    ]);
    assert!(result.is_err());
}

#[tokio::test]
async fn permission_denied() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::new();
    let material = TestStruct::not_permitted(&store);
    let request_decoder = standard_request_decoder();

    let mut action = ModifyAuthUserAccountAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "authenticate success; ticket: ticket-id / user: user-id (granted: [])",
        "user permission denied; granted: [], require: any [user]",
    ]);
    assert!(result.is_err());
}

#[tokio::test]
async fn error_conflict_changes() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::new();
    let material = TestStruct::standard(&store);
    let request_decoder = conflict_request_decoder();

    let mut action = ModifyAuthUserAccountAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "authenticate success; ticket: ticket-id / user: user-id (granted: [user])",
        "modify auth user account error; changes conflicted",
    ]);
    assert!(result.is_err());
}

#[tokio::test]
async fn error_not_found() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::new();
    let material = TestStruct::standard(&store);
    let request_decoder = not_found_request_decoder();

    let mut action = ModifyAuthUserAccountAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "authenticate success; ticket: ticket-id / user: user-id (granted: [user])",
        "modify auth user account error; not found",
    ]);
    assert!(result.is_err());
}

#[tokio::test]
async fn error_invalid_granted_roles() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::new();
    let material = TestStruct::standard(&store);
    let request_decoder = invalid_granted_roles_request_decoder();

    let mut action = ModifyAuthUserAccountAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "authenticate success; ticket: ticket-id / user: user-id (granted: [user])",
        "modify auth user account error; invalid to; invalid role",
    ]);
    assert!(result.is_err());
}

#[tokio::test]
async fn error_invalid_memo() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::new();
    let material = TestStruct::standard(&store);
    let request_decoder = invalid_memo_request_decoder();

    let mut action = ModifyAuthUserAccountAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "authenticate success; ticket: ticket-id / user: user-id (granted: [user])",
        "modify auth user account error; invalid to; memo: too long",
    ]);
    assert!(result.is_err());
}

struct TestStruct<'a> {
    validate: StaticAuthenticateStruct<'a>,
    user_repository: MemoryAuthUserRepository<'a>,
}

impl<'a> ModifyAuthUserAccountMaterial for TestStruct<'a> {
    type Authenticate = StaticAuthenticateStruct<'a>;

    type UserRepository = MemoryAuthUserRepository<'a>;

    fn authenticate(&self) -> &Self::Authenticate {
        &self.validate
    }
    fn user_repository(&self) -> &Self::UserRepository {
        &self.user_repository
    }
}

struct TestStore {
    nonce: MemoryAuthNonceStore,
    user: MemoryAuthUserStore,
}

impl TestStore {
    fn new() -> Self {
        Self {
            nonce: MemoryAuthNonceStore::new(),
            user: MemoryAuthUserStore::new(),
        }
    }
}

impl<'a> TestStruct<'a> {
    fn standard(store: &'a TestStore) -> Self {
        Self::new(store, standard_token_decoder())
    }
    fn not_permitted(store: &'a TestStore) -> Self {
        Self::new(store, not_permitted_token_decoder())
    }
    fn new(store: &'a TestStore, token_decoder: StaticAuthTokenDecoder) -> Self {
        Self {
            validate: StaticAuthenticateStruct {
                validate_nonce: StaticValidateAuthNonceStruct {
                    config: standard_nonce_config(),
                    clock: standard_clock(),
                    nonce_metadata: standard_nonce_header(),
                    nonce_repository: MemoryAuthNonceRepository::new(&store.nonce),
                },
                token_metadata: standard_token_header(),
                token_decoder,
            },
            user_repository: standard_user_repository(&store.user),
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
        granted_roles: standard_granted_roles().into_iter().collect(),
    })
}
fn not_permitted_token_decoder() -> StaticAuthTokenDecoder {
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
const REGISTERED_USER_ID: &'static str = "registered-user-id";
const REGISTERED_LOGIN_ID: &'static str = "registered-login-id";
const PASSWORD: &'static str = "current-password";

fn standard_request_decoder() -> StaticModifyAuthUserAccountRequestDecoder {
    StaticModifyAuthUserAccountRequestDecoder::Valid(ModifyAuthUserAccountFieldsExtract {
        login_id: LOGIN_ID.into(),
        from: Some(ModifyAuthUserAccountChangesExtract {
            granted_roles: standard_granted_roles(),
            attrs: AuthUserAttributesExtract::default(),
        }),
        to: Some(ModifyAuthUserAccountChangesExtract {
            granted_roles: vec![],
            attrs: AuthUserAttributesExtract {
                memo: "memo".into(),
            },
        }),
    })
}
fn conflict_request_decoder() -> StaticModifyAuthUserAccountRequestDecoder {
    StaticModifyAuthUserAccountRequestDecoder::Valid(ModifyAuthUserAccountFieldsExtract {
        login_id: LOGIN_ID.into(),
        from: Some(ModifyAuthUserAccountChangesExtract {
            granted_roles: vec![],
            attrs: AuthUserAttributesExtract {
                memo: "memo".into(),
            },
        }),
        to: Some(ModifyAuthUserAccountChangesExtract {
            granted_roles: vec![],
            attrs: AuthUserAttributesExtract {
                memo: "memo".into(),
            },
        }),
    })
}
fn not_found_request_decoder() -> StaticModifyAuthUserAccountRequestDecoder {
    StaticModifyAuthUserAccountRequestDecoder::Valid(ModifyAuthUserAccountFieldsExtract {
        login_id: "unknown-user".into(),
        from: Some(ModifyAuthUserAccountChangesExtract {
            granted_roles: vec![],
            attrs: AuthUserAttributesExtract::default(),
        }),
        to: Some(ModifyAuthUserAccountChangesExtract {
            granted_roles: vec![],
            attrs: AuthUserAttributesExtract::default(),
        }),
    })
}
fn invalid_granted_roles_request_decoder() -> StaticModifyAuthUserAccountRequestDecoder {
    StaticModifyAuthUserAccountRequestDecoder::Valid(ModifyAuthUserAccountFieldsExtract {
        login_id: LOGIN_ID.into(),
        from: Some(ModifyAuthUserAccountChangesExtract {
            granted_roles: standard_granted_roles(),
            attrs: AuthUserAttributesExtract::default(),
        }),
        to: Some(ModifyAuthUserAccountChangesExtract {
            granted_roles: vec!["invalid-role".into()],
            attrs: AuthUserAttributesExtract {
                memo: "memo".into(),
            },
        }),
    })
}
fn invalid_memo_request_decoder() -> StaticModifyAuthUserAccountRequestDecoder {
    StaticModifyAuthUserAccountRequestDecoder::Valid(ModifyAuthUserAccountFieldsExtract {
        login_id: LOGIN_ID.into(),
        from: Some(ModifyAuthUserAccountChangesExtract {
            granted_roles: standard_granted_roles(),
            attrs: AuthUserAttributesExtract::default(),
        }),
        to: Some(ModifyAuthUserAccountChangesExtract {
            granted_roles: vec![],
            attrs: AuthUserAttributesExtract {
                memo: "a".repeat(255 + 1),
            },
        }),
    })
}

fn standard_user_repository<'a>(store: &'a MemoryAuthUserStore) -> MemoryAuthUserRepository<'a> {
    MemoryAuthUserRepository::with_user_and_password(
        store,
        standard_login_id(),
        standard_user(),
        standard_password(),
        vec![(registered_login_id(), registered_user_id())],
    )
}

fn standard_user() -> AuthUser {
    AuthUserExtract {
        user_id: USER_ID.into(),
        granted_roles: standard_granted_roles().into_iter().collect(),
    }
    .restore()
}
fn standard_granted_roles() -> Vec<String> {
    vec!["user".into()]
}
fn standard_login_id() -> LoginId {
    LoginId::restore(LOGIN_ID.into())
}
fn standard_password() -> HashedPassword {
    HashedPassword::restore(PASSWORD.into())
}

fn registered_user_id() -> AuthUserId {
    AuthUserId::restore(REGISTERED_USER_ID.into())
}
fn registered_login_id() -> LoginId {
    LoginId::restore(REGISTERED_LOGIN_ID.into())
}

use std::collections::HashSet;

use chrono::{DateTime, Duration, TimeZone, Utc};
use getto_application_test::ActionTestRunner;

use crate::auth::{
    ticket::{
        kernel::init::{
            clock::test::StaticChronoAuthClock,
            ticket_repository::memory::{MemoryAuthTicketRepository, MemoryAuthTicketStore},
        },
        validate::init::{
            nonce_metadata::test::StaticAuthNonceMetadata,
            nonce_repository::memory::{MemoryAuthNonceRepository, MemoryAuthNonceStore},
            test::{StaticValidateAuthNonceStruct, StaticValidateAuthTokenStruct},
            token_decoder::test::StaticAuthTokenDecoder,
            token_metadata::test::StaticAuthTokenMetadata,
        },
    },
    user::{
        account::unregister::init::request_decoder::test::StaticUnregisterAuthUserAccountRequestDecoder,
        kernel::init::user_repository::memory::{MemoryAuthUserRepository, MemoryAuthUserStore},
    },
};

use crate::auth::user::account::unregister::action::{
    UnregisterAuthUserAccountAction, UnregisterAuthUserAccountMaterial,
};

use crate::auth::ticket::validate::method::AuthNonceConfig;

use crate::auth::user::{
    account::unregister::infra::UnregisterAuthUserAccountFieldsExtract,
    password::kernel::infra::HashedPassword,
};

use crate::auth::{
    ticket::kernel::data::{AuthTicketExtract, ExpireDuration},
    user::{
        kernel::data::{AuthUser, AuthUserExtract, AuthUserId},
        login_id::kernel::data::LoginId,
    },
};

#[tokio::test]
async fn success_unregister_user() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::new();
    let material = TestStruct::standard(&store);
    let request_decoder = standard_request_decoder();

    let mut action = UnregisterAuthUserAccountAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "validate success; ticket: ticket-id / user: user-id (granted: [user])",
        "unregister auth user account success",
    ]);
    assert!(result.is_ok());
}

#[tokio::test]
async fn permission_denied() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::new();
    let material = TestStruct::not_permitted(&store);
    let request_decoder = standard_request_decoder();

    let mut action = UnregisterAuthUserAccountAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "validate success; ticket: ticket-id / user: user-id (granted: [])",
        "user permission denied; granted: [], require: any [user]",
    ]);
    assert!(result.is_err());
}

struct TestStruct<'a> {
    validate: StaticValidateAuthTokenStruct<'a>,
    ticket_repository: MemoryAuthTicketRepository<'a>,
    user_repository: MemoryAuthUserRepository<'a>,
}

impl<'a> UnregisterAuthUserAccountMaterial for TestStruct<'a> {
    type Validate = StaticValidateAuthTokenStruct<'a>;

    type TicketRepository = MemoryAuthTicketRepository<'a>;
    type UserRepository = MemoryAuthUserRepository<'a>;

    fn validate(&self) -> &Self::Validate {
        &self.validate
    }

    fn ticket_repository(&self) -> &Self::TicketRepository {
        &self.ticket_repository
    }
    fn user_repository(&self) -> &Self::UserRepository {
        &self.user_repository
    }
}

struct TestStore {
    nonce: MemoryAuthNonceStore,
    ticket: MemoryAuthTicketStore,
    user: MemoryAuthUserStore,
}

impl TestStore {
    fn new() -> Self {
        Self {
            nonce: MemoryAuthNonceStore::new(),
            ticket: MemoryAuthTicketStore::new(),
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
            validate: StaticValidateAuthTokenStruct {
                validate_nonce: StaticValidateAuthNonceStruct {
                    config: standard_nonce_config(),
                    clock: standard_clock(),
                    nonce_metadata: standard_nonce_header(),
                    nonce_repository: MemoryAuthNonceRepository::new(&store.nonce),
                },
                token_metadata: standard_token_header(),
                token_decoder,
            },
            ticket_repository: MemoryAuthTicketRepository::new(&store.ticket),
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
        granted_roles: standard_granted_roles(),
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

fn standard_request_decoder() -> StaticUnregisterAuthUserAccountRequestDecoder {
    StaticUnregisterAuthUserAccountRequestDecoder::Valid(UnregisterAuthUserAccountFieldsExtract {
        login_id: LOGIN_ID.into(),
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
        granted_roles: standard_granted_roles(),
    }
    .restore()
}
fn standard_granted_roles() -> HashSet<String> {
    vec!["user".into()].into_iter().collect()
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

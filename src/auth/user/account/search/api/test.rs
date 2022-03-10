use std::collections::HashSet;

use chrono::{DateTime, Duration, TimeZone, Utc};

use getto_application_test::ActionTestRunner;

use crate::auth::{
    ticket::{
        kernel::init::clock::test::StaticChronoAuthClock,
        validate::init::{
            nonce_metadata::test::StaticAuthNonceMetadata,
            nonce_repository::memory::{
                MemoryAuthNonceMap, MemoryAuthNonceRepository, MemoryAuthNonceStore,
            },
            test::{StaticValidateAuthNonceStruct, StaticValidateAuthTokenStruct},
            token_decoder::test::StaticAuthTokenDecoder,
            token_metadata::test::StaticAuthTokenMetadata,
        },
    },
    user::{
        account::search::init::request_decoder::test::StaticSearchAuthUserAccountRequestDecoder,
        kernel::init::user_repository::memory::{
            MemoryAuthUserMap, MemoryAuthUserRepository, MemoryAuthUserStore,
        },
    },
};

use crate::auth::ticket::validate::method::AuthNonceConfig;

use super::action::{SearchAuthUserAccountAction, SearchAuthUserAccountMaterial};

use crate::auth::user::password::kernel::infra::HashedPassword;

use crate::{
    auth::{
        ticket::kernel::data::{AuthTicketExtract, ExpireDuration},
        user::{
            account::search::data::SearchAuthUserAccountFilterExtract,
            kernel::data::{AuthUser, AuthUserExtract},
            login_id::kernel::data::LoginId,
        },
    },
    z_lib::search::data::SearchSortExtract,
};

#[tokio::test]
async fn success_search() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let material = TestStruct::new(&store);
    let request_decoder = standard_request_decoder();

    let mut action = SearchAuthUserAccountAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "nonce expires calculated; 2021-01-02 10:00:00 UTC",
        "validate nonce success",
        "validate success; ticket: ticket-id / user: user-id (granted: [manage_auth_user])",
        "search user account success",
    ]);
    assert!(result.is_ok());
}

struct TestStruct<'a> {
    validate: StaticValidateAuthTokenStruct<'a>,
    search_repository: MemoryAuthUserRepository<'a>,
}

impl<'a> SearchAuthUserAccountMaterial for TestStruct<'a> {
    type Validate = StaticValidateAuthTokenStruct<'a>;
    type SearchRepository = MemoryAuthUserRepository<'a>;

    fn validate(&self) -> &Self::Validate {
        &self.validate
    }
    fn search_repository(&self) -> &Self::SearchRepository {
        &self.search_repository
    }
}

struct TestStore {
    nonce: MemoryAuthNonceStore,
    search: MemoryAuthUserStore,
}

impl TestStore {
    fn standard() -> Self {
        Self {
            nonce: standard_nonce_store(),
            search: standard_search_store(),
        }
    }
}

impl<'a> TestStruct<'a> {
    fn new(store: &'a TestStore) -> Self {
        Self {
            validate: StaticValidateAuthTokenStruct {
                validate_nonce: StaticValidateAuthNonceStruct {
                    config: standard_nonce_config(),
                    clock: standard_clock(),
                    nonce_metadata: standard_nonce_header(),
                    nonce_repository: MemoryAuthNonceRepository::new(&store.nonce),
                },
                token_metadata: standard_token_header(),
                token_decoder: standard_token_decoder(),
            },
            search_repository: MemoryAuthUserRepository::new(&store.search),
        }
    }
}

const NONCE: &'static str = "nonce";
const TICKET_ID: &'static str = "ticket-id";
const LOGIN_ID: &'static str = "login-id";
const USER_ID: &'static str = "user-id";
const PASSWORD: &'static str = "password";

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
    let mut granted_roles = HashSet::new();
    granted_roles.insert("manage_auth_user".into());

    StaticAuthTokenDecoder::Valid(AuthTicketExtract {
        ticket_id: TICKET_ID.into(),
        user_id: USER_ID.into(),
        granted_roles,
    })
}

fn standard_request_decoder() -> StaticSearchAuthUserAccountRequestDecoder {
    StaticSearchAuthUserAccountRequestDecoder::Valid(SearchAuthUserAccountFilterExtract {
        offset: 0,
        sort: SearchSortExtract {
            key: "login-id".to_owned().into(),
            order: "normal".into(),
        },
        login_id: Some("login-id".into()),
    })
}

fn standard_nonce_store() -> MemoryAuthNonceStore {
    MemoryAuthNonceMap::new().to_store()
}

fn standard_search_store() -> MemoryAuthUserStore {
    MemoryAuthUserMap::with_user_and_password(
        test_user_login_id(),
        test_user(),
        test_user_password(),
        vec![],
    )
    .to_store()
}

fn test_user() -> AuthUser {
    let mut granted_roles = HashSet::new();
    granted_roles.insert("manage_auth_user".into());

    AuthUserExtract {
        user_id: "test-user-id".into(),
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

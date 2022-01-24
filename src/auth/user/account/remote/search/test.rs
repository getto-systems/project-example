use std::collections::HashSet;

use chrono::{DateTime, Duration, TimeZone, Utc};

use getto_application_test::ActionTestRunner;

use crate::{
    auth::{
        ticket::remote::{
            check_nonce::init::{
                nonce_repository::test::{
                    MemoryAuthNonceMap, MemoryAuthNonceRepository, MemoryAuthNonceStore,
                },
                test::StaticCheckAuthNonceStruct,
            },
            kernel::init::{
                clock::test::StaticChronoAuthClock, nonce_metadata::test::StaticAuthNonceMetadata,
                token_decoder::test::StaticAuthTokenDecoder,
                token_metadata::test::StaticAuthTokenMetadata,
            },
            validate::init::test::StaticValidateAuthTokenStruct,
        },
        user::{
            account::remote::search::init::{
                request_decoder::test::StaticSearchAuthUserAccountRequestDecoder,
                search_repository::test::{
                    MemorySearchAuthUserAccountMap, MemorySearchAuthUserAccountRepository,
                    MemorySearchAuthUserAccountStore,
                },
            },
            password::remote::kernel::init::password_repository::test::MemoryAuthUserPasswordMap,
            remote::kernel::init::user_repository::test::MemoryAuthUserMap,
        },
    },
    z_lib::remote::search::data::SearchSortExtract,
};

use crate::auth::{
    ticket::remote::check_nonce::infra::AuthNonceConfig,
    user::{
        account::remote::search::infra::SearchAuthUserAccountFieldsExtract,
        password::remote::kernel::infra::HashedPassword,
    },
};

use super::action::{SearchAuthUserAccountAction, SearchAuthUserAccountMaterial};

use crate::auth::{
    ticket::remote::kernel::data::{AuthTicketExtract, ExpireDuration},
    user::{
        login_id::remote::data::LoginId,
        remote::kernel::data::{AuthUser, AuthUserExtract},
    },
};

#[tokio::test]
async fn success_search() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let feature = TestStruct::new(&store);
    let request_decoder = standard_request_decoder();

    let mut action = SearchAuthUserAccountAction::with_material(request_decoder, feature);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "validate success; ticket: ticket-id / user: user-id (granted: [manage_auth_user])",
        "search user account success",
    ]);
    assert!(result.is_ok());
}

struct TestStruct<'a> {
    validate: StaticValidateAuthTokenStruct<'a>,

    search_repository: MemorySearchAuthUserAccountRepository<'a>,
}

impl<'a> SearchAuthUserAccountMaterial for TestStruct<'a> {
    type Validate = StaticValidateAuthTokenStruct<'a>;
    type SearchRepository = MemorySearchAuthUserAccountRepository<'a>;

    fn validate(&self) -> &Self::Validate {
        &self.validate
    }

    fn search_repository(&self) -> &Self::SearchRepository {
        &self.search_repository
    }
}

struct TestStore {
    nonce: MemoryAuthNonceStore,
    search: MemorySearchAuthUserAccountStore,
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
                check_nonce_infra: StaticCheckAuthNonceStruct {
                    config: standard_nonce_config(),
                    clock: standard_clock(),
                    nonce_metadata: standard_nonce_header(),
                    nonce_repository: MemoryAuthNonceRepository::new(&store.nonce),
                },
                token_metadata: standard_token_header(),
                token_decoder: standard_token_decoder(),
            },
            search_repository: MemorySearchAuthUserAccountRepository::new(&store.search),
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
    StaticSearchAuthUserAccountRequestDecoder::Valid(SearchAuthUserAccountFieldsExtract {
        offset: 0,
        sort: SearchSortExtract {
            key: "login-id".into(),
            order: "normal".into(),
        },
        login_id: "login-id".into(),
    })
}

fn standard_nonce_store() -> MemoryAuthNonceStore {
    MemoryAuthNonceMap::new().to_store()
}

fn standard_search_store() -> MemorySearchAuthUserAccountStore {
    MemorySearchAuthUserAccountMap {
        password: MemoryAuthUserPasswordMap::with_password(
            test_user_login_id(),
            test_user(),
            test_user_password(),
        ),
        user: MemoryAuthUserMap::new(),
    }
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

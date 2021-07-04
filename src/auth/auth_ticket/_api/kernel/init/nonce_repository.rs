use std::collections::HashMap;
use std::fmt::Display;

use rusoto_core::RusotoError;
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, PutItemError, PutItemInput};

use crate::auth::auth_ticket::_api::kernel::data::AuthDateTime;
use crate::auth::auth_ticket::_api::kernel::infra::{AuthNonceEntry, AuthNonceRepository};

use crate::z_details::_api::repository::data::{RegisterResult, RepositoryError};

pub struct DynamoDbAuthNonceRepository<'a> {
    client: &'a DynamoDbClient,
    table_name: &'a str,
}

impl<'a> DynamoDbAuthNonceRepository<'a> {
    pub const fn new(client: &'a DynamoDbClient, table_name: &'a str) -> Self {
        Self { client, table_name }
    }
}

const NONCE: &'static str = "nonce";
const EXPIRES: &'static str = "expires";
const REGISTERED_AT: &'static str = "registered_at";
const PUT_CONDITION_EXPRESSION: &'static str = "attribute_not_exists(nonce)";

#[async_trait::async_trait]
impl<'a> AuthNonceRepository for DynamoDbAuthNonceRepository<'a> {
    async fn put(
        &self,
        entry: AuthNonceEntry,
        registered_at: AuthDateTime,
    ) -> Result<RegisterResult<()>, RepositoryError> {
        let extract = entry.extract();

        let mut item = AttributeMap::new();
        item.insert_nonce(extract.nonce);
        item.insert_expires(extract.expires);
        item.insert_registered_at(registered_at);

        // 有効期限が切れた項目は dynamodb の TTL の設定によって削除される
        let input = PutItemInput {
            table_name: self.table_name.into(),
            condition_expression: Some(PUT_CONDITION_EXPRESSION.into()),
            item: item.extract(),
            ..Default::default()
        };

        match self.client.put_item(input).await {
            Ok(_) => Ok(RegisterResult::Success(())),
            Err(err) => match err {
                RusotoError::Service(err) => match err {
                    PutItemError::ConditionalCheckFailed(_) => Ok(RegisterResult::Conflict),
                    _ => Err(repository_error(err)),
                },
                _ => Err(repository_error(err)),
            },
        }
    }
}

fn repository_error(err: impl Display) -> RepositoryError {
    RepositoryError::InfraError(format!("{}", err))
}

struct AttributeMap(HashMap<String, AttributeValue>);

impl AttributeMap {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn extract(self) -> HashMap<String, AttributeValue> {
        self.0
    }

    fn insert_nonce(&mut self, nonce: String) -> &mut Self {
        self.0.insert(NONCE.into(), string_value(nonce));
        self
    }
    fn insert_expires(&mut self, expires: Option<i64>) -> &mut Self {
        if let Some(expires) = expires {
            self.0.insert(EXPIRES.into(), timestamp_value(expires));
        }
        self
    }
    fn insert_registered_at(&mut self, registered_at: AuthDateTime) -> &mut Self {
        self.0.insert(
            REGISTERED_AT.into(),
            timestamp_value(registered_at.timestamp()),
        );
        self
    }
}

fn string_value(value: String) -> AttributeValue {
    AttributeValue {
        s: Some(value),
        ..Default::default()
    }
}
fn timestamp_value(value: i64) -> AttributeValue {
    AttributeValue {
        n: Some(value.to_string()),
        ..Default::default()
    }
}

#[cfg(test)]
pub mod test {
    use std::{collections::HashMap, sync::Mutex};

    use crate::auth::auth_ticket::_api::kernel::infra::{
        AuthNonceEntry, AuthNonceEntryExtract, AuthNonceRepository,
    };

    use crate::auth::auth_ticket::_api::kernel::data::{
        AuthDateTime, AuthNonceValue, ExpireDateTime,
    };
    use crate::z_details::_api::repository::data::{RegisterResult, RepositoryError};

    pub type MemoryAuthNonceStore = Mutex<MemoryAuthNonceMap>;
    pub struct MemoryAuthNonceMap(HashMap<String, AuthNonceEntryExtract>);

    impl MemoryAuthNonceMap {
        pub fn new() -> Self {
            Self(HashMap::new())
        }

        pub fn with_nonce(nonce: String, expires: ExpireDateTime) -> Self {
            let mut hash_map = HashMap::new();
            hash_map.insert(
                nonce.clone(),
                AuthNonceEntryExtract {
                    nonce,
                    expires: Some(expires.timestamp()),
                },
            );
            Self(hash_map)
        }

        pub fn to_store(self) -> MemoryAuthNonceStore {
            Mutex::new(self)
        }

        fn get(&self, nonce: &AuthNonceValue) -> Option<&AuthNonceEntryExtract> {
            self.0.get(nonce.as_str())
        }
        fn insert(&mut self, entry: AuthNonceEntry) {
            let extract = entry.extract();
            self.0.insert(extract.nonce.clone(), extract);
        }
    }

    pub struct MemoryAuthNonceRepository<'a> {
        store: &'a MemoryAuthNonceStore,
    }

    impl<'a> MemoryAuthNonceRepository<'a> {
        pub const fn new(store: &'a MemoryAuthNonceStore) -> Self {
            Self { store }
        }
    }

    #[async_trait::async_trait]
    impl<'a> AuthNonceRepository for MemoryAuthNonceRepository<'a> {
        async fn put(
            &self,
            entry: AuthNonceEntry,
            registered_at: AuthDateTime,
        ) -> Result<RegisterResult<()>, RepositoryError> {
            let mut store = self.store.lock().unwrap();

            if let Some(found) = store.get(entry.nonce()) {
                if !has_expired(found.expires, &registered_at) {
                    return Ok(RegisterResult::Conflict);
                }
            }

            store.insert(entry);
            Ok(RegisterResult::Success(()))
        }
    }

    fn has_expired(expires: Option<i64>, now: &AuthDateTime) -> bool {
        match expires {
            None => false,
            Some(expires) => ExpireDateTime::restore(expires).has_elapsed(now),
        }
    }
}

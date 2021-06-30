use std::collections::HashMap;

use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, GetItemInput, PutItemInput};

use crate::auth::auth_ticket::_api::kernel::infra::{
    AuthNonceEntry, AuthNonceEntryExtract, AuthNonceRepository,
};

use crate::auth::auth_ticket::_api::kernel::data::AuthNonceValue;
use crate::z_details::_api::repository::data::RepositoryError;

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
const PUT_CONDITION_EXPRESSION: &'static str = "attribute_not_exists(nonce)";

#[async_trait::async_trait]
impl<'a> AuthNonceRepository for DynamoDbAuthNonceRepository<'a> {
    async fn get(&self, nonce: &AuthNonceValue) -> Result<Option<AuthNonceEntry>, RepositoryError> {
        let mut key: HashMap<String, AttributeValue> = HashMap::new();
        key.insert(
            NONCE.into(),
            AttributeValue {
                s: Some(nonce.as_str().into()),
                ..Default::default()
            },
        );

        let input = GetItemInput {
            table_name: self.table_name.into(),
            key,
            attributes_to_get: Some(vec![EXPIRES.into()]),
            ..Default::default()
        };

        let output = self
            .client
            .get_item(input)
            .await
            .map_err(|err| RepositoryError::InfraError(format!("{}", err)))?;

        Ok(output.item.map(|mut value| {
            AuthNonceEntryExtract {
                nonce: nonce.as_str().into(),
                expires: value
                    .remove(EXPIRES)
                    .and_then(|value| value.n.and_then(|value| value.parse::<i64>().ok())),
            }
            .into()
        }))
    }
    async fn put(&self, entry: AuthNonceEntry) -> Result<(), RepositoryError> {
        let extract = entry.extract();

        let mut item: HashMap<String, AttributeValue> = HashMap::new();
        item.insert(
            NONCE.into(),
            AttributeValue {
                s: Some(extract.nonce),
                ..Default::default()
            },
        );
        if let Some(expires) = extract.expires {
            item.insert(
                EXPIRES.into(),
                AttributeValue {
                    n: Some(expires.to_string()),
                    ..Default::default()
                },
            );
        }

        let input = PutItemInput {
            table_name: self.table_name.into(),
            condition_expression: Some(PUT_CONDITION_EXPRESSION.into()),
            item,
            ..Default::default()
        };

        self.client
            .put_item(input)
            .await
            .map_err(|err| RepositoryError::InfraError(format!("{}", err)))?;

        Ok(())
    }
}

#[cfg(test)]
pub mod test {
    use std::{collections::HashMap, sync::Mutex};

    use crate::auth::auth_ticket::_api::kernel::infra::{
        AuthNonceEntry, AuthNonceEntryExtract, AuthNonceRepository,
    };

    use crate::auth::auth_ticket::_api::kernel::data::{AuthNonceValue, ExpireDateTime};
    use crate::z_details::_api::repository::data::RepositoryError;

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
        async fn get(
            &self,
            nonce: &AuthNonceValue,
        ) -> Result<Option<AuthNonceEntry>, RepositoryError> {
            let store = self.store.lock().unwrap();
            Ok(store.get(nonce).map(|entry| entry.clone().into()))
        }
        async fn put(&self, entry: AuthNonceEntry) -> Result<(), RepositoryError> {
            let mut store = self.store.lock().unwrap();
            store.insert(entry);
            Ok(())
        }
    }
}

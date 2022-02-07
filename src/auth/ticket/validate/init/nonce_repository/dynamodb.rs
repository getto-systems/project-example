use std::collections::HashMap;

use chrono::{DateTime, Utc};
use rusoto_core::RusotoError;
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, PutItemError, PutItemInput};

use crate::z_lib::repository::helper::infra_error;

use crate::auth::ticket::validate::infra::{AuthNonceEntry, AuthNonceRepository};

use crate::{
    auth::ticket::kernel::data::AuthDateTime,
    z_lib::repository::data::{RegisterResult, RepositoryError},
};

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
        let mut item = AttributeMap::new();
        item.insert_entry(entry);
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
                    _ => Err(infra_error(err)),
                },
                _ => Err(infra_error(err)),
            },
        }
    }
}

struct AttributeMap(HashMap<String, AttributeValue>);

impl AttributeMap {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn extract(self) -> HashMap<String, AttributeValue> {
        self.0
    }

    fn insert_entry(&mut self, entry: AuthNonceEntry) -> &mut Self {
        let extract = entry.extract();

        self.0.insert(NONCE.into(), string_value(extract.nonce));

        if let Some(expires) = extract.expires {
            self.0.insert(EXPIRES.into(), timestamp_value(expires));
        }

        self
    }
    fn insert_registered_at(&mut self, registered_at: AuthDateTime) -> &mut Self {
        self.0.insert(
            REGISTERED_AT.into(),
            timestamp_value(registered_at.extract()),
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
fn timestamp_value(value: DateTime<Utc>) -> AttributeValue {
    AttributeValue {
        n: Some(value.timestamp().to_string()),
        ..Default::default()
    }
}

#[cfg(test)]
pub mod test {
}

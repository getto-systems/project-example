use std::collections::HashMap;

use rusoto_core::RusotoError;
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, PutItemError, PutItemInput};

use crate::z_lib::repository::dynamodb::helper::{string_value, timestamp_value};
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

#[async_trait::async_trait]
impl<'a> AuthNonceRepository for DynamoDbAuthNonceRepository<'a> {
    async fn put(
        &self,
        entry: AuthNonceEntry,
        registered_at: AuthDateTime,
    ) -> Result<RegisterResult<()>, RepositoryError> {
        put_nonce(self, entry, registered_at).await
    }
}
async fn put_nonce<'a>(
    repository: &DynamoDbAuthNonceRepository<'a>,
    entry: AuthNonceEntry,
    registered_at: AuthDateTime,
) -> Result<RegisterResult<()>, RepositoryError> {
    let mut item = AttributeMap::new();
    item.add_entry(entry);
    item.add_registered_at(registered_at);

    // 有効期限が切れた項目は dynamodb の TTL の設定によって削除される
    let input = PutItemInput {
        table_name: repository.table_name.into(),
        condition_expression: Some("attribute_not_exists(nonce)".into()),
        item: item.extract(),
        ..Default::default()
    };

    match repository.client.put_item(input).await {
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

struct AttributeMap(HashMap<String, AttributeValue>);

impl AttributeMap {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn extract(self) -> HashMap<String, AttributeValue> {
        self.0
    }

    fn add_entry(&mut self, entry: AuthNonceEntry) -> &mut Self {
        let extract = entry.extract();

        self.0.insert("nonce".into(), string_value(extract.nonce));
        self.0
            .insert("expires".into(), timestamp_value(extract.expires));

        self
    }
    fn add_registered_at(&mut self, registered_at: AuthDateTime) -> &mut Self {
        self.0.insert(
            "registered_at".into(),
            timestamp_value(registered_at.extract()),
        );
        self
    }
}

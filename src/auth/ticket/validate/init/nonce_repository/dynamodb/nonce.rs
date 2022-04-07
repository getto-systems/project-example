use chrono::{DateTime, NaiveDateTime, Utc};
use rusoto_core::RusotoError;
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, PutItemError, PutItemInput};

use crate::auth::x_outside_feature::feature::AuthOutsideStore;

use crate::z_lib::repository::{
    dynamodb::helper::{string_value, timestamp_value, DynamoDbColumn},
    helper::infra_error,
};

use crate::{
    auth::ticket::kernel::data::{AuthDateTime, AuthNonce, ExpireDateTime},
    z_lib::repository::data::{RegisterResult, RepositoryError},
};

pub struct TableNonce<'a> {
    client: &'a DynamoDbClient,
    table_name: &'a str,
}
impl<'a> TableNonce<'a> {
    pub const fn new(feature: &'a AuthOutsideStore) -> Self {
        Self {
            client: &feature.dynamodb,
            table_name: feature.login_id_table_name,
        }
    }

    pub async fn put_nonce(
        &self,
        nonce: AuthNonce,
        expires: ExpireDateTime,
        registered_at: AuthDateTime,
    ) -> Result<RegisterResult<()>, RepositoryError> {
        // 有効期限が切れた項目は dynamodb の TTL の設定によって削除される
        let input = PutItemInput {
            table_name: self.table_name.into(),
            condition_expression: Some(format!("attribute_not_exists({})", ColumnNonce::as_name())),
            item: vec![
                ColumnNonce::to_attr_pair(nonce),
                ColumnExpires::to_attr_pair(expires),
                ColumnRegisteredAt::to_attr_pair(registered_at),
            ]
            .into_iter()
            .collect(),
            ..Default::default()
        };

        match self.client.put_item(input).await {
            Ok(_) => Ok(RegisterResult::Success(())),
            Err(err) => match err {
                RusotoError::Service(PutItemError::ConditionalCheckFailed(_)) => {
                    Ok(RegisterResult::Conflict)
                }
                _ => Err(infra_error("put nonce error", err)),
            },
        }
    }
}

struct ColumnNonce;
impl DynamoDbColumn for ColumnNonce {
    type Value = AuthNonce;

    fn as_name() -> &'static str {
        "nonce"
    }
    fn to_attr(value: Self::Value) -> AttributeValue {
        string_value(value.extract())
    }
    fn to_value(attr: AttributeValue) -> Option<Self::Value> {
        attr.s.map(|value| Self::Value::restore(value))
    }
}

struct ColumnExpires;
impl DynamoDbColumn for ColumnExpires {
    type Value = ExpireDateTime;

    fn as_name() -> &'static str {
        "expires"
    }
    fn to_attr(value: Self::Value) -> AttributeValue {
        timestamp_value(value.extract())
    }
    fn to_value(attr: AttributeValue) -> Option<Self::Value> {
        attr.n
            .and_then(|value| value.parse::<i64>().ok())
            .map(|value| {
                Self::Value::restore(DateTime::<Utc>::from_utc(
                    NaiveDateTime::from_timestamp(value, 0),
                    Utc,
                ))
            })
    }
}

struct ColumnRegisteredAt;
impl DynamoDbColumn for ColumnRegisteredAt {
    type Value = AuthDateTime;

    fn as_name() -> &'static str {
        "registered_at"
    }
    fn to_attr(value: Self::Value) -> AttributeValue {
        timestamp_value(value.extract())
    }
    fn to_value(attr: AttributeValue) -> Option<Self::Value> {
        attr.n
            .and_then(|value| value.parse::<i64>().ok())
            .map(|value| {
                Self::Value::restore(DateTime::<Utc>::from_utc(
                    NaiveDateTime::from_timestamp(value, 0),
                    Utc,
                ))
            })
    }
}

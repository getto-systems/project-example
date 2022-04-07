use std::collections::HashMap;

use chrono::{DateTime, NaiveDateTime, Utc};
use rusoto_dynamodb::{
    AttributeValue, DeleteItemInput, DynamoDb, DynamoDbClient, GetItemInput, PutItemInput,
};

use crate::auth::x_outside_feature::feature::AuthOutsideStore;

use crate::z_lib::repository::{
    dynamodb::helper::{string_value, timestamp_value, DynamoDbColumn},
    helper::infra_error,
};

use crate::{
    auth::{
        ticket::kernel::data::{AuthDateTime, AuthTicket, AuthTicketId, ExpansionLimitDateTime},
        user::kernel::data::AuthUserId,
    },
    z_lib::repository::data::RepositoryError,
};

pub struct TableTicket<'a> {
    client: &'a DynamoDbClient,
    table_name: &'a str,
}
impl<'a> TableTicket<'a> {
    pub const fn new(feature: &'a AuthOutsideStore) -> Self {
        Self {
            client: &feature.dynamodb,
            table_name: feature.login_id_table_name,
        }
    }

    fn key(ticket: AuthTicket) -> HashMap<String, AttributeValue> {
        let (ticket_id, user) = ticket.extract();

        vec![
            ColumnTicketId::to_attr_pair(ticket_id),
            ColumnUserId::to_attr_pair(user.into_user_id()),
        ]
        .into_iter()
        .collect()
    }

    pub async fn get_expansion_limit(
        &self,
        ticket: AuthTicket,
    ) -> Result<Option<ExpansionLimitDateTime>, RepositoryError> {
        let input = GetItemInput {
            table_name: self.table_name.into(),
            key: Self::key(ticket),
            projection_expression: Some(
                vec![ColumnExpansionLimit::as_name()].into_iter().collect(),
            ),
            ..Default::default()
        };

        let response = self
            .client
            .get_item(input)
            .await
            .map_err(|err| infra_error("get expansion limit error", err))?;

        Ok(response
            .item
            .and_then(|mut attrs| ColumnExpansionLimit::remove_value(&mut attrs)))
    }

    pub async fn put_ticket(
        &self,
        ticket: AuthTicket,
        expansion_limit: ExpansionLimitDateTime,
        issued_at: AuthDateTime,
    ) -> Result<(), RepositoryError> {
        let (ticket_id, user) = ticket.extract();

        // 有効期限が切れた項目は dynamodb の TTL の設定によって削除される
        let input = PutItemInput {
            table_name: self.table_name.into(),
            condition_expression: Some(format!(
                "attribute_not_exists({})",
                ColumnTicketId::as_name()
            )),
            item: vec![
                ColumnTicketId::to_attr_pair(ticket_id),
                ColumnUserId::to_attr_pair(user.into_user_id()),
                ColumnExpansionLimit::to_attr_pair(expansion_limit),
                ColumnIssuedAt::to_attr_pair(issued_at),
            ]
            .into_iter()
            .collect(),
            ..Default::default()
        };

        self.client
            .put_item(input)
            .await
            .map_err(|err| infra_error("put ticket error", err))?;

        Ok(())
    }

    pub async fn delete_ticket(&self, ticket: AuthTicket) -> Result<(), RepositoryError> {
        let input = DeleteItemInput {
            table_name: self.table_name.into(),
            key: Self::key(ticket),
            ..Default::default()
        };

        self.client
            .delete_item(input)
            .await
            .map_err(|err| infra_error("delete ticket error", err))?;

        Ok(())
    }
}

struct ColumnTicketId;
impl DynamoDbColumn for ColumnTicketId {
    type Value = AuthTicketId;

    fn as_name() -> &'static str {
        "ticket_id"
    }
    fn to_attr(value: Self::Value) -> AttributeValue {
        string_value(value.extract())
    }
    fn to_value(attr: AttributeValue) -> Option<Self::Value> {
        attr.s.map(|value| Self::Value::restore(value))
    }
}

struct ColumnUserId;
impl DynamoDbColumn for ColumnUserId {
    type Value = AuthUserId;

    fn as_name() -> &'static str {
        "user_id"
    }
    fn to_attr(value: Self::Value) -> AttributeValue {
        string_value(value.extract())
    }
    fn to_value(attr: AttributeValue) -> Option<Self::Value> {
        attr.s.map(|value| Self::Value::restore(value))
    }
}

struct ColumnExpansionLimit;
impl DynamoDbColumn for ColumnExpansionLimit {
    type Value = ExpansionLimitDateTime;

    fn as_name() -> &'static str {
        "expansion_limit"
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

struct ColumnIssuedAt;
impl DynamoDbColumn for ColumnIssuedAt {
    type Value = AuthDateTime;

    fn as_name() -> &'static str {
        "issued_at"
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

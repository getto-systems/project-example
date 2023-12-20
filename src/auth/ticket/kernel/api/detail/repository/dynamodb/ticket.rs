use std::{collections::HashMap, sync::Arc};

use aws_sdk_dynamodb::{types::AttributeValue, Client};
use chrono::{NaiveDateTime, TimeZone, Utc};

use crate::common::api::{
    feature::AsInfra,
    repository::dynamodb::detail::{DynamoDbColumn, ScanKey},
};

use crate::x_outside_feature::auth::feature::AuthAppFeature;

use crate::{
    auth::{
        kernel::data::{AuthDateTime, ExpansionLimitDateTime},
        ticket::kernel::data::{AuthTicket, AuthTicketId},
        user::kernel::data::AuthUserId,
    },
    common::api::repository::data::RepositoryError,
};

pub struct ConnectionTicket {
    client: Arc<Client>,
    table_name: &'static str,
}

impl AsInfra<ConnectionTicket> for Arc<AuthAppFeature> {
    fn as_infra(&self) -> ConnectionTicket {
        ConnectionTicket {
            client: Arc::clone(&self.store.dynamodb),
            table_name: self.store.ticket_table_name,
        }
    }
}

pub struct TableTicket;

impl TableTicket {
    fn key(
        ticket_id: AuthTicketId,
        user_id: AuthUserId,
    ) -> Option<HashMap<String, AttributeValue>> {
        Some(
            vec![
                ColumnTicketId::into_attr(ticket_id),
                ColumnUserId::into_attr(user_id),
            ]
            .into_iter()
            .collect(),
        )
    }

    pub async fn get_expansion_limit(
        conn: &ConnectionTicket,
        ticket: AuthTicket,
    ) -> Result<Option<ExpansionLimitDateTime>, RepositoryError> {
        let request = conn
            .client
            .get_item()
            .table_name(conn.table_name)
            .set_key(Self::key(ticket.ticket_id, ticket.attrs.user_id))
            .projection_expression(vec![ColumnExpansionLimit::as_name()].join(","));

        let response = request
            .send()
            .await
            .map_err(|err| ("get expansion-limit error", err))?;

        Ok(response
            .item
            .and_then(|mut attrs| ColumnExpansionLimit::restore(&mut attrs)))
    }

    pub async fn put_ticket(
        conn: &ConnectionTicket,
        ticket: AuthTicket,
        expansion_limit: ExpansionLimitDateTime,
        issued_at: AuthDateTime,
    ) -> Result<(), RepositoryError> {
        // 有効期限が切れた項目は dynamodb の TTL の設定によって削除される
        let request = conn
            .client
            .put_item()
            .table_name(conn.table_name)
            .condition_expression(format!(
                "attribute_not_exists({})",
                ColumnTicketId::as_name()
            ))
            .set_item(Some(
                vec![
                    ColumnTicketId::into_attr(ticket.ticket_id),
                    ColumnUserId::into_attr(ticket.attrs.user_id),
                    ColumnExpansionLimit::into_attr(expansion_limit),
                    ColumnIssuedAt::into_attr(issued_at),
                ]
                .into_iter()
                .collect(),
            ));

        let _response = request
            .send()
            .await
            .map_err(|err| ("put ticket error", err))?;

        Ok(())
    }

    pub async fn delete_ticket(
        conn: &ConnectionTicket,
        ticket_id: AuthTicketId,
        user_id: AuthUserId,
    ) -> Result<(), RepositoryError> {
        let request = conn
            .client
            .delete_item()
            .table_name(conn.table_name)
            .set_key(Self::key(ticket_id, user_id));

        let _response = request
            .send()
            .await
            .map_err(|err| ("delete ticket error", err))?;

        Ok(())
    }

    pub async fn query_ticket_id(
        conn: &ConnectionTicket,
        user_id: AuthUserId,
    ) -> Result<Vec<AuthTicketId>, RepositoryError> {
        let mut acc = vec![];
        let mut scan_key = ScanKey::FirstTime;
        while scan_key.has_next() {
            let (mut items, key) =
                Self::query_ticket_id_part(conn, user_id.clone(), scan_key).await?;
            acc.append(&mut items);
            scan_key = key;
        }
        Ok(acc)
    }
    async fn query_ticket_id_part(
        conn: &ConnectionTicket,
        user_id: AuthUserId,
        scan_key: ScanKey,
    ) -> Result<(Vec<AuthTicketId>, ScanKey), RepositoryError> {
        let request = conn
            .client
            .query()
            .table_name(conn.table_name)
            .key_condition_expression(format!("{} = :user_id", ColumnUserId::as_name()))
            .set_expression_attribute_values(Some(
                vec![ColumnUserId::into_attr_with_name(":user_id", user_id)]
                    .into_iter()
                    .collect(),
            ))
            .set_exclusive_start_key(scan_key.extract())
            .projection_expression(vec![ColumnTicketId::as_name()].join(","));

        let response = request
            .send()
            .await
            .map_err(|err| ("scan user error", err))?;

        let items = match response.items {
            None => vec![],
            Some(items) => items
                .into_iter()
                .filter_map(|mut attrs| ColumnTicketId::restore(&mut attrs))
                .collect(),
        };

        Ok((items, ScanKey::next(response.last_evaluated_key)))
    }
}

struct ColumnTicketId;
impl DynamoDbColumn for ColumnTicketId {
    type Value = AuthTicketId;

    fn as_name() -> &'static str {
        "ticket_id"
    }
    fn into(value: Self::Value) -> AttributeValue {
        AttributeValue::S(value.extract())
    }
    fn from(attr: AttributeValue) -> Option<Self::Value> {
        if let AttributeValue::S(value) = attr {
            Some(AuthTicketId::restore(value))
        } else {
            None
        }
    }
}

struct ColumnUserId;
impl DynamoDbColumn for ColumnUserId {
    type Value = AuthUserId;

    fn as_name() -> &'static str {
        "user_id"
    }
    fn into(value: Self::Value) -> AttributeValue {
        AttributeValue::S(value.extract())
    }
    fn from(attr: AttributeValue) -> Option<Self::Value> {
        if let AttributeValue::S(value) = attr {
            Some(AuthUserId::restore(value))
        } else {
            None
        }
    }
}

struct ColumnExpansionLimit;
impl DynamoDbColumn for ColumnExpansionLimit {
    type Value = ExpansionLimitDateTime;

    fn as_name() -> &'static str {
        "expansion_limit"
    }
    fn into(value: Self::Value) -> AttributeValue {
        AttributeValue::N(value.extract().timestamp().to_string())
    }
    fn from(attr: AttributeValue) -> Option<Self::Value> {
        if let AttributeValue::N(value) = attr {
            value.parse::<i64>().ok().map({
                |value| {
                    ExpansionLimitDateTime::restore(Utc.from_utc_datetime(
                        &NaiveDateTime::from_timestamp_opt(value, 0).unwrap_or_default(),
                    ))
                }
            })
        } else {
            None
        }
    }
}

struct ColumnIssuedAt;
impl DynamoDbColumn for ColumnIssuedAt {
    type Value = AuthDateTime;

    fn as_name() -> &'static str {
        "issued_at"
    }
    fn into(value: Self::Value) -> AttributeValue {
        AttributeValue::N(value.extract().timestamp().to_string())
    }
    fn from(attr: AttributeValue) -> Option<Self::Value> {
        if let AttributeValue::N(value) = attr {
            value.parse::<i64>().ok().map({
                |value| {
                    AuthDateTime::restore(Utc.from_utc_datetime(
                        &NaiveDateTime::from_timestamp_opt(value, 0).unwrap_or_default(),
                    ))
                }
            })
        } else {
            None
        }
    }
}

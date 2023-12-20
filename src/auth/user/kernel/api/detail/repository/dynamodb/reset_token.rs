use std::{collections::HashMap, sync::Arc};

use aws_sdk_dynamodb::{types::AttributeValue, Client};
use chrono::{NaiveDateTime, TimeZone, Utc};

use crate::x_outside_feature::auth::feature::AuthAppFeature;

use crate::common::api::{feature::AsInfra, repository::dynamodb::detail::DynamoDbColumn};

use crate::auth::user::password::reset::reset::infra::ResetPasswordTokenMoment;

use crate::{
    auth::{
        kernel::data::{AuthDateTime, ExpireDateTime},
        user::{
            kernel::data::AuthUserId,
            password::reset::kernel::data::{
                ResetPasswordId, ResetPasswordTokenDestination, ResetPasswordTokenDestinationEmail,
            },
        },
    },
    common::api::repository::data::RepositoryError,
};

pub struct ConnectionResetToken {
    client: Arc<Client>,
    table_name: &'static str,
}

impl AsInfra<ConnectionResetToken> for Arc<AuthAppFeature> {
    fn as_infra(&self) -> ConnectionResetToken {
        ConnectionResetToken {
            client: Arc::clone(&self.store.dynamodb),
            table_name: self.store.reset_token_table_name,
        }
    }
}

pub struct TableResetToken;

impl TableResetToken {
    fn key(reset_token: ResetPasswordId) -> Option<HashMap<String, AttributeValue>> {
        Some(
            vec![ColumnResetToken::into_attr(reset_token)]
                .into_iter()
                .collect(),
        )
    }

    pub async fn get_reset_token_entry(
        conn: &ConnectionResetToken,
        reset_token: ResetPasswordId,
    ) -> Result<
        Option<(
            AuthUserId,
            ResetPasswordTokenDestination,
            ResetPasswordTokenMoment,
        )>,
        RepositoryError,
    > {
        let request = conn
            .client
            .get_item()
            .table_name(conn.table_name)
            .set_key(Self::key(reset_token))
            .projection_expression(
                vec![
                    ColumnUserId::as_name(),
                    ColumnEmail::as_name(),
                    ColumnExpires::as_name(),
                    ColumnResetAt::as_name(),
                ]
                .join(","),
            );

        let response = request
            .send()
            .await
            .map_err(|err| ("get reset-token error", err))?;

        Ok(response.item.and_then(|mut attrs| {
            match (
                ColumnUserId::restore(&mut attrs),
                ColumnEmail::restore(&mut attrs),
                ColumnExpires::restore(&mut attrs),
                ColumnResetAt::restore(&mut attrs),
            ) {
                (Some(user_id), Some(email), Some(expires), reset_at) => Some((
                    user_id,
                    email,
                    ResetPasswordTokenMoment::restore(expires, reset_at),
                )),
                _ => None,
            }
        }))
    }

    pub async fn put_reset_token(
        conn: &ConnectionResetToken,
        reset_token: ResetPasswordId,
        user_id: AuthUserId,
        destination: ResetPasswordTokenDestination,
        expires: ExpireDateTime,
        requested_at: AuthDateTime,
    ) -> Result<(), RepositoryError> {
        // reset token がすでに登録されていたらエラーになる
        // reset token は削除しないので、衝突が発生したら reset token の桁数を増やす
        let request = conn
            .client
            .put_item()
            .table_name(conn.table_name)
            .set_item(Some(
                vec![
                    ColumnResetToken::into_attr(reset_token),
                    ColumnUserId::into_attr(user_id),
                    ColumnEmail::into_attr(destination),
                    ColumnExpires::into_attr(expires),
                    ColumnRequestedAt::into_attr(requested_at),
                ]
                .into_iter()
                .collect(),
            ))
            .condition_expression(format!(
                "attribute_not_exists({})",
                ColumnResetToken::as_name()
            ));

        let _response = request
            .send()
            .await
            .map_err(|err| ("put reset-token error", err))?;

        Ok(())
    }
    pub async fn update_reset_at(
        conn: &ConnectionResetToken,
        reset_token: ResetPasswordId,
        reset_at: AuthDateTime,
    ) -> Result<(), RepositoryError> {
        let request = conn
            .client
            .update_item()
            .table_name(conn.table_name)
            .set_key(Self::key(reset_token))
            .update_expression(format!("set {} = :reset_at", ColumnResetAt::as_name()))
            .set_expression_attribute_values(Some(
                vec![ColumnResetAt::into_attr_with_name(":reset_at", reset_at)]
                    .into_iter()
                    .collect(),
            ));

        let _response = request
            .send()
            .await
            .map_err(|err| ("update reset-at error", err))?;

        Ok(())
    }
}

struct ColumnResetToken;
impl DynamoDbColumn for ColumnResetToken {
    type Value = ResetPasswordId;

    fn as_name() -> &'static str {
        "reset_token"
    }
    fn into(value: Self::Value) -> AttributeValue {
        AttributeValue::S(value.extract())
    }
    fn from(attr: AttributeValue) -> Option<Self::Value> {
        if let AttributeValue::S(value) = attr {
            Some(ResetPasswordId::restore(value))
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

struct ColumnEmail;
impl DynamoDbColumn for ColumnEmail {
    type Value = ResetPasswordTokenDestination;

    fn as_name() -> &'static str {
        "email"
    }
    fn into(value: Self::Value) -> AttributeValue {
        match value {
            Self::Value::None => AttributeValue::Null(true),
            Self::Value::Email(email) => AttributeValue::S(email.extract()),
        }
    }
    fn from(attr: AttributeValue) -> Option<Self::Value> {
        if let AttributeValue::S(value) = attr {
            Some(ResetPasswordTokenDestination::Email(
                ResetPasswordTokenDestinationEmail::restore(value),
            ))
        } else {
            None
        }
    }
}

struct ColumnExpires;
impl DynamoDbColumn for ColumnExpires {
    type Value = ExpireDateTime;

    fn as_name() -> &'static str {
        "expires"
    }
    fn into(value: Self::Value) -> AttributeValue {
        AttributeValue::N(value.extract().timestamp().to_string())
    }
    fn from(attr: AttributeValue) -> Option<Self::Value> {
        if let AttributeValue::N(value) = attr {
            value.parse::<i64>().ok().map({
                |value| {
                    ExpireDateTime::restore(Utc.from_utc_datetime(
                        &NaiveDateTime::from_timestamp_opt(value, 0).unwrap_or_default(),
                    ))
                }
            })
        } else {
            None
        }
    }
}

struct ColumnRequestedAt;
impl DynamoDbColumn for ColumnRequestedAt {
    type Value = AuthDateTime;

    fn as_name() -> &'static str {
        "requested_at"
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

struct ColumnResetAt;
impl DynamoDbColumn for ColumnResetAt {
    type Value = AuthDateTime;

    fn as_name() -> &'static str {
        "reset_at"
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

use std::collections::HashMap;

use chrono::{DateTime, NaiveDateTime, Utc};
use rusoto_dynamodb::{
    AttributeValue, DynamoDb, DynamoDbClient, GetItemInput, PutItemInput, UpdateItemInput,
};

use crate::auth::x_outside_feature::feature::AuthOutsideStore;

use crate::z_lib::repository::{
    dynamodb::helper::{string_value, timestamp_value, DynamoDbColumn},
    helper::repository_infra_error,
};

use crate::auth::user::password::reset::reset::infra::ResetTokenMoment;

use crate::{
    auth::{
        ticket::kernel::data::{AuthDateTime, ExpireDateTime},
        user::{
            kernel::data::AuthUserId,
            login_id::kernel::data::LoginId,
            password::reset::kernel::data::{
                ResetToken, ResetTokenDestination, ResetTokenDestinationExtract,
            },
        },
    },
    z_lib::repository::data::RepositoryError,
};

pub struct TableResetToken<'a> {
    client: &'a DynamoDbClient,
    table_name: &'a str,
}
impl<'a> TableResetToken<'a> {
    pub const fn new(feature: &'a AuthOutsideStore) -> Self {
        Self {
            client: &feature.dynamodb,
            table_name: feature.reset_token_table_name,
        }
    }

    fn key(reset_token: ResetToken) -> HashMap<String, AttributeValue> {
        vec![ColumnResetToken::to_attr_pair(reset_token)]
            .into_iter()
            .collect()
    }

    pub async fn get_reset_token_entry(
        &self,
        reset_token: ResetToken,
    ) -> Result<
        Option<(AuthUserId, LoginId, ResetTokenDestination, ResetTokenMoment)>,
        RepositoryError,
    > {
        let input = GetItemInput {
            table_name: self.table_name.into(),
            key: Self::key(reset_token),
            projection_expression: Some(
                vec![
                    ColumnUserId::as_name(),
                    ColumnLoginId::as_name(),
                    ColumnEmail::as_name(),
                    ColumnExpires::as_name(),
                    ColumnResetAt::as_name(),
                ]
                .join(","),
            ),
            ..Default::default()
        };

        let response = self
            .client
            .get_item(input)
            .await
            .map_err(|err| repository_infra_error("get reset token error", err))?;

        Ok(response.item.and_then(|mut attrs| {
            match (
                ColumnUserId::remove_value(&mut attrs),
                ColumnLoginId::remove_value(&mut attrs),
                ColumnEmail::remove_value(&mut attrs),
                ColumnExpires::remove_value(&mut attrs),
                ColumnResetAt::remove_value(&mut attrs),
            ) {
                (Some(user_id), Some(login_id), Some(email), Some(expires), reset_at) => Some((
                    user_id,
                    login_id,
                    email,
                    ResetTokenMoment::restore(expires, reset_at),
                )),
                _ => None,
            }
        }))
    }

    pub async fn put_reset_token(
        &self,
        reset_token: ResetToken,
        user_id: AuthUserId,
        login_id: LoginId,
        destination: ResetTokenDestination,
        expires: ExpireDateTime,
        requested_at: AuthDateTime,
    ) -> Result<(), RepositoryError> {
        // reset token がすでに登録されていたらエラーになる
        // reset token は削除しないので、衝突が発生したら reset token の桁数を増やす
        let input = PutItemInput {
            table_name: self.table_name.into(),
            item: vec![
                ColumnResetToken::to_attr_pair(reset_token),
                ColumnUserId::to_attr_pair(user_id),
                ColumnLoginId::to_attr_pair(login_id),
                ColumnEmail::to_attr_pair(destination),
                ColumnExpires::to_attr_pair(expires),
                ColumnRequestedAt::to_attr_pair(requested_at),
            ]
            .into_iter()
            .collect(),
            condition_expression: Some(format!(
                "attribute_not_exists({})",
                ColumnResetToken::as_name()
            )),
            ..Default::default()
        };

        self.client
            .put_item(input)
            .await
            .map_err(|err| repository_infra_error("put reset token error", err))?;
        Ok(())
    }
    pub async fn update_reset_at(
        &self,
        reset_token: ResetToken,
        reset_at: AuthDateTime,
    ) -> Result<(), RepositoryError> {
        let input = UpdateItemInput {
            table_name: self.table_name.into(),
            key: Self::key(reset_token),
            update_expression: Some(format!("set {} = :reset_at", ColumnResetAt::as_name())),
            expression_attribute_values: Some(
                vec![(":reset_at".to_owned(), ColumnResetAt::to_attr(reset_at))]
                    .into_iter()
                    .collect(),
            ),
            ..Default::default()
        };

        self.client
            .update_item(input)
            .await
            .map_err(|err| repository_infra_error("update reset at error", err))?;
        Ok(())
    }
}

struct ColumnResetToken;
impl DynamoDbColumn for ColumnResetToken {
    type Value = ResetToken;

    fn as_name() -> &'static str {
        "reset_token"
    }
    fn to_attr(value: Self::Value) -> AttributeValue {
        string_value(value.extract())
    }
    fn to_value(attr: AttributeValue) -> Option<Self::Value> {
        attr.s.map(|value| Self::Value::restore(value))
    }
}

struct ColumnLoginId;
impl DynamoDbColumn for ColumnLoginId {
    type Value = LoginId;

    fn as_name() -> &'static str {
        "login_id"
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

struct ColumnEmail;
impl DynamoDbColumn for ColumnEmail {
    type Value = ResetTokenDestination;

    fn as_name() -> &'static str {
        "email"
    }
    fn to_attr(value: Self::Value) -> AttributeValue {
        match value {
            Self::Value::None => AttributeValue::default(),
            Self::Value::Email(email) => string_value(email.extract()),
        }
    }
    fn to_value(attr: AttributeValue) -> Option<Self::Value> {
        attr.s
            .map(|value| Self::Value::restore(ResetTokenDestinationExtract::Email(value)))
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

struct ColumnRequestedAt;
impl DynamoDbColumn for ColumnRequestedAt {
    type Value = AuthDateTime;

    fn as_name() -> &'static str {
        "requested_at"
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

struct ColumnResetAt;
impl DynamoDbColumn for ColumnResetAt {
    type Value = AuthDateTime;

    fn as_name() -> &'static str {
        "reset_at"
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

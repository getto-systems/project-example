use std::{collections::HashMap, sync::Arc};

use aws_sdk_dynamodb::{types::AttributeValue, Client};

use crate::x_outside_feature::auth::feature::AuthAppFeature;

use crate::common::api::{
    feature::AsInfra,
    repository::dynamodb::detail::{DynamoDbColumn, ScanKey},
};

use crate::{
    auth::user::{
        kernel::data::AuthUserId,
        login_id::kernel::data::LoginId,
        password::reset::kernel::data::{
            ResetPasswordTokenDestination, ResetPasswordTokenDestinationEmail,
        },
    },
    common::api::repository::data::RepositoryError,
};

pub struct ConnectionLoginId {
    client: Arc<Client>,
    table_name: &'static str,
}

impl AsInfra<ConnectionLoginId> for Arc<AuthAppFeature> {
    fn as_infra(&self) -> ConnectionLoginId {
        ConnectionLoginId {
            client: Arc::clone(&self.store.dynamodb),
            table_name: self.store.login_id_table_name,
        }
    }
}

pub struct TableLoginId;

impl TableLoginId {
    fn key(login_id: LoginId) -> Option<HashMap<String, AttributeValue>> {
        Some(
            vec![ColumnLoginId::into_attr(login_id)]
                .into_iter()
                .collect(),
        )
    }

    pub async fn get_user_id(
        conn: &ConnectionLoginId,
        login_id: LoginId,
    ) -> Result<Option<AuthUserId>, RepositoryError> {
        let request = conn
            .client
            .get_item()
            .table_name(conn.table_name)
            .set_key(Self::key(login_id))
            .projection_expression(vec![ColumnUserId::as_name()].join(","));

        let response = request
            .send()
            .await
            .map_err(|err| ("get user-id error", err))?;

        Ok(response
            .item
            .and_then(|mut attrs| ColumnUserId::restore(&mut attrs)))
    }
    pub async fn get_reset_token_entry(
        conn: &ConnectionLoginId,
        login_id: LoginId,
    ) -> Result<Option<(AuthUserId, ResetPasswordTokenDestination)>, RepositoryError> {
        let request = conn
            .client
            .get_item()
            .table_name(conn.table_name)
            .set_key(Self::key(login_id))
            .projection_expression(
                vec![
                    ColumnUserId::as_name(),
                    ColumnResetTokenDestinationEmail::as_name(),
                ]
                .join(","),
            );

        let response = request
            .send()
            .await
            .map_err(|err| ("get reset-token entry error", err))?;

        Ok(response.item.and_then(|mut attrs| {
            match (
                ColumnUserId::restore(&mut attrs),
                ColumnResetTokenDestinationEmail::restore(&mut attrs),
            ) {
                (Some(user_id), email) => Some((
                    user_id,
                    email
                        .map(ResetPasswordTokenDestination::Email)
                        .unwrap_or_default(),
                )),
                _ => None,
            }
        }))
    }
    pub async fn get_reset_token_destination(
        conn: &ConnectionLoginId,
        login_id: LoginId,
    ) -> Result<Option<ResetPasswordTokenDestination>, RepositoryError> {
        let request = conn
            .client
            .get_item()
            .table_name(conn.table_name)
            .set_key(Self::key(login_id))
            .projection_expression(vec![ColumnResetTokenDestinationEmail::as_name()].join(","));

        let response = request
            .send()
            .await
            .map_err(|err| ("get reset-token entry error", err))?;

        Ok(response.item.map(|mut attrs| {
            ColumnResetTokenDestinationEmail::restore(&mut attrs)
                .map(ResetPasswordTokenDestination::Email)
                .unwrap_or_default()
        }))
    }

    pub async fn put_new_entry(
        conn: &ConnectionLoginId,
        login_id: LoginId,
        user_id: AuthUserId,
        reset_token_destination: ResetPasswordTokenDestination,
    ) -> Result<(), RepositoryError> {
        let mut item = vec![
            ColumnLoginId::into_attr(login_id),
            ColumnUserId::into_attr(user_id),
        ];
        if let ResetPasswordTokenDestination::Email(email) = reset_token_destination {
            item.push(ColumnResetTokenDestinationEmail::into_attr(email))
        }

        let request = conn
            .client
            .put_item()
            .table_name(conn.table_name)
            .set_item(Some(item.into_iter().collect()))
            .condition_expression(format!(
                "attribute_not_exists({})",
                ColumnLoginId::as_name()
            ));

        let _response = request
            .send()
            .await
            .map_err(|err| ("put new login-id error", err))?;

        Ok(())
    }

    pub async fn put_overwrite_entry(
        conn: &ConnectionLoginId,
        login_id: LoginId,
        user_id: AuthUserId,
        reset_token_destination: ResetPasswordTokenDestination,
    ) -> Result<(), RepositoryError> {
        let mut item = vec![
            ColumnLoginId::into_attr(login_id),
            ColumnUserId::into_attr(user_id),
        ];
        if let ResetPasswordTokenDestination::Email(email) = reset_token_destination {
            item.push(ColumnResetTokenDestinationEmail::into_attr(email))
        }

        let request = conn
            .client
            .put_item()
            .table_name(conn.table_name)
            .set_item(Some(item.into_iter().collect()))
            .condition_expression(format!(
                "attribute_not_exists({})",
                ColumnLoginId::as_name()
            ));

        let _response = request
            .send()
            .await
            .map_err(|err| ("put login-id error", err))?;

        Ok(())
    }
    pub async fn delete_entry(
        conn: &ConnectionLoginId,
        login_id: LoginId,
    ) -> Result<Option<(AuthUserId, ResetPasswordTokenDestination)>, RepositoryError> {
        let request = conn
            .client
            .delete_item()
            .table_name(conn.table_name)
            .set_key(Self::key(login_id));

        let response = request
            .send()
            .await
            .map_err(|err| ("delete login-id error", err))?;

        Ok(response.attributes.and_then(move |mut attrs| {
            match (
                ColumnUserId::restore(&mut attrs),
                ColumnResetTokenDestinationEmail::restore(&mut attrs),
            ) {
                (Some(user_id), reset_token_destination_email) => Some((
                    user_id,
                    reset_token_destination_email
                        .map(ResetPasswordTokenDestination::Email)
                        .unwrap_or_default(),
                )),
                _ => None,
            }
        }))
    }

    pub async fn update_reset_token_destination(
        conn: &ConnectionLoginId,
        login_id: LoginId,
        new_destination: ResetPasswordTokenDestination,
    ) -> Result<(), RepositoryError> {
        let request = conn
            .client
            .update_item()
            .table_name(conn.table_name)
            .set_key(Self::key(login_id));
        let request = match new_destination {
            ResetPasswordTokenDestination::None => request.update_expression(format!(
                "remove {}",
                ColumnResetTokenDestinationEmail::as_name()
            )),
            ResetPasswordTokenDestination::Email(email) => request
                .update_expression(format!(
                    "set {} = :email",
                    ColumnResetTokenDestinationEmail::as_name()
                ))
                .set_expression_attribute_values(Some(
                    vec![ColumnResetTokenDestinationEmail::into_attr_with_name(
                        ":email", email,
                    )]
                    .into_iter()
                    .collect(),
                )),
        };

        let _response = request
            .send()
            .await
            .map_err(|err| ("update reset-token-destination-email error", err))?;

        Ok(())
    }

    pub async fn scan_reset_token_destination(
        conn: &ConnectionLoginId,
    ) -> Result<Vec<(LoginId, ResetPasswordTokenDestination)>, RepositoryError> {
        let mut acc = vec![];
        let mut scan_key = ScanKey::FirstTime;
        while scan_key.has_next() {
            let (mut items, key) = Self::scan_reset_token_destination_part(conn, scan_key).await?;
            acc.append(&mut items);
            scan_key = key;
        }
        Ok(acc)
    }
    async fn scan_reset_token_destination_part(
        conn: &ConnectionLoginId,
        scan_key: ScanKey,
    ) -> Result<(Vec<(LoginId, ResetPasswordTokenDestination)>, ScanKey), RepositoryError> {
        let request = conn
            .client
            .scan()
            .table_name(conn.table_name)
            .set_exclusive_start_key(scan_key.extract())
            .projection_expression(
                vec![
                    ColumnLoginId::as_name(),
                    ColumnResetTokenDestinationEmail::as_name(),
                ]
                .join(","),
            );

        let response = request
            .send()
            .await
            .map_err(|err| ("scan user error", err))?;

        let items = match response.items {
            None => vec![],
            Some(items) => items
                .into_iter()
                .filter_map(|mut attrs| {
                    match (
                        ColumnLoginId::restore(&mut attrs),
                        ColumnResetTokenDestinationEmail::restore(&mut attrs),
                    ) {
                        (Some(login_id), email) => Some((login_id, email.into())),
                        _ => None,
                    }
                })
                .collect(),
        };

        Ok((items, ScanKey::next(response.last_evaluated_key)))
    }
}

struct ColumnLoginId;
impl DynamoDbColumn for ColumnLoginId {
    type Value = LoginId;

    fn as_name() -> &'static str {
        "login_id"
    }
    fn into(value: Self::Value) -> AttributeValue {
        AttributeValue::S(value.extract())
    }
    fn from(attr: AttributeValue) -> Option<Self::Value> {
        if let AttributeValue::S(value) = attr {
            Some(LoginId::restore(value))
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

struct ColumnResetTokenDestinationEmail;
impl DynamoDbColumn for ColumnResetTokenDestinationEmail {
    type Value = ResetPasswordTokenDestinationEmail;

    fn as_name() -> &'static str {
        "reset_token_destination_email"
    }
    fn into(value: Self::Value) -> AttributeValue {
        AttributeValue::S(value.extract())
    }
    fn from(attr: AttributeValue) -> Option<Self::Value> {
        if let AttributeValue::S(value) = attr {
            Some(ResetPasswordTokenDestinationEmail::restore(value))
        } else {
            None
        }
    }
}

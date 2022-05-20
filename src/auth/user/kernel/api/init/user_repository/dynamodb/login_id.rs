use std::collections::HashMap;

use rusoto_dynamodb::{
    AttributeValue, DeleteItemInput, DynamoDb, DynamoDbClient, GetItemInput, PutItemInput,
    ScanInput, UpdateItemInput,
};

use crate::auth::x_outside_feature::feature::AuthOutsideStore;

use crate::z_lib::repository::{
    dynamodb::helper::{string_value, DynamoDbColumn, ScanKey},
    helper::repository_infra_error,
};

use crate::auth::user::login_id::change::infra::OverwriteLoginIdEntry;

use crate::{
    auth::user::{
        kernel::data::AuthUserId,
        login_id::kernel::data::LoginId,
        password::reset::kernel::data::{ResetTokenDestination, ResetTokenDestinationEmail},
    },
    z_lib::repository::data::RepositoryError,
};

pub struct TableLoginId<'a> {
    client: &'a DynamoDbClient,
    table_name: &'a str,
}
impl<'a> TableLoginId<'a> {
    pub const fn new(feature: &'a AuthOutsideStore) -> Self {
        Self {
            client: &feature.dynamodb,
            table_name: feature.login_id_table_name,
        }
    }

    fn key(login_id: LoginId) -> HashMap<String, AttributeValue> {
        vec![ColumnLoginId::to_attr_pair(login_id)]
            .into_iter()
            .collect()
    }

    pub async fn get_user_id(
        &self,
        login_id: LoginId,
    ) -> Result<Option<AuthUserId>, RepositoryError> {
        let input = GetItemInput {
            table_name: self.table_name.into(),
            key: Self::key(login_id),
            projection_expression: Some(vec![ColumnUserId::as_name()].join(",")),
            ..Default::default()
        };

        let response = self
            .client
            .get_item(input)
            .await
            .map_err(|err| repository_infra_error("get user id error", err))?;

        Ok(response
            .item
            .and_then(|mut attrs| ColumnUserId::remove_value(&mut attrs)))
    }
    pub async fn check_login_id_registered(
        &self,
        login_id: LoginId,
    ) -> Result<bool, RepositoryError> {
        Ok(self.get_user_id(login_id).await?.is_some())
    }
    pub async fn get_overwrite_entry(
        &self,
        login_id: LoginId,
    ) -> Result<Option<OverwriteLoginIdEntry>, RepositoryError> {
        let input = GetItemInput {
            table_name: self.table_name.into(),
            key: TableLoginId::key(login_id.clone()),
            projection_expression: Some(
                vec![
                    ColumnUserId::as_name(),
                    ColumnResetTokenDestinationEmail::as_name(),
                ]
                .join(","),
            ),
            ..Default::default()
        };

        let response = self
            .client
            .get_item(input)
            .await
            .map_err(|err| repository_infra_error("get overwrite entry error", err))?;

        Ok(response.item.and_then(move |mut attrs| {
            match (
                ColumnUserId::remove_value(&mut attrs),
                ColumnResetTokenDestinationEmail::remove_value(&mut attrs),
            ) {
                (Some(user_id), reset_token_destination_email) => Some(OverwriteLoginIdEntry {
                    login_id,
                    user_id,
                    reset_token_destination: reset_token_destination_email
                        .map(ResetTokenDestination::Email),
                }),
                _ => None,
            }
        }))
    }
    pub async fn get_reset_token_entry(
        &self,
        login_id: LoginId,
    ) -> Result<Option<(AuthUserId, Option<ResetTokenDestination>)>, RepositoryError> {
        let input = GetItemInput {
            table_name: self.table_name.into(),
            key: TableLoginId::key(login_id),
            projection_expression: Some(
                vec![
                    ColumnUserId::as_name(),
                    ColumnResetTokenDestinationEmail::as_name(),
                ]
                .join(","),
            ),
            ..Default::default()
        };

        let response = self
            .client
            .get_item(input)
            .await
            .map_err(|err| repository_infra_error("get reset token entry error", err))?;

        Ok(response.item.and_then(|mut attrs| {
            match (
                ColumnUserId::remove_value(&mut attrs),
                ColumnResetTokenDestinationEmail::remove_value(&mut attrs),
            ) {
                (Some(user_id), email) => Some((user_id, email.map(ResetTokenDestination::Email))),
                _ => None,
            }
        }))
    }
    pub async fn get_reset_token_destination(
        &self,
        login_id: LoginId,
    ) -> Result<Option<ResetTokenDestination>, RepositoryError> {
        let input = GetItemInput {
            table_name: self.table_name.into(),
            key: TableLoginId::key(login_id),
            projection_expression: Some(
                vec![ColumnResetTokenDestinationEmail::as_name()].join(","),
            ),
            ..Default::default()
        };

        let response = self
            .client
            .get_item(input)
            .await
            .map_err(|err| repository_infra_error("get reset token entry error", err))?;

        Ok(response.item.map(|mut attrs| {
            ColumnResetTokenDestinationEmail::remove_value(&mut attrs)
                .map(ResetTokenDestination::Email)
                .unwrap_or(ResetTokenDestination::None)
        }))
    }

    pub async fn put_new_entry(
        &self,
        login_id: LoginId,
        user_id: AuthUserId,
        reset_token_destination: ResetTokenDestination,
    ) -> Result<(), RepositoryError> {
        let mut item = vec![
            ColumnLoginId::to_attr_pair(login_id),
            ColumnUserId::to_attr_pair(user_id),
        ];
        if let ResetTokenDestination::Email(email) = reset_token_destination {
            item.push(ColumnResetTokenDestinationEmail::to_attr_pair(email))
        }

        let input = PutItemInput {
            table_name: self.table_name.into(),
            item: item.into_iter().collect(),
            condition_expression: Some(format!(
                "attribute_not_exists({})",
                ColumnLoginId::as_name()
            )),
            ..Default::default()
        };

        self.client
            .put_item(input)
            .await
            .map_err(|err| repository_infra_error("put new login-id error", err))?;

        Ok(())
    }

    pub async fn put_overwrite_entry(
        &self,
        login_id: LoginId,
        user: OverwriteLoginIdEntry,
    ) -> Result<(), RepositoryError> {
        let mut item = vec![
            ColumnLoginId::to_attr_pair(login_id),
            ColumnUserId::to_attr_pair(user.user_id),
        ];
        if let Some(ResetTokenDestination::Email(email)) = user.reset_token_destination {
            item.push(ColumnResetTokenDestinationEmail::to_attr_pair(email))
        }

        let input = PutItemInput {
            table_name: self.table_name.into(),
            item: item.into_iter().collect(),
            condition_expression: Some(format!(
                "attribute_not_exists({})",
                ColumnLoginId::as_name()
            )),
            ..Default::default()
        };

        self.client
            .put_item(input)
            .await
            .map_err(|err| repository_infra_error("put login id error", err))?;

        Ok(())
    }
    pub async fn delete_entry(&self, login_id: LoginId) -> Result<(), RepositoryError> {
        let input = DeleteItemInput {
            table_name: self.table_name.into(),
            key: Self::key(login_id),
            ..Default::default()
        };

        self.client
            .delete_item(input)
            .await
            .map_err(|err| repository_infra_error("delete login id error", err))?;

        Ok(())
    }

    pub async fn update_reset_token_destination(
        &self,
        login_id: LoginId,
        new_destination: ResetTokenDestination,
    ) -> Result<(), RepositoryError> {
        let input = match new_destination {
            ResetTokenDestination::None => UpdateItemInput {
                table_name: self.table_name.into(),
                key: Self::key(login_id),
                update_expression: Some(format!(
                    "remove {}",
                    ColumnResetTokenDestinationEmail::as_name()
                )),
                ..Default::default()
            },
            ResetTokenDestination::Email(email) => UpdateItemInput {
                table_name: self.table_name.into(),
                key: Self::key(login_id),
                update_expression: Some(format!(
                    "set {} = :email",
                    ColumnResetTokenDestinationEmail::as_name()
                )),
                expression_attribute_values: Some(
                    vec![(
                        ":email".to_owned(),
                        ColumnResetTokenDestinationEmail::to_attr(email),
                    )]
                    .into_iter()
                    .collect(),
                ),
                ..Default::default()
            },
        };

        self.client.update_item(input).await.map_err(|err| {
            repository_infra_error("update reset token destination email error", err)
        })?;

        Ok(())
    }

    pub async fn scan_reset_token_destination(
        &self,
    ) -> Result<Vec<(LoginId, Option<ResetTokenDestination>)>, RepositoryError> {
        let mut acc = vec![];
        let mut scan_key = ScanKey::FirstTime;
        while scan_key.has_next() {
            let (mut items, key) = self.scan_reset_token_destination_part(scan_key).await?;
            acc.append(&mut items);
            scan_key = key;
        }
        Ok(acc)
    }
    async fn scan_reset_token_destination_part(
        &self,
        scan_key: ScanKey,
    ) -> Result<(Vec<(LoginId, Option<ResetTokenDestination>)>, ScanKey), RepositoryError> {
        let input = ScanInput {
            table_name: self.table_name.into(),
            exclusive_start_key: scan_key.extract(),
            projection_expression: Some(
                vec![
                    ColumnLoginId::as_name(),
                    ColumnResetTokenDestinationEmail::as_name(),
                ]
                .join(","),
            ),
            ..Default::default()
        };

        let response = self
            .client
            .scan(input)
            .await
            .map_err(|err| repository_infra_error("scan user error", err))?;

        let items = match response.items {
            None => vec![],
            Some(items) => items
                .into_iter()
                .filter_map(|mut attrs| {
                    match (
                        ColumnLoginId::remove_value(&mut attrs),
                        ColumnResetTokenDestinationEmail::remove_value(&mut attrs),
                    ) {
                        (Some(login_id), email) => {
                            Some((login_id, email.map(ResetTokenDestination::Email)))
                        }
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

struct ColumnResetTokenDestinationEmail;
impl DynamoDbColumn for ColumnResetTokenDestinationEmail {
    type Value = ResetTokenDestinationEmail;

    fn as_name() -> &'static str {
        "reset_token_destination_email"
    }
    fn to_attr(value: Self::Value) -> AttributeValue {
        string_value(value.extract())
    }
    fn to_value(attr: AttributeValue) -> Option<Self::Value> {
        attr.s.map(|value| Self::Value::restore(value))
    }
}

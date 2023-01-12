use std::collections::HashMap;

use rusoto_dynamodb::{
    AttributeValue, DeleteItemInput, DynamoDb, DynamoDbClient, GetItemInput, PutItemInput,
    ScanInput, UpdateItemInput,
};

use crate::common::api::repository::{
    dynamodb::helper::{string_set_value, string_value, DynamoDbColumn, ScanKey},
    helper::repository_infra_error,
};

use crate::auth::x_outside_feature::feature::AuthOutsideStore;

use crate::x_content::permission::AuthPermission;

use crate::auth::user::password::kernel::infra::HashedPassword;

use crate::{
    auth::{
        ticket::kernel::data::AuthPermissionGranted,
        user::{
            account::kernel::data::{AuthUserAccountAttrs, AuthUserMemo},
            kernel::data::AuthUserId,
            login_id::kernel::data::LoginId,
        },
    },
    common::api::repository::data::RepositoryError,
};

pub struct TableUser<'a> {
    client: &'a DynamoDbClient,
    table_name: &'a str,
}
impl<'a> TableUser<'a> {
    pub const fn new(feature: &'a AuthOutsideStore) -> Self {
        Self {
            client: &feature.dynamodb,
            table_name: feature.user_table_name,
        }
    }

    fn key(user_id: AuthUserId) -> HashMap<String, AttributeValue> {
        vec![ColumnUserId::to_attr_pair(user_id)]
            .into_iter()
            .collect()
    }

    pub async fn get_granted(
        &self,
        user_id: AuthUserId,
    ) -> Result<Option<AuthPermissionGranted>, RepositoryError> {
        let input = GetItemInput {
            table_name: self.table_name.into(),
            key: Self::key(user_id),
            projection_expression: Some(vec![ColumnGranted::as_name()].join(",")),
            ..Default::default()
        };

        let response = self
            .client
            .get_item(input)
            .await
            .map_err(|err| repository_infra_error("get granted error", err))?;

        Ok(response
            .item
            .map(move |mut attrs| ColumnGranted::remove_value(&mut attrs).unwrap_or_default()))
    }
    pub async fn get_password_and_granted(
        &self,
        user_id: AuthUserId,
    ) -> Result<Option<(HashedPassword, Option<AuthPermissionGranted>)>, RepositoryError> {
        let input = GetItemInput {
            table_name: self.table_name.into(),
            key: Self::key(user_id),
            projection_expression: Some(
                vec![ColumnPassword::as_name(), ColumnGranted::as_name()].join(","),
            ),
            ..Default::default()
        };

        let response = self
            .client
            .get_item(input)
            .await
            .map_err(|err| repository_infra_error("get password and granted error", err))?;

        Ok(response.item.and_then(move |mut attrs| {
            if let Some(hashed_password) = ColumnPassword::remove_value(&mut attrs) {
                Some((hashed_password, ColumnGranted::remove_value(&mut attrs)))
            } else {
                None
            }
        }))
    }
    pub async fn get_password(
        &self,
        user_id: AuthUserId,
    ) -> Result<Option<HashedPassword>, RepositoryError> {
        let input = GetItemInput {
            table_name: self.table_name.into(),
            key: Self::key(user_id),
            projection_expression: Some(vec![ColumnPassword::as_name()].join(",")),
            ..Default::default()
        };

        let response = self
            .client
            .get_item(input)
            .await
            .map_err(|err| repository_infra_error("get password error", err))?;

        Ok(response
            .item
            .and_then(move |mut attrs| ColumnPassword::remove_value(&mut attrs)))
    }
    pub async fn get_attrs(
        &self,
        user_id: AuthUserId,
    ) -> Result<Option<AuthUserAccountAttrs>, RepositoryError> {
        let input = GetItemInput {
            table_name: self.table_name.into(),
            key: Self::key(user_id),
            projection_expression: Some(
                vec![ColumnGranted::as_name(), ColumnMemo::as_name()].join(","),
            ),
            ..Default::default()
        };

        let response = self
            .client
            .get_item(input)
            .await
            .map_err(|err| repository_infra_error("get granted attrs error", err))?;

        Ok(response.item.map(move |mut attrs| AuthUserAccountAttrs {
            granted: ColumnGranted::remove_value(&mut attrs).unwrap_or_default(),
            memo: ColumnMemo::remove_value(&mut attrs).unwrap_or(AuthUserMemo::empty()),
        }))
    }
    pub async fn get_entry(
        &self,
        user_id: AuthUserId,
    ) -> Result<Option<EntryUser>, RepositoryError> {
        let input = GetItemInput {
            table_name: self.table_name.into(),
            key: Self::key(user_id),
            projection_expression: Some(
                vec![
                    ColumnLoginId::as_name(),
                    ColumnGranted::as_name(),
                    ColumnPassword::as_name(),
                    ColumnMemo::as_name(),
                ]
                .join(","),
            ),
            ..Default::default()
        };

        let response = self
            .client
            .get_item(input)
            .await
            .map_err(|err| repository_infra_error("get granted entry error", err))?;

        Ok(response.item.and_then(move |mut attrs| {
            if let Some(login_id) = ColumnLoginId::remove_value(&mut attrs) {
                Some(EntryUser {
                    login_id,
                    granted: ColumnGranted::remove_value(&mut attrs),
                    hashed_password: ColumnPassword::remove_value(&mut attrs),
                    memo: ColumnMemo::remove_value(&mut attrs),
                })
            } else {
                None
            }
        }))
    }

    pub async fn put_new_entry(
        &self,
        user_id: AuthUserId,
        login_id: LoginId,
        granted: AuthPermissionGranted,
        memo: AuthUserMemo,
    ) -> Result<(), RepositoryError> {
        self.put_entry(
            user_id,
            EntryUser {
                login_id,
                granted: Some(granted),
                hashed_password: None,
                memo: Some(memo),
            },
        )
        .await
    }
    pub async fn put_entry(
        &self,
        user_id: AuthUserId,
        entry: EntryUser,
    ) -> Result<(), RepositoryError> {
        let mut item = vec![
            ColumnUserId::to_attr_pair(user_id),
            ColumnLoginId::to_attr_pair(entry.login_id),
        ];
        if let Some(granted) = entry.granted {
            if !granted.is_empty() {
                item.push(ColumnGranted::to_attr_pair(granted))
            }
        }
        if let Some(hashed_password) = entry.hashed_password {
            item.push(ColumnPassword::to_attr_pair(hashed_password))
        }
        if let Some(memo) = entry.memo {
            item.push(ColumnMemo::to_attr_pair(memo))
        }

        let input = PutItemInput {
            table_name: self.table_name.into(),
            item: item.into_iter().collect(),
            condition_expression: Some(format!(
                "attribute_not_exists({})",
                ColumnUserId::as_name()
            )),
            ..Default::default()
        };

        self.client
            .put_item(input)
            .await
            .map_err(|err| repository_infra_error("put user error", err))?;

        Ok(())
    }

    pub async fn delete_entry(&self, user_id: AuthUserId) -> Result<(), RepositoryError> {
        let input = DeleteItemInput {
            table_name: self.table_name.into(),
            key: Self::key(user_id),
            ..Default::default()
        };

        self.client
            .delete_item(input)
            .await
            .map_err(|err| repository_infra_error("delete login id error", err))?;

        Ok(())
    }

    pub async fn update_password(
        &self,
        user_id: AuthUserId,
        password: HashedPassword,
    ) -> Result<(), RepositoryError> {
        let input = UpdateItemInput {
            table_name: self.table_name.into(),
            key: Self::key(user_id),
            update_expression: Some(format!("set {} = :password", ColumnPassword::as_name())),
            expression_attribute_values: Some(
                vec![(":password".to_owned(), ColumnPassword::to_attr(password))]
                    .into_iter()
                    .collect(),
            ),
            ..Default::default()
        };

        self.client
            .update_item(input)
            .await
            .map_err(|err| repository_infra_error("update password error", err))?;

        Ok(())
    }
    pub async fn update_login_id(
        &self,
        user_id: AuthUserId,
        new_login_id: LoginId,
    ) -> Result<(), RepositoryError> {
        let input = UpdateItemInput {
            table_name: self.table_name.into(),
            key: Self::key(user_id),
            update_expression: Some(format!("set {} = :login_id", ColumnLoginId::as_name())),
            expression_attribute_values: Some(
                vec![(":login_id".to_owned(), ColumnLoginId::to_attr(new_login_id))]
                    .into_iter()
                    .collect(),
            ),
            ..Default::default()
        };

        self.client
            .update_item(input)
            .await
            .map_err(|err| repository_infra_error("update login id error", err))?;

        Ok(())
    }
    pub async fn update_user(
        &self,
        user_id: AuthUserId,
        attrs: AuthUserAccountAttrs,
    ) -> Result<(), RepositoryError> {
        self.update_memo(user_id.clone(), attrs.memo).await?;

        if attrs.granted.is_empty() {
            self.remove_granted(user_id).await?;
        } else {
            self.set_granted(user_id, attrs.granted).await?;
        }

        Ok(())
    }
    async fn set_granted(
        &self,
        user_id: AuthUserId,
        granted: AuthPermissionGranted,
    ) -> Result<(), RepositoryError> {
        let input = UpdateItemInput {
            table_name: self.table_name.into(),
            key: Self::key(user_id),
            update_expression: Some(format!("set {} = :granted", ColumnGranted::as_name())),
            expression_attribute_values: Some(
                vec![(":granted".to_owned(), ColumnGranted::to_attr(granted))]
                    .into_iter()
                    .collect(),
            ),
            ..Default::default()
        };

        self.client
            .update_item(input)
            .await
            .map_err(|err| repository_infra_error("set granted error", err))?;

        Ok(())
    }
    async fn remove_granted(&self, user_id: AuthUserId) -> Result<(), RepositoryError> {
        let input = UpdateItemInput {
            table_name: self.table_name.into(),
            key: Self::key(user_id),
            update_expression: Some(format!(
                "remove {}",
                vec![ColumnGranted::as_name()].join(",")
            )),
            ..Default::default()
        };

        self.client
            .update_item(input)
            .await
            .map_err(|err| repository_infra_error("remove granted error", err))?;

        Ok(())
    }
    async fn update_memo(
        &self,
        user_id: AuthUserId,
        memo: AuthUserMemo,
    ) -> Result<(), RepositoryError> {
        let input = UpdateItemInput {
            table_name: self.table_name.into(),
            key: Self::key(user_id),
            update_expression: Some(format!("set {} = :memo", ColumnMemo::as_name())),
            expression_attribute_values: Some(
                vec![(":memo".to_owned(), ColumnMemo::to_attr(memo))]
                    .into_iter()
                    .collect(),
            ),
            ..Default::default()
        };

        self.client
            .update_item(input)
            .await
            .map_err(|err| repository_infra_error("set memo error", err))?;

        Ok(())
    }

    pub async fn scan_user(
        &self,
    ) -> Result<Vec<(LoginId, Option<AuthPermissionGranted>, Option<AuthUserMemo>)>, RepositoryError>
    {
        let mut acc = vec![];
        let mut scan_key = ScanKey::FirstTime;
        while scan_key.has_next() {
            let (mut items, key) = self.scan_user_part(scan_key).await?;
            acc.append(&mut items);
            scan_key = key;
        }
        Ok(acc)
    }
    async fn scan_user_part(
        &self,
        scan_key: ScanKey,
    ) -> Result<
        (
            Vec<(LoginId, Option<AuthPermissionGranted>, Option<AuthUserMemo>)>,
            ScanKey,
        ),
        RepositoryError,
    > {
        let input = ScanInput {
            table_name: self.table_name.into(),
            exclusive_start_key: scan_key.extract(),
            projection_expression: Some(
                vec![
                    ColumnLoginId::as_name(),
                    ColumnGranted::as_name(),
                    ColumnMemo::as_name(),
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
                    if let Some(login_id) = ColumnLoginId::remove_value(&mut attrs) {
                        Some((
                            login_id,
                            ColumnGranted::remove_value(&mut attrs),
                            ColumnMemo::remove_value(&mut attrs),
                        ))
                    } else {
                        None
                    }
                })
                .collect(),
        };

        Ok((items, ScanKey::next(response.last_evaluated_key)))
    }
}

pub struct EntryUser {
    pub login_id: LoginId,
    pub granted: Option<AuthPermissionGranted>,
    pub hashed_password: Option<HashedPassword>,
    pub memo: Option<AuthUserMemo>,
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

struct ColumnGranted;
impl DynamoDbColumn for ColumnGranted {
    type Value = AuthPermissionGranted;

    fn as_name() -> &'static str {
        "granted"
    }
    fn to_attr(value: Self::Value) -> AttributeValue {
        string_set_value(value.extract().into_iter().collect())
    }
    fn to_value(attr: AttributeValue) -> Option<Self::Value> {
        attr.ss.map(|value| {
            AuthPermissionGranted::restore(
                value
                    .into_iter()
                    .filter_map(AuthPermission::convert)
                    .collect(),
            )
        })
    }
}

struct ColumnPassword;
impl DynamoDbColumn for ColumnPassword {
    type Value = HashedPassword;

    fn as_name() -> &'static str {
        "password"
    }
    fn to_attr(value: Self::Value) -> AttributeValue {
        string_value(value.extract())
    }
    fn to_value(attr: AttributeValue) -> Option<Self::Value> {
        attr.s.map(|value| Self::Value::restore(value))
    }
}

struct ColumnMemo;
impl DynamoDbColumn for ColumnMemo {
    type Value = AuthUserMemo;

    fn as_name() -> &'static str {
        "memo"
    }
    fn to_attr(value: Self::Value) -> AttributeValue {
        string_value(value.extract())
    }
    fn to_value(attr: AttributeValue) -> Option<Self::Value> {
        attr.s.map(Self::Value::restore)
    }
}

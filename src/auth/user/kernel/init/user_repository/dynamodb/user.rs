use std::collections::HashMap;

use rusoto_dynamodb::{
    AttributeValue, DynamoDb, DynamoDbClient, GetItemInput, ScanInput, UpdateItemInput,
};

use crate::auth::x_outside_feature::feature::AuthOutsideStore;

use crate::z_lib::repository::{
    dynamodb::helper::{string_set_value, string_value, DynamoDbColumn, ScanKey},
    helper::infra_error,
};

use crate::auth::user::password::kernel::infra::HashedPassword;

use crate::{
    auth::user::{
        account::modify::data::ModifyAuthUserAccountChanges,
        kernel::data::{AuthUserId, GrantedAuthRoles},
        login_id::kernel::data::LoginId,
    },
    z_lib::repository::data::RepositoryError,
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

    pub async fn lookup_granted_roles(
        &self,
        user_id: AuthUserId,
    ) -> Result<Option<Option<GrantedAuthRoles>>, RepositoryError> {
        let input = GetItemInput {
            table_name: self.table_name.into(),
            key: Self::key(user_id),
            projection_expression: Some(vec![ColumnGrantedAuthRoles::as_name()].join(",")),
            ..Default::default()
        };

        let response = self
            .client
            .get_item(input)
            .await
            .map_err(|err| infra_error("lookup user and granted roles error", err))?;

        Ok(response
            .item
            .map(move |mut attrs| ColumnGrantedAuthRoles::remove_value(&mut attrs)))
    }
    pub async fn lookup_password_and_granted_roles(
        &self,
        user_id: AuthUserId,
    ) -> Result<Option<(HashedPassword, Option<GrantedAuthRoles>)>, RepositoryError> {
        let input = GetItemInput {
            table_name: self.table_name.into(),
            key: Self::key(user_id),
            projection_expression: Some(
                vec![
                    ColumnHashedPassword::as_name(),
                    ColumnGrantedAuthRoles::as_name(),
                ]
                .join(","),
            ),
            ..Default::default()
        };

        let response = self
            .client
            .get_item(input)
            .await
            .map_err(|err| infra_error("lookup password and granted roles error", err))?;

        Ok(response.item.and_then(move |mut attrs| {
            match (
                ColumnHashedPassword::remove_value(&mut attrs),
                ColumnGrantedAuthRoles::remove_value(&mut attrs),
            ) {
                (Some(hashed_password), granted_roles) => Some((hashed_password, granted_roles)),
                _ => None,
            }
        }))
    }
    pub async fn lookup_password(
        &self,
        user_id: AuthUserId,
    ) -> Result<Option<HashedPassword>, RepositoryError> {
        let input = GetItemInput {
            table_name: self.table_name.into(),
            key: Self::key(user_id),
            projection_expression: Some(vec![ColumnHashedPassword::as_name()].join(",")),
            ..Default::default()
        };

        let response = self
            .client
            .get_item(input)
            .await
            .map_err(|err| infra_error("lookup password error", err))?;

        Ok(response
            .item
            .and_then(move |mut attrs| ColumnHashedPassword::remove_value(&mut attrs)))
    }
    pub async fn lookup_modify_changes(
        &self,
        user_id: AuthUserId,
    ) -> Result<Option<ModifyAuthUserAccountChanges>, RepositoryError> {
        let input = GetItemInput {
            table_name: self.table_name.into(),
            key: Self::key(user_id),
            projection_expression: Some(vec![ColumnGrantedAuthRoles::as_name()].join(",")),
            ..Default::default()
        };

        let response = self
            .client
            .get_item(input)
            .await
            .map_err(|err| infra_error("lookup granted roles error", err))?;

        Ok(response
            .item
            .map(move |mut attrs| ModifyAuthUserAccountChanges {
                granted_roles: ColumnGrantedAuthRoles::remove_value(&mut attrs)
                    .unwrap_or(GrantedAuthRoles::empty()),
            }))
    }

    pub async fn update_password(
        &self,
        user_id: AuthUserId,
        password: HashedPassword,
    ) -> Result<(), RepositoryError> {
        let input = UpdateItemInput {
            table_name: self.table_name.into(),
            key: Self::key(user_id),
            update_expression: Some(format!(
                "set {} = :password",
                ColumnHashedPassword::as_name()
            )),
            expression_attribute_values: Some(
                vec![(
                    ":password".to_owned(),
                    ColumnHashedPassword::to_attr(password),
                )]
                .into_iter()
                .collect(),
            ),
            ..Default::default()
        };

        self.client
            .update_item(input)
            .await
            .map_err(|err| infra_error("update password error", err))?;

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
            .map_err(|err| infra_error("update login id error", err))?;

        Ok(())
    }
    pub async fn update_user(
        &self,
        user_id: AuthUserId,
        changes: ModifyAuthUserAccountChanges,
    ) -> Result<(), RepositoryError> {
        let input = UpdateItemInput {
            table_name: self.table_name.into(),
            key: Self::key(user_id),
            update_expression: Some(format!(
                "set {} = :granted_roles",
                ColumnGrantedAuthRoles::as_name()
            )),
            expression_attribute_values: Some(
                vec![(
                    ":granted_roles".to_owned(),
                    ColumnGrantedAuthRoles::to_attr(changes.granted_roles),
                )]
                .into_iter()
                .collect(),
            ),
            ..Default::default()
        };

        self.client
            .update_item(input)
            .await
            .map_err(|err| infra_error("update user error", err))?;

        Ok(())
    }

    pub async fn scan_user(
        &self,
    ) -> Result<Vec<(LoginId, Option<GrantedAuthRoles>)>, RepositoryError> {
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
    ) -> Result<(Vec<(LoginId, Option<GrantedAuthRoles>)>, ScanKey), RepositoryError> {
        let input = ScanInput {
            table_name: self.table_name.into(),
            exclusive_start_key: scan_key.extract(),
            projection_expression: Some(
                vec![ColumnLoginId::as_name(), ColumnGrantedAuthRoles::as_name()].join(","),
            ),
            ..Default::default()
        };

        let response = self
            .client
            .scan(input)
            .await
            .map_err(|err| infra_error("scan user error", err))?;

        let items = match response.items {
            None => vec![],
            Some(items) => items
                .into_iter()
                .filter_map(|mut attrs| {
                    match (
                        ColumnLoginId::remove_value(&mut attrs),
                        ColumnGrantedAuthRoles::remove_value(&mut attrs),
                    ) {
                        (Some(login_id), granted_roles) => Some((login_id, granted_roles)),
                        _ => None,
                    }
                })
                .collect(),
        };

        Ok((items, ScanKey::next(response.last_evaluated_key)))
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

struct ColumnGrantedAuthRoles;
impl DynamoDbColumn for ColumnGrantedAuthRoles {
    type Value = GrantedAuthRoles;

    fn as_name() -> &'static str {
        "granted_roles"
    }
    fn to_attr(value: Self::Value) -> AttributeValue {
        string_set_value(value.extract().into_iter().collect())
    }
    fn to_value(attr: AttributeValue) -> Option<Self::Value> {
        attr.ss
            .map(|value| Self::Value::restore(value.into_iter().collect()))
    }
}

struct ColumnHashedPassword;
impl DynamoDbColumn for ColumnHashedPassword {
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

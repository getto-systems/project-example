use std::{collections::HashMap, sync::Arc};

use aws_sdk_dynamodb::{types::AttributeValue, Client};

use crate::x_outside_feature::auth::feature::AuthAppFeature;

use crate::{
    common::api::{
        feature::AsInfra,
        repository::dynamodb::detail::{DynamoDbColumn, ScanKey},
    },
    x_content::permission::AuthPermission,
};

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

pub struct ConnectionUser {
    client: Arc<Client>,
    table_name: &'static str,
}

impl AsInfra<ConnectionUser> for Arc<AuthAppFeature> {
    fn as_infra(&self) -> ConnectionUser {
        ConnectionUser {
            client: Arc::clone(&self.store.dynamodb),
            table_name: self.store.user_table_name,
        }
    }
}

pub struct TableUser;

impl TableUser {
    fn key(user_id: AuthUserId) -> Option<HashMap<String, AttributeValue>> {
        Some(vec![ColumnUserId::into_attr(user_id)].into_iter().collect())
    }

    pub async fn get_granted(
        conn: &ConnectionUser,
        user_id: AuthUserId,
    ) -> Result<Option<AuthPermissionGranted>, RepositoryError> {
        let request = conn
            .client
            .get_item()
            .table_name(conn.table_name)
            .set_key(Self::key(user_id))
            .projection_expression(vec![ColumnGranted::as_name()].join(","));

        let response = request
            .send()
            .await
            .map_err(|err| ("get granted error", err))?;

        Ok(response
            .item
            .map(move |mut attrs| ColumnGranted::restore(&mut attrs).unwrap_or_default()))
    }
    pub async fn get_password_and_granted(
        conn: &ConnectionUser,
        user_id: AuthUserId,
    ) -> Result<Option<(HashedPassword, Option<AuthPermissionGranted>)>, RepositoryError> {
        let request = conn
            .client
            .get_item()
            .table_name(conn.table_name)
            .set_key(Self::key(user_id))
            .projection_expression(
                vec![ColumnPassword::as_name(), ColumnGranted::as_name()].join(","),
            );

        let response = request
            .send()
            .await
            .map_err(|err| ("get password and granted error", err))?;

        Ok(response.item.and_then(move |mut attrs| {
            if let Some(hashed_password) = ColumnPassword::restore(&mut attrs) {
                Some((hashed_password, ColumnGranted::restore(&mut attrs)))
            } else {
                None
            }
        }))
    }
    pub async fn get_password(
        conn: &ConnectionUser,
        user_id: AuthUserId,
    ) -> Result<Option<HashedPassword>, RepositoryError> {
        let request = conn
            .client
            .get_item()
            .table_name(conn.table_name)
            .set_key(Self::key(user_id))
            .projection_expression(vec![ColumnPassword::as_name()].join(","));

        let response = request
            .send()
            .await
            .map_err(|err| ("get password error", err))?;

        Ok(response
            .item
            .and_then(move |mut attrs| ColumnPassword::restore(&mut attrs)))
    }
    pub async fn get_attrs(
        conn: &ConnectionUser,
        user_id: AuthUserId,
    ) -> Result<Option<AuthUserAccountAttrs>, RepositoryError> {
        let request = conn
            .client
            .get_item()
            .table_name(conn.table_name)
            .set_key(Self::key(user_id))
            .projection_expression(vec![ColumnGranted::as_name(), ColumnMemo::as_name()].join(","));

        let response = request
            .send()
            .await
            .map_err(|err| ("get attrs error", err))?;

        Ok(response.item.map(move |mut attrs| AuthUserAccountAttrs {
            granted: ColumnGranted::restore(&mut attrs).unwrap_or_default(),
            memo: ColumnMemo::restore(&mut attrs).unwrap_or_default(),
        }))
    }
    pub async fn get_entry(
        conn: &ConnectionUser,
        user_id: AuthUserId,
    ) -> Result<Option<EntryUser>, RepositoryError> {
        let request = conn
            .client
            .get_item()
            .table_name(conn.table_name)
            .set_key(Self::key(user_id))
            .projection_expression(
                vec![
                    ColumnLoginId::as_name(),
                    ColumnGranted::as_name(),
                    ColumnPassword::as_name(),
                    ColumnMemo::as_name(),
                ]
                .join(","),
            );

        let response = request
            .send()
            .await
            .map_err(|err| ("get entry error", err))?;

        Ok(response.item.and_then(move |mut attrs| {
            if let Some(login_id) = ColumnLoginId::restore(&mut attrs) {
                Some(EntryUser {
                    login_id,
                    granted: ColumnGranted::restore(&mut attrs),
                    hashed_password: ColumnPassword::restore(&mut attrs),
                    memo: ColumnMemo::restore(&mut attrs),
                })
            } else {
                None
            }
        }))
    }

    pub async fn put_new_entry(
        conn: &ConnectionUser,
        user_id: AuthUserId,
        login_id: LoginId,
        granted: AuthPermissionGranted,
        memo: AuthUserMemo,
    ) -> Result<(), RepositoryError> {
        Self::put_entry(
            conn,
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
        conn: &ConnectionUser,
        user_id: AuthUserId,
        entry: EntryUser,
    ) -> Result<(), RepositoryError> {
        let mut item = vec![
            ColumnUserId::into_attr(user_id),
            ColumnLoginId::into_attr(entry.login_id),
        ];
        if let Some(granted) = entry.granted {
            if !granted.is_empty() {
                item.push(ColumnGranted::into_attr(granted))
            }
        }
        if let Some(hashed_password) = entry.hashed_password {
            item.push(ColumnPassword::into_attr(hashed_password))
        }
        if let Some(memo) = entry.memo {
            item.push(ColumnMemo::into_attr(memo))
        }

        let request = conn
            .client
            .put_item()
            .table_name(conn.table_name)
            .set_item(Some(item.into_iter().collect()))
            .condition_expression(format!("attribute_not_exists({})", ColumnUserId::as_name()));

        let _response = request
            .send()
            .await
            .map_err(|err| ("put entry error", err))?;

        Ok(())
    }

    pub async fn delete_entry(
        conn: &ConnectionUser,
        user_id: AuthUserId,
    ) -> Result<(), RepositoryError> {
        let request = conn
            .client
            .delete_item()
            .table_name(conn.table_name)
            .set_key(Self::key(user_id));

        let _response = request
            .send()
            .await
            .map_err(|err| ("delete entry error", err))?;

        Ok(())
    }

    pub async fn update_password(
        conn: &ConnectionUser,
        user_id: AuthUserId,
        password: HashedPassword,
    ) -> Result<(), RepositoryError> {
        let request = conn
            .client
            .update_item()
            .table_name(conn.table_name)
            .set_key(Self::key(user_id))
            .update_expression(format!("set {} = :password", ColumnPassword::as_name()))
            .set_expression_attribute_values(Some(
                vec![ColumnPassword::into_attr_with_name(":password", password)]
                    .into_iter()
                    .collect(),
            ));

        let _response = request
            .send()
            .await
            .map_err(|err| ("update password error", err))?;

        Ok(())
    }
    pub async fn update_login_id(
        conn: &ConnectionUser,
        user_id: AuthUserId,
        new_login_id: LoginId,
    ) -> Result<(), RepositoryError> {
        let request = conn
            .client
            .update_item()
            .table_name(conn.table_name)
            .set_key(Self::key(user_id))
            .update_expression(format!("set {} = :login_id", ColumnLoginId::as_name()))
            .set_expression_attribute_values(Some(
                vec![ColumnLoginId::into_attr_with_name(
                    ":login_id",
                    new_login_id,
                )]
                .into_iter()
                .collect(),
            ));

        let _response = request
            .send()
            .await
            .map_err(|err| ("update login-id error", err))?;

        Ok(())
    }
    pub async fn update_user(
        conn: &ConnectionUser,
        user_id: AuthUserId,
        attrs: AuthUserAccountAttrs,
    ) -> Result<(), RepositoryError> {
        Self::update_memo(conn, user_id.clone(), attrs.memo).await?;

        if attrs.granted.is_empty() {
            Self::remove_granted(conn, user_id).await?;
        } else {
            Self::set_granted(conn, user_id, attrs.granted).await?;
        }

        Ok(())
    }
    async fn set_granted(
        conn: &ConnectionUser,
        user_id: AuthUserId,
        granted: AuthPermissionGranted,
    ) -> Result<(), RepositoryError> {
        let request = conn
            .client
            .update_item()
            .table_name(conn.table_name)
            .set_key(Self::key(user_id))
            .update_expression(format!("set {} = :granted", ColumnGranted::as_name()))
            .set_expression_attribute_values(Some(
                vec![ColumnGranted::into_attr_with_name(":granted", granted)]
                    .into_iter()
                    .collect(),
            ));

        let _response = request
            .send()
            .await
            .map_err(|err| ("set granted error", err))?;

        Ok(())
    }
    async fn remove_granted(
        conn: &ConnectionUser,
        user_id: AuthUserId,
    ) -> Result<(), RepositoryError> {
        let request = conn
            .client
            .update_item()
            .table_name(conn.table_name)
            .set_key(Self::key(user_id))
            .update_expression(format!(
                "remove {}",
                vec![ColumnGranted::as_name()].join(",")
            ));

        let _response = request
            .send()
            .await
            .map_err(|err| ("remove granted error", err))?;

        Ok(())
    }
    async fn update_memo(
        conn: &ConnectionUser,
        user_id: AuthUserId,
        memo: AuthUserMemo,
    ) -> Result<(), RepositoryError> {
        let request = conn
            .client
            .update_item()
            .table_name(conn.table_name)
            .set_key(Self::key(user_id))
            .update_expression(format!("set {} = :memo", ColumnMemo::as_name()))
            .set_expression_attribute_values(Some(
                vec![ColumnMemo::into_attr_with_name(":memo", memo)]
                    .into_iter()
                    .collect(),
            ));

        let _response = request
            .send()
            .await
            .map_err(|err| ("update memo error", err))?;

        Ok(())
    }

    pub async fn scan_user(
        conn: &ConnectionUser,
    ) -> Result<Vec<(LoginId, Option<AuthPermissionGranted>, Option<AuthUserMemo>)>, RepositoryError>
    {
        let mut acc = vec![];
        let mut scan_key = ScanKey::FirstTime;
        while scan_key.has_next() {
            let (mut items, key) = Self::scan_user_part(conn, scan_key).await?;
            acc.append(&mut items);
            scan_key = key;
        }
        Ok(acc)
    }
    async fn scan_user_part(
        conn: &ConnectionUser,
        scan_key: ScanKey,
    ) -> Result<
        (
            Vec<(LoginId, Option<AuthPermissionGranted>, Option<AuthUserMemo>)>,
            ScanKey,
        ),
        RepositoryError,
    > {
        let request = conn
            .client
            .scan()
            .table_name(conn.table_name)
            .set_exclusive_start_key(scan_key.extract())
            .set_projection_expression(Some(
                vec![
                    ColumnLoginId::as_name(),
                    ColumnGranted::as_name(),
                    ColumnMemo::as_name(),
                ]
                .join(","),
            ));

        let response = request
            .send()
            .await
            .map_err(|err| ("scan user error", err))?;

        let items = match response.items {
            None => vec![],
            Some(items) => items
                .into_iter()
                .filter_map(|mut attrs| {
                    if let Some(login_id) = ColumnLoginId::restore(&mut attrs) {
                        Some((
                            login_id,
                            ColumnGranted::restore(&mut attrs),
                            ColumnMemo::restore(&mut attrs),
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

struct ColumnGranted;
impl DynamoDbColumn for ColumnGranted {
    type Value = AuthPermissionGranted;

    fn as_name() -> &'static str {
        "granted"
    }
    fn into(value: Self::Value) -> AttributeValue {
        AttributeValue::Ss(value.extract().into_iter().collect())
    }
    fn from(attr: AttributeValue) -> Option<Self::Value> {
        if let AttributeValue::Ss(value) = attr {
            Some(AuthPermissionGranted::restore(
                value
                    .into_iter()
                    .filter_map(AuthPermission::convert)
                    .collect(),
            ))
        } else {
            None
        }
    }
}

struct ColumnPassword;
impl DynamoDbColumn for ColumnPassword {
    type Value = HashedPassword;

    fn as_name() -> &'static str {
        "password"
    }
    fn into(value: Self::Value) -> AttributeValue {
        AttributeValue::S(value.extract())
    }
    fn from(attr: AttributeValue) -> Option<Self::Value> {
        if let AttributeValue::S(value) = attr {
            Some(HashedPassword::restore(value))
        } else {
            None
        }
    }
}

struct ColumnMemo;
impl DynamoDbColumn for ColumnMemo {
    type Value = AuthUserMemo;

    fn as_name() -> &'static str {
        "memo"
    }
    fn into(value: Self::Value) -> AttributeValue {
        AttributeValue::S(value.extract())
    }
    fn from(attr: AttributeValue) -> Option<Self::Value> {
        if let AttributeValue::S(value) = attr {
            Some(AuthUserMemo::restore(value))
        } else {
            None
        }
    }
}

use std::{
    collections::{HashMap, HashSet},
    convert::TryInto,
    iter::FromIterator,
};

use chrono::{DateTime, NaiveDateTime, Utc};
use rusoto_core::RusotoError;
use rusoto_dynamodb::{
    AttributeValue, DeleteItemError, DeleteItemInput, DynamoDb, DynamoDbClient, GetItemError,
    GetItemInput, PutItemError, PutItemInput, ScanError, ScanInput, UpdateItemError,
    UpdateItemInput,
};

use crate::auth::x_outside_feature::feature::AuthOutsideStore;

use crate::z_lib::repository::{
    dynamodb::helper::{string_set_value, string_value, timestamp_value, ScanKey},
    helper::infra_error,
};

use crate::auth::user::{
    account::{
        modify::infra::ModifyAuthUserAccountRepository,
        search::infra::SearchAuthUserAccountRepository,
    },
    login_id::change::infra::{OverrideLoginIdRepository, OverrideUserEntry},
    password::{
        authenticate::infra::AuthenticatePasswordRepository,
        change::infra::{ChangePasswordRepository, OverridePasswordRepository},
        kernel::infra::HashedPassword,
        reset::{
            request_token::infra::RegisterResetTokenRepository,
            reset::infra::{ResetPasswordRepository, ResetTokenMoment},
            token_destination::change::infra::ChangeResetTokenDestinationRepository,
        },
    },
};

use crate::{
    auth::{
        ticket::kernel::data::{AuthDateTime, ExpireDateTime},
        user::{
            account::{
                kernel::data::AuthUserAccount,
                modify::data::ModifyAuthUserAccountChanges,
                search::data::{
                    SearchAuthUserAccountBasket, SearchAuthUserAccountFilter,
                    SearchAuthUserAccountSortKey,
                },
            },
            kernel::data::{AuthUserId, GrantedAuthRoles},
            login_id::kernel::data::LoginId,
            password::reset::kernel::data::{
                ResetToken, ResetTokenDestination, ResetTokenDestinationEmail,
                ResetTokenDestinationExtract,
            },
        },
    },
    z_lib::{
        repository::data::RepositoryError,
        search::data::{SearchOffset, SearchPage, SearchSortOrder},
    },
};

pub struct DynamoDbAuthUserRepository<'a> {
    client: &'a DynamoDbClient,
    user: &'a str,
    login_id: &'a str,
    reset_token: &'a str,
}

impl<'a> DynamoDbAuthUserRepository<'a> {
    pub const fn new(feature: &'a AuthOutsideStore) -> Self {
        Self {
            client: &feature.dynamodb,
            user: feature.user_table_name,
            login_id: feature.login_id_table_name,
            reset_token: feature.reset_token_table_name,
        }
    }
}

#[async_trait::async_trait]
impl<'client> OverrideLoginIdRepository for DynamoDbAuthUserRepository<'client> {
    async fn lookup_user<'a>(
        &self,
        login_id: &'a LoginId,
    ) -> Result<Option<OverrideUserEntry>, RepositoryError> {
        lookup_user(self, login_id).await
    }

    async fn check_login_id_registered<'a>(
        &self,
        login_id: &'a LoginId,
    ) -> Result<bool, RepositoryError> {
        check_login_id_registered(self, login_id).await
    }

    async fn override_login_id<'a>(
        &self,
        user: OverrideUserEntry,
        new_login_id: LoginId,
    ) -> Result<(), RepositoryError> {
        override_login_id(self, user, new_login_id).await
    }
}
async fn lookup_user<'client, 'a>(
    repository: &DynamoDbAuthUserRepository<'client>,
    login_id: &'a LoginId,
) -> Result<Option<OverrideUserEntry>, RepositoryError> {
    Ok(get_login_id_entry(repository, login_id.clone())
        .await
        .map_err(|err| infra_error("get login id entry error", err))?)
}
async fn check_login_id_registered<'client, 'a>(
    repository: &DynamoDbAuthUserRepository<'client>,
    new_login_id: &'a LoginId,
) -> Result<bool, RepositoryError> {
    Ok(get_user_id(repository, new_login_id.clone())
        .await
        .map_err(|err| infra_error("get user id error", err))?
        .is_some())
}
async fn override_login_id<'client, 'a>(
    repository: &DynamoDbAuthUserRepository<'client>,
    user: OverrideUserEntry,
    new_login_id: LoginId,
) -> Result<(), RepositoryError> {
    update_login_id(repository, user.user_id.clone(), new_login_id.clone())
        .await
        .map_err(|err| infra_error("update login id error", err))?;

    delete_login_id_entry(repository, user.login_id.clone())
        .await
        .map_err(|err| infra_error("delete login id entry error", err))?;

    put_login_id_entry(repository, new_login_id, user)
        .await
        .map_err(|err| infra_error("put login id entry error", err))?;

    Ok(())
}

#[async_trait::async_trait]
impl<'client> AuthenticatePasswordRepository for DynamoDbAuthUserRepository<'client> {
    async fn lookup_user_id<'a>(
        &self,
        login_id: &'a LoginId,
    ) -> Result<Option<AuthUserId>, RepositoryError> {
        lookup_user_id(self, login_id).await
    }

    async fn lookup_granted_roles<'a>(
        &self,
        user_id: &'a AuthUserId,
    ) -> Result<Option<GrantedAuthRoles>, RepositoryError> {
        lookup_granted_roles(self, user_id).await
    }

    async fn lookup_password<'a>(
        &self,
        user_id: &'a AuthUserId,
    ) -> Result<Option<HashedPassword>, RepositoryError> {
        lookup_password(self, user_id).await
    }
}
async fn lookup_user_id<'client, 'a>(
    repository: &DynamoDbAuthUserRepository<'client>,
    login_id: &'a LoginId,
) -> Result<Option<AuthUserId>, RepositoryError> {
    get_user_id(repository, login_id.clone())
        .await
        .map_err(|err| infra_error("get user error", err))
}
async fn lookup_granted_roles<'client, 'a>(
    repository: &DynamoDbAuthUserRepository<'client>,
    user_id: &'a AuthUserId,
) -> Result<Option<GrantedAuthRoles>, RepositoryError> {
    Ok(Some(
        get_granted_roles(repository, user_id.clone())
            .await
            .map_err(|err| infra_error("get granted roles error", err))?,
    ))
}
async fn lookup_password<'client, 'a>(
    repository: &DynamoDbAuthUserRepository<'client>,
    user_id: &'a AuthUserId,
) -> Result<Option<HashedPassword>, RepositoryError> {
    get_password(repository, user_id.clone())
        .await
        .map_err(|err| infra_error("get password error", err))
}

#[async_trait::async_trait]
impl<'client> ChangePasswordRepository for DynamoDbAuthUserRepository<'client> {
    async fn lookup_password<'a>(
        &self,
        user_id: &'a AuthUserId,
    ) -> Result<Option<HashedPassword>, RepositoryError> {
        lookup_password(self, user_id).await
    }

    async fn change_password<'a>(
        &self,
        user_id: &'a AuthUserId,
        new_password: HashedPassword,
    ) -> Result<(), RepositoryError> {
        change_password(self, user_id, new_password).await
    }
}
async fn change_password<'client, 'a>(
    repository: &DynamoDbAuthUserRepository<'client>,
    user_id: &'a AuthUserId,
    new_password: HashedPassword,
) -> Result<(), RepositoryError> {
    update_password(repository, user_id.clone(), new_password)
        .await
        .map_err(|err| infra_error("update password error", err))
}

#[async_trait::async_trait]
impl<'client> OverridePasswordRepository for DynamoDbAuthUserRepository<'client> {
    async fn lookup_user_id<'a>(
        &self,
        login_id: &'a LoginId,
    ) -> Result<Option<AuthUserId>, RepositoryError> {
        lookup_user_id(self, login_id).await
    }

    async fn override_password<'a>(
        &self,
        user_id: &'a AuthUserId,
        new_password: HashedPassword,
    ) -> Result<(), RepositoryError> {
        override_password(self, user_id, new_password).await
    }
}
async fn override_password<'client, 'a>(
    repository: &DynamoDbAuthUserRepository<'client>,
    user_id: &'a AuthUserId,
    new_password: HashedPassword,
) -> Result<(), RepositoryError> {
    update_password(repository, user_id.clone(), new_password)
        .await
        .map_err(|err| infra_error("update password error", err))
}

#[async_trait::async_trait]
impl<'client> ModifyAuthUserAccountRepository for DynamoDbAuthUserRepository<'client> {
    async fn lookup_user(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<(AuthUserId, ModifyAuthUserAccountChanges)>, RepositoryError> {
        lookup_modify_user_data(self, login_id).await
    }

    async fn modify_user(
        &self,
        user_id: &AuthUserId,
        data: ModifyAuthUserAccountChanges,
    ) -> Result<(), RepositoryError> {
        modify_user(self, user_id, data).await
    }
}
async fn lookup_modify_user_data<'client>(
    repository: &DynamoDbAuthUserRepository<'client>,
    login_id: &LoginId,
) -> Result<Option<(AuthUserId, ModifyAuthUserAccountChanges)>, RepositoryError> {
    match get_user_id(repository, login_id.clone())
        .await
        .map_err(|err| infra_error("get login id error", err))?
    {
        None => Ok(None),
        Some(user_id) => Ok(Some((
            user_id.clone(),
            get_modify_user_data(repository, &user_id).await?,
        ))),
    }
}
async fn modify_user<'client>(
    repository: &DynamoDbAuthUserRepository<'client>,
    user_id: &AuthUserId,
    data: ModifyAuthUserAccountChanges,
) -> Result<(), RepositoryError> {
    update_granted_roles(repository, user_id.clone(), data.granted_roles)
        .await
        .map_err(|err| infra_error("update granted roles error", err))
}
async fn get_modify_user_data<'client>(
    repository: &DynamoDbAuthUserRepository<'client>,
    user_id: &AuthUserId,
) -> Result<ModifyAuthUserAccountChanges, RepositoryError> {
    Ok(ModifyAuthUserAccountChanges {
        granted_roles: get_granted_roles(repository, user_id.clone())
            .await
            .map_err(|err| infra_error("get granted roles error", err))?,
    })
}

#[async_trait::async_trait]
impl<'client> ChangeResetTokenDestinationRepository for DynamoDbAuthUserRepository<'client> {
    async fn lookup_destination(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<ResetTokenDestination>, RepositoryError> {
        lookup_reset_token_user(self, login_id)
            .await
            .map(|user| user.map(|(_user_id, destination)| destination))
    }

    async fn change_destination(
        &self,
        login_id: &LoginId,
        data: ResetTokenDestination,
    ) -> Result<(), RepositoryError> {
        change_reset_token_destination(self, login_id, data).await
    }
}
async fn change_reset_token_destination<'client>(
    repository: &DynamoDbAuthUserRepository<'client>,
    login_id: &LoginId,
    data: ResetTokenDestination,
) -> Result<(), RepositoryError> {
    update_destination(repository, login_id.clone(), data)
        .await
        .map_err(|err| infra_error("update destination error", err))
}

async fn update_destination<'client>(
    repository: &DynamoDbAuthUserRepository<'client>,
    login_id: LoginId,
    new_destination: ResetTokenDestination,
) -> Result<(), RusotoError<UpdateItemError>> {
    let mut key = AttributeMap::new();
    key.add_login_id(login_id);

    match new_destination {
        ResetTokenDestination::None => {
            let input = UpdateItemInput {
                table_name: repository.user.into(),
                key: key.extract(),
                update_expression: Some("set reset_token_destination_email = null".into()),
                ..Default::default()
            };

            repository.client.update_item(input).await?;
        }
        ResetTokenDestination::Email(email) => {
            let mut item = AttributeMap::new();
            item.add_destination_email_as(email, ":email");

            let input = UpdateItemInput {
                table_name: repository.user.into(),
                key: key.extract(),
                update_expression: Some("set reset_token_destination_email = :email".into()),
                expression_attribute_values: Some(item.extract()),
                ..Default::default()
            };

            repository.client.update_item(input).await?;
        }
    }

    Ok(())
}

#[async_trait::async_trait]
impl<'client> RegisterResetTokenRepository for DynamoDbAuthUserRepository<'client> {
    async fn lookup_user(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<(AuthUserId, ResetTokenDestination)>, RepositoryError> {
        lookup_reset_token_user(self, login_id).await
    }

    async fn register_reset_token(
        &self,
        reset_token: ResetToken,
        user_id: AuthUserId,
        login_id: LoginId,
        destination: ResetTokenDestination,
        expires: ExpireDateTime,
        requested_at: AuthDateTime,
    ) -> Result<(), RepositoryError> {
        register_reset_token(
            self,
            reset_token,
            user_id,
            login_id,
            destination,
            expires,
            requested_at,
        )
        .await
    }
}
async fn lookup_reset_token_user<'client>(
    repository: &DynamoDbAuthUserRepository<'client>,
    login_id: &LoginId,
) -> Result<Option<(AuthUserId, ResetTokenDestination)>, RepositoryError> {
    let mut key = AttributeMap::new();
    key.add_login_id(login_id.clone());

    let input = GetItemInput {
        table_name: repository.login_id.into(),
        key: key.extract(),
        projection_expression: Some("user_id, reset_token_destination_email".into()),
        ..Default::default()
    };

    let response = repository
        .client
        .get_item(input)
        .await
        .map_err(|err| infra_error("get destination error", err))?;

    Ok(response.item.and_then(|mut attrs| {
        match (
            attrs.remove("user_id").and_then(|attr| attr.s),
            attrs
                .remove("reset_token_destination_email")
                .and_then(|attr| attr.s),
        ) {
            (Some(user_id), Some(email)) => Some((
                AuthUserId::restore(user_id),
                ResetTokenDestination::restore(ResetTokenDestinationExtract::Email(email)),
            )),
            _ => None,
        }
    }))
}
async fn register_reset_token<'client>(
    repository: &DynamoDbAuthUserRepository<'client>,
    reset_token: ResetToken,
    user_id: AuthUserId,
    login_id: LoginId,
    destination: ResetTokenDestination,
    expires: ExpireDateTime,
    requested_at: AuthDateTime,
) -> Result<(), RepositoryError> {
    let mut item = AttributeMap::new();
    item.add_reset_token(reset_token);
    item.add_login_id(login_id);
    item.add_user_id(user_id);
    item.add_destination(destination);
    item.add_expires(expires);
    item.add_requested_at(requested_at);

    let input = PutItemInput {
        table_name: repository.reset_token.into(),
        item: item.extract(),
        condition_expression: Some("attribute_not_exists(reset_token)".into()),
        ..Default::default()
    };

    repository
        .client
        .put_item(input)
        .await
        .map_err(|err| infra_error("put reset token error", err))?;
    Ok(())
}

#[async_trait::async_trait]
impl<'client> ResetPasswordRepository for DynamoDbAuthUserRepository<'client> {
    async fn lookup_reset_token_entry(
        &self,
        reset_token: &ResetToken,
    ) -> Result<
        Option<(AuthUserId, LoginId, ResetTokenDestination, ResetTokenMoment)>,
        RepositoryError,
    > {
        lookup_reset_token_entry(self, reset_token).await
    }

    async fn lookup_granted_roles<'a>(
        &self,
        user_id: &'a AuthUserId,
    ) -> Result<Option<GrantedAuthRoles>, RepositoryError> {
        lookup_granted_roles(self, user_id).await
    }

    async fn reset_password(
        &self,
        reset_token: &ResetToken,
        user_id: &AuthUserId,
        new_password: HashedPassword,
        reset_at: AuthDateTime,
    ) -> Result<(), RepositoryError> {
        reset_password(self, reset_token, user_id, new_password, reset_at).await
    }
}
async fn lookup_reset_token_entry<'client>(
    repository: &DynamoDbAuthUserRepository<'client>,
    reset_token: &ResetToken,
) -> Result<Option<(AuthUserId, LoginId, ResetTokenDestination, ResetTokenMoment)>, RepositoryError>
{
    let mut key = AttributeMap::new();
    key.add_reset_token(reset_token.clone());

    let input = GetItemInput {
        table_name: repository.reset_token.into(),
        key: key.extract(),
        projection_expression: Some("user_id, login_id, email, expires, reset_at".into()),
        ..Default::default()
    };

    let response = repository
        .client
        .get_item(input)
        .await
        .map_err(|err| infra_error("get reset token error", err))?;

    Ok(response.item.and_then(|mut attrs| {
        match (
            attrs
                .remove("user_id")
                .and_then(|attr| attr.s)
                .map(AuthUserId::restore),
            attrs
                .remove("login_id")
                .and_then(|attr| attr.s)
                .map(LoginId::restore),
            attrs.remove("email").and_then(|attr| attr.s).map(|email| {
                ResetTokenDestination::restore(ResetTokenDestinationExtract::Email(email))
            }),
            attrs
                .remove("expires")
                .and_then(|attr| attr.n)
                .and_then(|value| value.parse::<i64>().ok())
                .map(|expires| {
                    ExpireDateTime::restore(DateTime::<Utc>::from_utc(
                        NaiveDateTime::from_timestamp(expires, 0),
                        Utc,
                    ))
                }),
            attrs
                .remove("reset_at")
                .and_then(|attr| attr.n)
                .and_then(|value| value.parse::<i64>().ok())
                .map(|reset_at| {
                    AuthDateTime::restore(DateTime::<Utc>::from_utc(
                        NaiveDateTime::from_timestamp(reset_at, 0),
                        Utc,
                    ))
                }),
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
async fn reset_password<'client, 'a>(
    repository: &DynamoDbAuthUserRepository<'client>,
    reset_token: &ResetToken,
    user_id: &AuthUserId,
    new_password: HashedPassword,
    reset_at: AuthDateTime,
) -> Result<(), RepositoryError> {
    update_reset_at(repository, reset_token.clone(), reset_at)
        .await
        .map_err(|err| infra_error("update reset error", err))?;

    update_password(repository, user_id.clone(), new_password)
        .await
        .map_err(|err| infra_error("reset password error", err))?;

    Ok(())
}

async fn get_login_id_entry<'client>(
    repository: &DynamoDbAuthUserRepository<'client>,
    login_id: LoginId,
) -> Result<Option<OverrideUserEntry>, RusotoError<GetItemError>> {
    let mut key = AttributeMap::new();
    key.add_login_id(login_id.clone());

    let input = GetItemInput {
        table_name: repository.login_id.into(),
        key: key.extract(),
        projection_expression: Some("user_id, reset_token_destination_email".into()),
        ..Default::default()
    };

    let response = repository.client.get_item(input).await?;

    Ok(response.item.and_then(move |mut attrs| {
        match (
            attrs.remove("user_id").and_then(|attr| attr.s),
            attrs
                .remove("reset_token_destination_email")
                .and_then(|attr| attr.s),
        ) {
            (Some(user_id), email) => Some(OverrideUserEntry {
                login_id,
                user_id: AuthUserId::restore(user_id),
                reset_token_destination: email
                    .map(|email| ResetTokenDestinationExtract::Email(email))
                    .unwrap_or(ResetTokenDestinationExtract::None),
            }),
            _ => None,
        }
    }))
}
async fn delete_login_id_entry<'client>(
    repository: &DynamoDbAuthUserRepository<'client>,
    old_login_id: LoginId,
) -> Result<(), RusotoError<DeleteItemError>> {
    let mut key = AttributeMap::new();
    key.add_login_id(old_login_id);

    let input = DeleteItemInput {
        table_name: repository.login_id.into(),
        key: key.extract(),
        ..Default::default()
    };

    repository.client.delete_item(input).await?;
    Ok(())
}
async fn put_login_id_entry<'client>(
    repository: &DynamoDbAuthUserRepository<'client>,
    new_login_id: LoginId,
    user: OverrideUserEntry,
) -> Result<(), RusotoError<PutItemError>> {
    let mut item = AttributeMap::new();
    item.add_login_id_entry(new_login_id, user);

    let input = PutItemInput {
        table_name: repository.login_id.into(),
        item: item.extract(),
        condition_expression: Some("attribute_not_exists(user_id)".into()),
        ..Default::default()
    };

    repository.client.put_item(input).await?;
    Ok(())
}
async fn get_user_id<'client>(
    repository: &DynamoDbAuthUserRepository<'client>,
    login_id: LoginId,
) -> Result<Option<AuthUserId>, RusotoError<GetItemError>> {
    let mut key = AttributeMap::new();
    key.add_login_id(login_id);

    let input = GetItemInput {
        table_name: repository.login_id.into(),
        key: key.extract(),
        projection_expression: Some("user_id".into()),
        ..Default::default()
    };

    let response = repository.client.get_item(input).await?;

    Ok(response
        .item
        .and_then(|mut attrs| attrs.remove("user_id"))
        .and_then(|attr| attr.s)
        .map(|user_id| AuthUserId::restore(user_id)))
}
async fn get_granted_roles<'client>(
    repository: &DynamoDbAuthUserRepository<'client>,
    user_id: AuthUserId,
) -> Result<GrantedAuthRoles, RusotoError<GetItemError>> {
    let mut key = AttributeMap::new();
    key.add_user_id(user_id);

    let input = GetItemInput {
        table_name: repository.user.into(),
        key: key.extract(),
        projection_expression: Some("granted_roles".into()),
        ..Default::default()
    };

    let response = repository.client.get_item(input).await?;

    let found = response
        .item
        .and_then(|mut attrs| attrs.remove("granted_roles"))
        .and_then(|attr| attr.ss)
        .map(|roles| HashSet::from_iter(roles.into_iter()));

    Ok(match found {
        None => GrantedAuthRoles::empty(),
        Some(roles) => GrantedAuthRoles::restore(roles),
    })
}
async fn update_granted_roles<'client>(
    repository: &DynamoDbAuthUserRepository<'client>,
    user_id: AuthUserId,
    new_granted_roles: GrantedAuthRoles,
) -> Result<(), RusotoError<UpdateItemError>> {
    let mut key = AttributeMap::new();
    key.add_user_id(user_id);

    let mut item = AttributeMap::new();
    item.add_granted_roles_as(new_granted_roles, ":granted_roles");

    let input = UpdateItemInput {
        table_name: repository.user.into(),
        key: key.extract(),
        update_expression: Some("set granted_roles = :granted_roles".into()),
        expression_attribute_values: Some(item.extract()),
        ..Default::default()
    };

    repository.client.update_item(input).await?;

    Ok(())
}
async fn update_login_id<'client>(
    repository: &DynamoDbAuthUserRepository<'client>,
    user_id: AuthUserId,
    new_login_id: LoginId,
) -> Result<(), RusotoError<UpdateItemError>> {
    let mut key = AttributeMap::new();
    key.add_user_id(user_id);

    let mut item = AttributeMap::new();
    item.add_login_id_as(new_login_id, ":login_id");

    let input = UpdateItemInput {
        table_name: repository.user.into(),
        key: key.extract(),
        update_expression: Some("set login_id = :login_id".into()),
        expression_attribute_values: Some(item.extract()),
        ..Default::default()
    };

    repository.client.update_item(input).await?;

    Ok(())
}
async fn get_password<'client>(
    repository: &DynamoDbAuthUserRepository<'client>,
    user_id: AuthUserId,
) -> Result<Option<HashedPassword>, RusotoError<GetItemError>> {
    let mut key = AttributeMap::new();
    key.add_user_id(user_id);

    let input = GetItemInput {
        table_name: repository.user.into(),
        key: key.extract(),
        projection_expression: Some("password".into()),
        ..Default::default()
    };

    let response = repository.client.get_item(input).await?;

    Ok(response
        .item
        .and_then(|mut attrs| attrs.remove("password"))
        .and_then(|attr| attr.s)
        .map(|password| HashedPassword::restore(password)))
}
async fn update_password<'client>(
    repository: &DynamoDbAuthUserRepository<'client>,
    user_id: AuthUserId,
    password: HashedPassword,
) -> Result<(), RusotoError<UpdateItemError>> {
    let mut key = AttributeMap::new();
    key.add_user_id(user_id);

    let mut item = AttributeMap::new();
    item.add_password_as(password, ":password");

    let input = UpdateItemInput {
        table_name: repository.user.into(),
        key: key.extract(),
        update_expression: Some("set password = :password".into()),
        expression_attribute_values: Some(item.extract()),
        ..Default::default()
    };

    repository.client.update_item(input).await?;

    Ok(())
}
async fn update_reset_at<'client, 'a>(
    repository: &DynamoDbAuthUserRepository<'client>,
    reset_token: ResetToken,
    reset_at: AuthDateTime,
) -> Result<(), RusotoError<UpdateItemError>> {
    let mut key = AttributeMap::new();
    key.add_reset_token(reset_token);

    let mut item = AttributeMap::new();
    item.add_reset_at_as(reset_at, ":reset_at");

    let input = UpdateItemInput {
        table_name: repository.reset_token.into(),
        key: key.extract(),
        update_expression: Some("set reset_at = :reset_at".into()),
        expression_attribute_values: Some(item.extract()),
        ..Default::default()
    };

    repository.client.update_item(input).await?;
    Ok(())
}

#[async_trait::async_trait]
impl<'client> SearchAuthUserAccountRepository for DynamoDbAuthUserRepository<'client> {
    async fn search(
        &self,
        filter: SearchAuthUserAccountFilter,
    ) -> Result<SearchAuthUserAccountBasket, RepositoryError> {
        search_user_account(&self, filter).await
    }
}

async fn search_user_account<'client>(
    repository: &DynamoDbAuthUserRepository<'client>,
    filter: SearchAuthUserAccountFilter,
) -> Result<SearchAuthUserAccountBasket, RepositoryError> {
    // 業務用アプリケーションなので、ユーザー数は 100を超えない
    // dynamodb から全てのデータを取得してフィルタ、ソートする
    let mut users = scan_user(repository)
        .await
        .map_err(|err| infra_error("scan user error", err))?;
    let all: i32 = users
        .len()
        .try_into()
        .map_err(|err| infra_error("convert users length error", err))?;

    match filter.sort().key() {
        SearchAuthUserAccountSortKey::LoginId => {
            users.sort_by_cached_key(|user| user.login_id.as_str().to_owned());
            match filter.sort().order() {
                SearchSortOrder::Normal => (),
                SearchSortOrder::Reverse => users.reverse(),
            }
        }
    };

    let mut users: Vec<AuthUserAccount> = users
        .into_iter()
        .filter(|user| match filter.login_id() {
            None => true,
            Some(login_id) => user.login_id.as_str() == login_id,
        })
        .collect();

    let limit = 1000;
    let offset = SearchOffset { all, limit }.detect(filter.offset());
    let mut users = users.split_off(
        offset
            .try_into()
            .map_err(|err| infra_error("convert offset error", err))?,
    );
    users.truncate(
        limit
            .try_into()
            .map_err(|err| infra_error("convert limit error", err))?,
    );

    Ok(SearchAuthUserAccountBasket {
        page: SearchPage { all, limit, offset },
        sort: filter.into_sort(),
        users,
    })
}

async fn scan_user<'client>(
    repository: &DynamoDbAuthUserRepository<'client>,
) -> Result<Vec<AuthUserAccount>, RusotoError<ScanError>> {
    let mut acc = vec![];
    let mut scan_key = ScanKey::FirstTime;
    while scan_key.has_next() {
        let (mut items, key) = scan_user_part(repository, scan_key).await?;
        acc.append(&mut items);
        scan_key = key;
    }
    Ok(acc)
}
async fn scan_user_part<'client>(
    repository: &DynamoDbAuthUserRepository<'client>,
    scan_key: ScanKey,
) -> Result<(Vec<AuthUserAccount>, ScanKey), RusotoError<ScanError>> {
    let input = ScanInput {
        table_name: repository.user.into(),
        exclusive_start_key: scan_key.extract(),
        projection_expression: Some(
            "login_id, granted_roles, reset_token_destination_email".into(),
        ),
        ..Default::default()
    };

    let response = repository.client.scan(input).await?;

    let items = match response.items {
        None => vec![],
        Some(items) => items
            .into_iter()
            .filter_map(|mut item| {
                match (
                    item.remove("login_id").and_then(|attr| attr.s),
                    item.remove("granted_roles")
                        .and_then(|attr| attr.ss)
                        .map(|roles| HashSet::from_iter(roles)),
                    item.remove("reset_token_destination_email")
                        .and_then(|attr| attr.s),
                ) {
                    (Some(login_id), granted_roles, reset_token_destination_email) => {
                        Some(AuthUserAccount {
                            login_id: LoginId::restore(login_id),
                            granted_roles: GrantedAuthRoles::restore(
                                granted_roles.unwrap_or(HashSet::new()),
                            ),
                            reset_token_destination: {
                                match reset_token_destination_email {
                                    None => ResetTokenDestination::None,
                                    Some(email) => ResetTokenDestination::restore(
                                        ResetTokenDestinationExtract::Email(email),
                                    ),
                                }
                            },
                        })
                    }
                    _ => None,
                }
            })
            .collect(),
    };

    Ok((items, ScanKey::next(response.last_evaluated_key)))
}

struct AttributeMap(HashMap<String, AttributeValue>);

impl AttributeMap {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn extract(self) -> HashMap<String, AttributeValue> {
        self.0
    }

    fn add(&mut self, key: &str, attr: AttributeValue) {
        self.0.insert(key.into(), attr);
    }

    fn add_login_id(&mut self, login_id: LoginId) {
        self.add_login_id_as(login_id, "login_id");
    }
    fn add_login_id_entry(&mut self, login_id: LoginId, user: OverrideUserEntry) {
        self.add_login_id(login_id);
        self.add_user_id(user.user_id);
        match user.reset_token_destination {
            ResetTokenDestinationExtract::None => {}
            ResetTokenDestinationExtract::Email(email) => {
                self.add("reset_token_destination_email", string_value(email));
            }
        }
    }
    fn add_user_id(&mut self, user_id: AuthUserId) {
        self.add("user_id", string_value(user_id.extract()));
    }
    fn add_reset_token(&mut self, reset_token: ResetToken) {
        self.add("reset_token", string_value(reset_token.extract()));
    }
    fn add_destination(&mut self, destination: ResetTokenDestination) {
        match destination {
            ResetTokenDestination::None => (),
            ResetTokenDestination::Email(email) => {
                self.add("email", string_value(email.extract()));
            }
        }
    }
    fn add_expires(&mut self, expires: ExpireDateTime) {
        self.add("expires", timestamp_value(expires.extract()));
    }
    fn add_requested_at(&mut self, requested_at: AuthDateTime) {
        self.add("requested_at", timestamp_value(requested_at.extract()));
    }

    fn add_login_id_as(&mut self, login_id: LoginId, name: &str) {
        self.add(name, string_value(login_id.extract()));
    }
    fn add_granted_roles_as(&mut self, granted_roles: GrantedAuthRoles, name: &str) {
        self.add(
            name,
            string_set_value(granted_roles.extract().into_iter().collect()),
        );
    }
    fn add_password_as(&mut self, password: HashedPassword, name: &str) {
        self.add(name, string_value(password.extract()));
    }
    fn add_reset_at_as(&mut self, reset_at: AuthDateTime, name: &str) {
        self.add(name, timestamp_value(reset_at.extract()));
    }
    fn add_destination_email_as(&mut self, email: ResetTokenDestinationEmail, name: &str) {
        self.add(name, string_value(email.extract()));
    }
}

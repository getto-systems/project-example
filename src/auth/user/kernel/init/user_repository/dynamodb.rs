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

use crate::{
    auth::x_outside_feature::feature::AuthOutsideStore,
    z_lib::repository::dynamodb::helper::string_set_value,
};

use crate::z_lib::repository::{
    dynamodb::helper::{string_value, timestamp_value, ScanKey},
    helper::infra_error,
};

use crate::auth::user::{
    account::{
        modify::infra::ModifyAuthUserAccountRepository,
        search::infra::SearchAuthUserAccountRepository,
    },
    kernel::infra::AuthUserRepository,
    login_id::change::infra::OverrideLoginIdRepository,
    password::{
        authenticate::infra::VerifyPasswordRepository,
        change::infra::{ChangePasswordRepository, OverridePasswordRepository},
        kernel::infra::{AuthUserPasswordHasher, AuthUserPasswordMatcher, HashedPassword},
        reset::{
            kernel::infra::{ResetTokenEntry, ResetTokenEntryExtract},
            request_token::infra::{RegisterResetTokenRepository, ResetTokenDestinationRepository},
            reset::infra::ResetPasswordRepository,
        },
    },
};

use crate::{
    auth::{
        ticket::kernel::data::{AuthDateTime, ExpireDateTime},
        user::{
            account::{
                kernel::data::AuthUserAccount,
                modify::data::AuthUserAccountChanges,
                search::data::{
                    SearchAuthUserAccountBasket, SearchAuthUserAccountFilter,
                    SearchAuthUserAccountSortKey,
                },
            },
            kernel::data::{AuthUser, AuthUserExtract, AuthUserId, GrantedAuthRoles},
            login_id::{change::data::OverrideLoginIdRepositoryError, kernel::data::LoginId},
            password::{
                authenticate::data::VerifyPasswordRepositoryError,
                change::data::{ChangePasswordRepositoryError, OverridePasswordRepositoryError},
                reset::{
                    kernel::data::{
                        ResetToken, ResetTokenDestination, ResetTokenDestinationExtract,
                    },
                    request_token::data::RegisterResetTokenRepositoryError,
                    reset::data::ResetPasswordRepositoryError,
                },
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
impl<'client> AuthUserRepository for DynamoDbAuthUserRepository<'client> {
    async fn get(&self, user_id: &AuthUserId) -> Result<Option<AuthUser>, RepositoryError> {
        get_user(self, user_id).await
    }
}
async fn get_user<'client>(
    repository: &DynamoDbAuthUserRepository<'client>,
    user_id: &AuthUserId,
) -> Result<Option<AuthUser>, RepositoryError> {
    // login id が存在すればユーザーは登録されているとみなす
    if let None = get_login_id(repository, user_id.clone())
        .await
        .map_err(|err| infra_error("get login id error", err))?
    {
        return Ok(None);
    }

    let roles = get_granted_roles(repository, user_id.clone())
        .await
        .map_err(|err| infra_error("get granted roles error", err))?;

    Ok(Some(
        AuthUserExtract {
            user_id: user_id.clone().extract(),
            granted_roles: roles.extract(),
        }
        .restore(),
    ))
}

#[async_trait::async_trait]
impl<'client> OverrideLoginIdRepository for DynamoDbAuthUserRepository<'client> {
    async fn override_login_id<'a>(
        &self,
        login_id: &'a LoginId,
        new_login_id: LoginId,
    ) -> Result<(), OverrideLoginIdRepositoryError> {
        override_login_id(self, login_id, new_login_id).await
    }
}
async fn override_login_id<'client, 'a>(
    repository: &DynamoDbAuthUserRepository<'client>,
    login_id: &'a LoginId,
    new_login_id: LoginId,
) -> Result<(), OverrideLoginIdRepositoryError> {
    let entry = get_login_id_entry(repository, login_id.clone())
        .await
        .map_err(|err| {
            OverrideLoginIdRepositoryError::RepositoryError(infra_error(
                "get login id entry error",
                err,
            ))
        })?
        .ok_or(OverrideLoginIdRepositoryError::UserNotFound)?;

    if let Some(_) = get_user_id(repository, new_login_id.clone())
        .await
        .map_err(|err| {
            OverrideLoginIdRepositoryError::RepositoryError(infra_error("get user id error", err))
        })?
    {
        return Err(OverrideLoginIdRepositoryError::LoginIdAlreadyRegistered);
    }

    update_login_id(repository, entry.user_id.clone(), new_login_id.clone())
        .await
        .map_err(|err| {
            OverrideLoginIdRepositoryError::RepositoryError(infra_error(
                "update login id error",
                err,
            ))
        })?;

    delete_login_id_entry(repository, login_id.clone())
        .await
        .map_err(|err| {
            OverrideLoginIdRepositoryError::RepositoryError(infra_error(
                "delete login id entry error",
                err,
            ))
        })?;

    put_login_id_entry(repository, new_login_id, entry)
        .await
        .map_err(|err| {
            OverrideLoginIdRepositoryError::RepositoryError(infra_error(
                "put login id entry error",
                err,
            ))
        })?;
    Ok(())
}

#[async_trait::async_trait]
impl<'client> VerifyPasswordRepository for DynamoDbAuthUserRepository<'client> {
    async fn verify_password<'a>(
        &self,
        login_id: &'a LoginId,
        matcher: impl 'a + AuthUserPasswordMatcher,
    ) -> Result<AuthUserId, VerifyPasswordRepositoryError> {
        verify_password(self, login_id, matcher).await
    }
}
async fn verify_password<'client, 'a>(
    repository: &DynamoDbAuthUserRepository<'client>,
    login_id: &'a LoginId,
    matcher: impl 'a + AuthUserPasswordMatcher,
) -> Result<AuthUserId, VerifyPasswordRepositoryError> {
    let user_id = get_user_id(repository, login_id.clone())
        .await
        .map_err(|err| {
            VerifyPasswordRepositoryError::RepositoryError(infra_error("get user error", err))
        })?
        .ok_or(VerifyPasswordRepositoryError::UserNotFound)?;

    let password = get_password(repository, user_id.clone())
        .await
        .map_err(|err| {
            VerifyPasswordRepositoryError::RepositoryError(infra_error("get password error", err))
        })?
        .ok_or(VerifyPasswordRepositoryError::PasswordNotFound)?;

    let matched = matcher
        .match_password(&password)
        .map_err(VerifyPasswordRepositoryError::PasswordHashError)?;

    if !matched {
        return Err(VerifyPasswordRepositoryError::PasswordNotMatched);
    }

    Ok(user_id)
}

#[async_trait::async_trait]
impl<'client> ChangePasswordRepository for DynamoDbAuthUserRepository<'client> {
    async fn change_password<'a>(
        &self,
        user_id: &'a AuthUserId,
        matcher: impl 'a + AuthUserPasswordMatcher,
        hasher: impl 'a + AuthUserPasswordHasher,
    ) -> Result<(), ChangePasswordRepositoryError> {
        change_password(self, user_id, matcher, hasher).await
    }
}
async fn change_password<'client, 'a>(
    repository: &DynamoDbAuthUserRepository<'client>,
    user_id: &'a AuthUserId,
    matcher: impl 'a + AuthUserPasswordMatcher,
    hasher: impl 'a + AuthUserPasswordHasher,
) -> Result<(), ChangePasswordRepositoryError> {
    let password = get_password(repository, user_id.clone())
        .await
        .map_err(|err| {
            ChangePasswordRepositoryError::RepositoryError(infra_error("get password error", err))
        })?
        .ok_or(ChangePasswordRepositoryError::PasswordNotFound)?;

    let matched = matcher
        .match_password(&password)
        .map_err(ChangePasswordRepositoryError::PasswordHashError)?;

    if !matched {
        return Err(ChangePasswordRepositoryError::PasswordNotMatched);
    }

    let password = hasher
        .hash_password()
        .map_err(ChangePasswordRepositoryError::PasswordHashError)?;

    update_password(repository, user_id.clone(), password)
        .await
        .map_err(|err| {
            ChangePasswordRepositoryError::RepositoryError(infra_error(
                "update password error",
                err,
            ))
        })
}

#[async_trait::async_trait]
impl<'client> OverridePasswordRepository for DynamoDbAuthUserRepository<'client> {
    async fn override_password<'a>(
        &self,
        login_id: &'a LoginId,
        hasher: impl 'a + AuthUserPasswordHasher,
    ) -> Result<(), OverridePasswordRepositoryError> {
        override_password(self, login_id, hasher).await
    }
}
async fn override_password<'client, 'a>(
    repository: &DynamoDbAuthUserRepository<'client>,
    login_id: &'a LoginId,
    hasher: impl 'a + AuthUserPasswordHasher,
) -> Result<(), OverridePasswordRepositoryError> {
    let password = hasher
        .hash_password()
        .map_err(OverridePasswordRepositoryError::PasswordHashError)?;

    let user_id = get_user_id(repository, login_id.clone())
        .await
        .map_err(|err| {
            OverridePasswordRepositoryError::RepositoryError(infra_error("get user id error", err))
        })?
        .ok_or(OverridePasswordRepositoryError::UserNotFound)?;

    update_password(repository, user_id, password)
        .await
        .map_err(|err| {
            OverridePasswordRepositoryError::RepositoryError(infra_error(
                "update password error",
                err,
            ))
        })
}

#[async_trait::async_trait]
impl<'client> ModifyAuthUserAccountRepository for DynamoDbAuthUserRepository<'client> {
    async fn lookup_user(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<(AuthUserId, AuthUserAccountChanges)>, RepositoryError> {
        lookup_modify_user_data(self, login_id).await
    }

    async fn modify_user(
        &self,
        user_id: &AuthUserId,
        data: AuthUserAccountChanges,
    ) -> Result<(), RepositoryError> {
        modify_user(self, user_id, data).await
    }

    async fn get_updated_user(
        &self,
        user_id: &AuthUserId,
    ) -> Result<AuthUserAccountChanges, RepositoryError> {
        get_modify_user_data(self, user_id).await
    }
}
async fn lookup_modify_user_data<'client>(
    repository: &DynamoDbAuthUserRepository<'client>,
    login_id: &LoginId,
) -> Result<Option<(AuthUserId, AuthUserAccountChanges)>, RepositoryError> {
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
    data: AuthUserAccountChanges,
) -> Result<(), RepositoryError> {
    update_granted_roles(repository, user_id.clone(), data.granted_roles)
        .await
        .map_err(|err| infra_error("update granted roles error", err))
}
async fn get_modify_user_data<'client>(
    repository: &DynamoDbAuthUserRepository<'client>,
    user_id: &AuthUserId,
) -> Result<AuthUserAccountChanges, RepositoryError> {
    Ok(AuthUserAccountChanges {
        granted_roles: get_granted_roles(repository, user_id.clone())
            .await
            .map_err(|err| infra_error("get granted roles error", err))?,
    })
}

#[async_trait::async_trait]
impl<'client> ResetTokenDestinationRepository for DynamoDbAuthUserRepository<'client> {
    async fn get(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<ResetTokenDestination>, RepositoryError> {
        get_destination(self, login_id).await
    }
}
async fn get_destination<'client>(
    repository: &DynamoDbAuthUserRepository<'client>,
    login_id: &LoginId,
) -> Result<Option<ResetTokenDestination>, RepositoryError> {
    let mut key = AttributeMap::new();
    key.add_login_id(login_id.clone());

    let input = GetItemInput {
        table_name: repository.login_id.into(),
        key: key.extract(),
        projection_expression: Some("reset_token_destination_email".into()),
        ..Default::default()
    };

    let response = repository
        .client
        .get_item(input)
        .await
        .map_err(|err| infra_error("get destination error", err))?;

    Ok(response
        .item
        .and_then(|mut attrs| attrs.remove("reset_token_destination_email"))
        .and_then(|attr| attr.s)
        .map(|email| ResetTokenDestination::restore(ResetTokenDestinationExtract::Email(email))))
}

#[async_trait::async_trait]
impl<'client> RegisterResetTokenRepository for DynamoDbAuthUserRepository<'client> {
    async fn register_reset_token(
        &self,
        reset_token: ResetToken,
        login_id: LoginId,
        destination: ResetTokenDestination,
        expires: ExpireDateTime,
        requested_at: AuthDateTime,
    ) -> Result<(), RegisterResetTokenRepositoryError> {
        register_reset_token(
            self,
            reset_token,
            login_id,
            destination,
            expires,
            requested_at,
        )
        .await
    }
}
async fn register_reset_token<'client>(
    repository: &DynamoDbAuthUserRepository<'client>,
    reset_token: ResetToken,
    login_id: LoginId,
    destination: ResetTokenDestination,
    expires: ExpireDateTime,
    requested_at: AuthDateTime,
) -> Result<(), RegisterResetTokenRepositoryError> {
    let user_id = get_user_id(repository, login_id.clone())
        .await
        .map_err(|err| {
            RegisterResetTokenRepositoryError::RepositoryError(infra_error(
                "get user id error",
                err,
            ))
        })?
        .ok_or(RegisterResetTokenRepositoryError::UserNotFound)?;

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

    repository.client.put_item(input).await.map_err(|err| {
        RegisterResetTokenRepositoryError::RepositoryError(infra_error(
            "put reset token error",
            err,
        ))
    })?;
    Ok(())
}

#[async_trait::async_trait]
impl<'client> ResetPasswordRepository for DynamoDbAuthUserRepository<'client> {
    async fn reset_token_entry(
        &self,
        reset_token: &ResetToken,
    ) -> Result<Option<ResetTokenEntry>, RepositoryError> {
        reset_token_entry(self, reset_token).await
    }
    async fn reset_password<'a>(
        &self,
        reset_token: &'a ResetToken,
        hasher: impl 'a + AuthUserPasswordHasher,
        reset_at: AuthDateTime,
    ) -> Result<AuthUserId, ResetPasswordRepositoryError> {
        reset_password(self, reset_token, hasher, reset_at).await
    }
}
async fn reset_token_entry<'client>(
    repository: &DynamoDbAuthUserRepository<'client>,
    reset_token: &ResetToken,
) -> Result<Option<ResetTokenEntry>, RepositoryError> {
    let mut key = AttributeMap::new();
    key.add_reset_token(reset_token.clone());

    let input = GetItemInput {
        table_name: repository.reset_token.into(),
        key: key.extract(),
        projection_expression: Some("login_id, email, expires, reset_at".into()),
        ..Default::default()
    };

    let response = repository
        .client
        .get_item(input)
        .await
        .map_err(|err| infra_error("get reset token error", err))?;

    let found = response
        .item
        .and_then(|mut attrs| {
            match (
                attrs.remove("login_id"),
                attrs.remove("email"),
                attrs.remove("expires"),
                attrs.remove("reset_at"),
            ) {
                (Some(login_id), Some(email), Some(expires), reset_at) => {
                    Some((login_id, email, expires, reset_at))
                }
                _ => None,
            }
        })
        .and_then(|(login_id, email, expires, reset_at)| {
            match (
                login_id.s,
                email.s,
                expires.n.and_then(|value| value.parse::<i64>().ok()),
                reset_at
                    .and_then(|attr| attr.n)
                    .and_then(|value| value.parse::<i64>().ok()),
            ) {
                (Some(login_id), Some(email), Some(expires), reset_at) => {
                    Some((login_id, email, expires, reset_at))
                }
                _ => None,
            }
        });

    Ok(found.map(|(login_id, email, expires, reset_at)| {
        ResetTokenEntryExtract {
            login_id,
            destination: ResetTokenDestinationExtract::Email(email),
            expires: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(expires, 0), Utc),
            reset_at: reset_at.map(|reset_at| {
                DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(reset_at, 0), Utc)
            }),
        }
        .restore()
    }))
}
async fn reset_password<'client, 'a>(
    repository: &DynamoDbAuthUserRepository<'client>,
    reset_token: &'a ResetToken,
    hasher: impl 'a + AuthUserPasswordHasher,
    reset_at: AuthDateTime,
) -> Result<AuthUserId, ResetPasswordRepositoryError> {
    // reset_token が正しいことが前提; reset_token_entry() で事前に確認する

    update_reset_at(repository, reset_token.clone(), reset_at)
        .await
        .map_err(|err| {
            ResetPasswordRepositoryError::RepositoryError(infra_error("update reset error", err))
        })?;

    let user_id = get_user_id_by_reset_token(repository, reset_token.clone())
        .await
        .map_err(|err| {
            ResetPasswordRepositoryError::RepositoryError(infra_error(
                "get user id by reset token error",
                err,
            ))
        })?
        .ok_or(ResetPasswordRepositoryError::ResetTokenNotFound)?;

    let password = hasher
        .hash_password()
        .map_err(ResetPasswordRepositoryError::PasswordHashError)?;

    update_password(repository, user_id.clone(), password)
        .await
        .map_err(|err| {
            ResetPasswordRepositoryError::RepositoryError(infra_error("reset password error", err))
        })?;

    Ok(user_id)
}

struct LoginIdEntry {
    user_id: AuthUserId,
    reset_token_destination_email: Option<String>,
}

async fn get_login_id_entry<'client>(
    repository: &DynamoDbAuthUserRepository<'client>,
    login_id: LoginId,
) -> Result<Option<LoginIdEntry>, RusotoError<GetItemError>> {
    let mut key = AttributeMap::new();
    key.add_login_id(login_id);

    let input = GetItemInput {
        table_name: repository.login_id.into(),
        key: key.extract(),
        projection_expression: Some("user_id, reset_token_destination_email".into()),
        ..Default::default()
    };

    let response = repository.client.get_item(input).await?;

    Ok(response.item.and_then(|mut attrs| {
        match (
            attrs.remove("user_id").and_then(|attr| attr.s),
            attrs
                .remove("reset_token_destination_email")
                .and_then(|attr| attr.s),
        ) {
            (Some(user_id), email) => Some(LoginIdEntry {
                user_id: AuthUserId::restore(user_id),
                reset_token_destination_email: email,
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
    entry: LoginIdEntry,
) -> Result<(), RusotoError<PutItemError>> {
    let mut item = AttributeMap::new();
    item.add_login_id_entry(new_login_id, entry);

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
async fn get_user_id_by_reset_token<'client>(
    repository: &DynamoDbAuthUserRepository<'client>,
    reset_token: ResetToken,
) -> Result<Option<AuthUserId>, RusotoError<GetItemError>> {
    let mut key = AttributeMap::new();
    key.add_reset_token(reset_token);

    let input = GetItemInput {
        table_name: repository.reset_token.into(),
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
        Some(roles) => GrantedAuthRoles::restore(roles),
        None => GrantedAuthRoles::restore(HashSet::new()),
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
async fn get_login_id<'client>(
    repository: &DynamoDbAuthUserRepository<'client>,
    user_id: AuthUserId,
) -> Result<Option<LoginId>, RusotoError<GetItemError>> {
    let mut key = AttributeMap::new();
    key.add_user_id(user_id);

    let input = GetItemInput {
        table_name: repository.user.into(),
        key: key.extract(),
        projection_expression: Some("login_id".into()),
        ..Default::default()
    };

    let response = repository.client.get_item(input).await?;

    Ok(response
        .item
        .and_then(|mut attrs| attrs.remove("login_id"))
        .and_then(|attr| attr.s)
        .map(|password| LoginId::restore(password)))
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
    fn add_login_id_entry(&mut self, login_id: LoginId, entry: LoginIdEntry) {
        self.add_login_id(login_id);
        self.add_user_id(entry.user_id);
        if let Some(email) = entry.reset_token_destination_email {
            self.add("reset_token_destination_email", string_value(email));
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
}

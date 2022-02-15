use std::{
    collections::{HashMap, HashSet},
    convert::TryInto,
    iter::FromIterator,
};

use chrono::{DateTime, NaiveDateTime, Utc};
use rusoto_core::RusotoError;
use rusoto_dynamodb::{
    AttributeValue, DynamoDb, DynamoDbClient, GetItemError, GetItemInput, PutItemInput, ScanError,
    ScanInput, UpdateItemError, UpdateItemInput,
};

use crate::auth::x_outside_feature::feature::AuthOutsideStore;

use crate::z_lib::repository::{
    dynamodb::helper::{string_value, timestamp_value, ScanKey},
    helper::infra_error,
};

use crate::auth::user::{
    account::search::infra::{SearchAuthUserAccountFields, SearchAuthUserAccountRepository},
    kernel::infra::AuthUserRepository,
    password::{
        authenticate::infra::VerifyPasswordRepository,
        change::infra::ChangePasswordRepository,
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
            account::search::data::{AuthUserAccountBasket, SearchAuthUserAccountBasket},
            kernel::data::{
                AuthUser, AuthUserExtract, AuthUserId, GrantedAuthRoles, GrantedAuthRolesBasket,
            },
            login_id::kernel::data::{LoginId, LoginIdBasket},
            password::{
                authenticate::data::VerifyPasswordRepositoryError,
                change::data::ChangePasswordRepositoryError,
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
        search::data::{SearchOffset, SearchPage, SearchSortOrderMap},
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
        .map(|email| ResetTokenDestinationExtract { email }.restore()))
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
            destination: ResetTokenDestinationExtract { email },
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
        fields: &SearchAuthUserAccountFields,
    ) -> Result<SearchAuthUserAccountBasket, RepositoryError> {
        search_user_account(&self, fields).await
    }
}

enum User {
    LoginId,
}
enum Order {
    Asc,
    Desc,
}
async fn search_user_account<'client>(
    repository: &DynamoDbAuthUserRepository<'client>,
    fields: &SearchAuthUserAccountFields,
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

    let (sort_col, sort_order) = fields
        .sort()
        .detect(vec![(
            "login-id",
            User::LoginId,
            SearchSortOrderMap {
                normal: Order::Asc,
                reverse: Order::Desc,
            },
        )])
        .unwrap_or((User::LoginId, Order::Asc));

    users.sort_by_cached_key(|user| match sort_col {
        User::LoginId => user.login_id.as_str().to_owned(),
    });
    match sort_order {
        Order::Asc => (),
        Order::Desc => users.reverse(),
    }

    let mut users: Vec<AuthUserAccountBasket> = users
        .into_iter()
        .filter(|user| {
            if fields.login_id() == "" {
                true
            } else {
                user.login_id.as_str() == fields.login_id()
            }
        })
        .collect();

    let limit = 1000;
    let offset = SearchOffset { all, limit }.detect(fields.offset());
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
        users,
    })
}

async fn scan_user<'client>(
    repository: &DynamoDbAuthUserRepository<'client>,
) -> Result<Vec<AuthUserAccountBasket>, RusotoError<ScanError>> {
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
) -> Result<(Vec<AuthUserAccountBasket>, ScanKey), RusotoError<ScanError>> {
    let input = ScanInput {
        table_name: repository.user.into(),
        exclusive_start_key: scan_key.extract(),
        projection_expression: Some("login_id, granted_roles".into()),
        ..Default::default()
    };

    let response = repository.client.scan(input).await?;

    let items = match response.items {
        None => vec![],
        Some(items) => items
            .into_iter()
            .filter_map(|mut item| {
                let login_id = item.remove("login_id").and_then(|attr| attr.n);
                let granted_roles = item
                    .remove("granted_roles")
                    .and_then(|attr| attr.ss)
                    .map(|roles| HashSet::from_iter(roles));
                match (login_id, granted_roles) {
                    (Some(login_id), Some(granted_roles)) => Some(AuthUserAccountBasket {
                        login_id: LoginIdBasket::new(login_id),
                        granted_roles: GrantedAuthRolesBasket::new(granted_roles),
                    }),
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
        self.add("login_id", string_value(login_id.extract()));
    }
    fn add_user_id(&mut self, user_id: AuthUserId) {
        self.add("user_id", string_value(user_id.extract()));
    }
    fn add_reset_token(&mut self, reset_token: ResetToken) {
        self.add("reset_token", string_value(reset_token.extract()));
    }
    fn add_destination(&mut self, destination: ResetTokenDestination) {
        self.add("email", string_value(destination.into_email()));
    }
    fn add_expires(&mut self, expires: ExpireDateTime) {
        self.add("expires", timestamp_value(expires.extract()));
    }
    fn add_requested_at(&mut self, requested_at: AuthDateTime) {
        self.add("requested_at", timestamp_value(requested_at.extract()));
    }

    fn add_password_as(&mut self, password: HashedPassword, name: &str) {
        self.add(name, string_value(password.extract()));
    }
    fn add_reset_at_as(&mut self, reset_at: AuthDateTime, name: &str) {
        self.add(name, timestamp_value(reset_at.extract()));
    }
}

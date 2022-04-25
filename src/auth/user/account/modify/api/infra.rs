use crate::{
    auth::user::{
        account::{
            kernel::data::{AuthUserAttributes, AuthUserAttributesExtract},
            modify::data::{
                ValidateModifyAuthUserAccountChangesError, ValidateModifyAuthUserAccountFieldsError,
            },
        },
        kernel::data::{AuthUserId, GrantedAuthRoles},
        login_id::kernel::data::LoginId,
    },
    z_lib::repository::data::RepositoryError,
};

pub trait ModifyAuthUserAccountRequestDecoder {
    fn decode(self) -> ModifyAuthUserAccountFieldsExtract;
}

pub struct ModifyAuthUserAccountFields {
    pub login_id: LoginId,
    pub from: ModifyAuthUserAccountChanges,
    pub to: ModifyAuthUserAccountChanges,
}

#[derive(PartialEq, Eq)]
pub struct ModifyAuthUserAccountChanges {
    pub granted_roles: GrantedAuthRoles,
    pub attrs: AuthUserAttributes,
}

pub struct ModifyAuthUserAccountFieldsExtract {
    pub login_id: String,
    pub from: Option<ModifyAuthUserAccountChangesExtract>,
    pub to: Option<ModifyAuthUserAccountChangesExtract>,
}

pub struct ModifyAuthUserAccountChangesExtract {
    pub granted_roles: Vec<String>,
    pub attrs: AuthUserAttributesExtract,
}

impl ModifyAuthUserAccountFields {
    pub fn convert(
        fields: ModifyAuthUserAccountFieldsExtract,
    ) -> Result<Self, ValidateModifyAuthUserAccountFieldsError> {
        Ok(Self {
            login_id: LoginId::convert(fields.login_id)
                .map_err(ValidateModifyAuthUserAccountFieldsError::InvalidLoginId)?,
            from: convert_changes(fields.from)
                .map_err(ValidateModifyAuthUserAccountFieldsError::InvalidFrom)?,
            to: convert_changes(fields.to)
                .map_err(ValidateModifyAuthUserAccountFieldsError::InvalidTo)?,
        })
    }
}

fn convert_changes(
    changes: Option<ModifyAuthUserAccountChangesExtract>,
) -> Result<ModifyAuthUserAccountChanges, ValidateModifyAuthUserAccountChangesError> {
    match changes {
        None => Err(ValidateModifyAuthUserAccountChangesError::NotFound),
        Some(data) => Ok(ModifyAuthUserAccountChanges {
            granted_roles: GrantedAuthRoles::convert(data.granted_roles)
                .map_err(ValidateModifyAuthUserAccountChangesError::InvalidGrantedRoles)?,
            attrs: AuthUserAttributes::convert(AuthUserAttributesExtract {
                memo: data.attrs.memo,
            })
            .map_err(ValidateModifyAuthUserAccountChangesError::InvalidAttrs)?,
        }),
    }
}

#[async_trait::async_trait]
pub trait ModifyAuthUserAccountRepository {
    async fn lookup_user_id(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<AuthUserId>, RepositoryError>;

    async fn lookup_changes(
        &self,
        user_id: &AuthUserId,
    ) -> Result<Option<ModifyAuthUserAccountChanges>, RepositoryError>;

    async fn modify_user(
        &self,
        user_id: AuthUserId,
        data: ModifyAuthUserAccountChanges,
    ) -> Result<(), RepositoryError>;
}

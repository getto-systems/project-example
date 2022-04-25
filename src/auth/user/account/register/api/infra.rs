use crate::{
    auth::user::{
        account::{
            kernel::data::{AuthUserAttributes, AuthUserAttributesExtract},
            register::data::ValidateRegisterAuthUserAccountFieldsError,
        },
        kernel::data::{AuthUserId, GrantedAuthRoles},
        login_id::kernel::data::LoginId,
        password::reset::kernel::data::{ResetTokenDestination, ResetTokenDestinationExtract},
    },
    z_lib::repository::data::RepositoryError,
};

pub trait RegisterAuthUserAccountRequestDecoder {
    fn decode(self) -> RegisterAuthUserAccountFieldsExtract;
}

pub struct RegisterAuthUserAccountFields {
    pub login_id: LoginId,
    pub granted_roles: GrantedAuthRoles,
    pub reset_token_destination: ResetTokenDestination,
    pub attrs: AuthUserAttributes,
}

pub struct RegisterAuthUserAccountFieldsExtract {
    pub login_id: String,
    pub granted_roles: Vec<String>,
    pub reset_token_destination: ResetTokenDestinationExtract,
    pub attrs: AuthUserAttributesExtract,
}

impl RegisterAuthUserAccountFields {
    pub fn convert(
        fields: RegisterAuthUserAccountFieldsExtract,
    ) -> Result<Self, ValidateRegisterAuthUserAccountFieldsError> {
        Ok(Self {
            login_id: LoginId::convert(fields.login_id)
                .map_err(ValidateRegisterAuthUserAccountFieldsError::InvalidLoginId)?,
            granted_roles: GrantedAuthRoles::convert(fields.granted_roles)
                .map_err(ValidateRegisterAuthUserAccountFieldsError::InvalidGrantedRoles)?,
            reset_token_destination: ResetTokenDestination::convert(fields.reset_token_destination)
                .map_err(
                    ValidateRegisterAuthUserAccountFieldsError::InvalidResetTokenDestination,
                )?,
            attrs: AuthUserAttributes::convert(fields.attrs)
                .map_err(ValidateRegisterAuthUserAccountFieldsError::InvalidAttrs)?,
        })
    }
}

pub trait AuthUserIdGenerator {
    fn generate(&self) -> AuthUserId;
}

#[async_trait::async_trait]
pub trait RegisterAuthUserAccountRepository {
    async fn check_login_id_registered(&self, login_id: &LoginId) -> Result<bool, RepositoryError>;

    async fn register_user(
        &self,
        user_id: AuthUserId,
        data: RegisterAuthUserAccountFields,
    ) -> Result<(), RepositoryError>;
}

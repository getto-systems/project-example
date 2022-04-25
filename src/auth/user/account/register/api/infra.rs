use crate::{
    auth::user::{
        account::{
            kernel::data::AuthUserAttributes,
            register::data::ValidateRegisterAuthUserAccountFieldsError,
        },
        kernel::data::{AuthUserId, GrantedAuthRoles},
        login_id::kernel::data::LoginId,
        password::reset::kernel::data::ResetTokenDestination,
    },
    z_lib::repository::data::RepositoryError,
};

// TODO Extract で受け取って validate は action でやる、だったはず
pub trait RegisterAuthUserAccountRequestDecoder {
    fn decode(
        self,
    ) -> Result<RegisterAuthUserAccountFields, ValidateRegisterAuthUserAccountFieldsError>;
}

pub struct RegisterAuthUserAccountFields {
    pub login_id: LoginId,
    pub granted_roles: GrantedAuthRoles,
    pub reset_token_destination: ResetTokenDestination,
    pub attrs: AuthUserAttributes,
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

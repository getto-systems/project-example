use crate::auth::user::{
    account::modify::data::{ModifyAuthUserAccountRepositoryError, ValidateAuthUserAccountError},
    kernel::data::GrantedAuthRoles,
    login_id::kernel::data::LoginId,
};

pub trait ModifyAuthUserAccountRequestDecoder {
    fn decode<F: ModifyAuthUserAccountFieldsExtract>(self) -> F;
}

pub struct ModifyAuthUserAccountFields {
    pub granted_roles: GrantedAuthRoles,
    pub reset_token_destination: ModifyResetTokenDestination,
}
pub enum ModifyResetTokenDestination {
    None,
    Email(ModifyResetTokenDestinationEmail),
}
pub struct ModifyResetTokenDestinationEmail(String);

pub trait ModifyAuthUserAccountFieldsExtract {
    fn validate() -> Result<ModifyAuthUserAccountFields, ValidateAuthUserAccountError>;
}

#[async_trait::async_trait]
pub trait ModifyAuthUserAccountRepository {
    async fn modify_user<'a>(
        &self,
        login_id: &'a LoginId,
        fields: ModifyAuthUserAccountFields,
    ) -> Result<(), ModifyAuthUserAccountRepositoryError>;
}

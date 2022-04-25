use crate::auth::user::{
    login_id::kernel::data::LoginId,
    password::{
        authenticate::{
            data::ValidateAuthenticatePasswordFieldsError,
            infra::{AuthenticatePasswordFields, AuthenticatePasswordFieldsExtract},
        },
        kernel::infra::PlainPassword,
    },
};

impl AuthenticatePasswordFields {
    pub fn validate(
        fields: AuthenticatePasswordFieldsExtract,
    ) -> Result<Self, ValidateAuthenticatePasswordFieldsError> {
        Ok(Self {
            login_id: LoginId::convert(fields.login_id)
                .map_err(ValidateAuthenticatePasswordFieldsError::InvalidLoginId)?,
            password: PlainPassword::validate(fields.password)
                .map_err(ValidateAuthenticatePasswordFieldsError::InvalidPassword)?,
        })
    }
}

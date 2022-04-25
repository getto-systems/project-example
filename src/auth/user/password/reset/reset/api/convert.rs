use crate::auth::user::password::{
    kernel::infra::PlainPassword,
    reset::reset::infra::{ResetPasswordFields, ResetPasswordFieldsExtract},
};

use crate::auth::user::{
    login_id::kernel::data::LoginId,
    password::reset::{
        kernel::data::ResetTokenEncoded, reset::data::ValidateResetPasswordFieldsError,
    },
};

impl ResetPasswordFields {
    pub fn convert(
        fields: ResetPasswordFieldsExtract,
    ) -> Result<Self, ValidateResetPasswordFieldsError> {
        Ok(Self {
            reset_token: ResetTokenEncoded::convert(fields.reset_token)
                .map_err(ValidateResetPasswordFieldsError::InvalidResetToken)?,
            login_id: LoginId::convert(fields.login_id)
                .map_err(ValidateResetPasswordFieldsError::InvalidLoginId)?,
            new_password: PlainPassword::convert(fields.new_password)
                .map_err(ValidateResetPasswordFieldsError::InvalidNewPassword)?,
        })
    }
}

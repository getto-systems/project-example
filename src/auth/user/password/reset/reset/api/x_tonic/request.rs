use crate::auth::user::password::reset::reset::y_protobuf::service::ResetPasswordRequestPb;

use crate::auth::user::password::{
    kernel::infra::PlainPassword,
    reset::reset::infra::{ResetPasswordFields, ResetPasswordFieldsExtract},
};

use crate::auth::user::{
    login_id::kernel::data::LoginId,
    password::reset::{
        kernel::data::ResetPasswordToken, reset::data::ValidateResetPasswordFieldsError,
    },
};

impl ResetPasswordFieldsExtract for ResetPasswordRequestPb {
    fn convert(self) -> Result<ResetPasswordFields, ValidateResetPasswordFieldsError> {
        Ok(ResetPasswordFields {
            reset_token: ResetPasswordToken::convert(self.reset_token)
                .map_err(ValidateResetPasswordFieldsError::InvalidResetToken)?,
            login_id: LoginId::convert(self.login_id)
                .map_err(ValidateResetPasswordFieldsError::InvalidLoginId)?,
            new_password: PlainPassword::convert(self.new_password)
                .map_err(ValidateResetPasswordFieldsError::InvalidNewPassword)?,
        })
    }
}

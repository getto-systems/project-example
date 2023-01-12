use crate::auth::user::password::change::y_protobuf::service::{
    ChangePasswordRequestPb, OverwritePasswordRequestPb,
};

use crate::auth::user::password::{
    change::infra::{
        ChangePasswordFields, ChangePasswordFieldsExtract, OverwritePasswordFields,
        OverwritePasswordFieldsExtract,
    },
    kernel::infra::PlainPassword,
};

use crate::auth::user::{
    login_id::kernel::data::LoginId,
    password::change::data::{
        ValidateChangePasswordFieldsError, ValidateOverwritePasswordFieldsError,
    },
};

impl ChangePasswordFieldsExtract for ChangePasswordRequestPb {
    fn convert(self) -> Result<ChangePasswordFields, ValidateChangePasswordFieldsError> {
        Ok(ChangePasswordFields {
            current_password: PlainPassword::convert(self.current_password)
                .map_err(ValidateChangePasswordFieldsError::InvalidCurrentPassword)?,
            new_password: PlainPassword::convert(self.new_password)
                .map_err(ValidateChangePasswordFieldsError::InvalidNewPassword)?,
        })
    }
}

impl OverwritePasswordFieldsExtract for OverwritePasswordRequestPb {
    fn convert(self) -> Result<OverwritePasswordFields, ValidateOverwritePasswordFieldsError> {
        Ok(OverwritePasswordFields {
            login_id: LoginId::convert(self.login_id)
                .map_err(ValidateOverwritePasswordFieldsError::InvalidLoginId)?,
            new_password: PlainPassword::convert(self.new_password)
                .map_err(ValidateOverwritePasswordFieldsError::InvalidNewPassword)?,
        })
    }
}

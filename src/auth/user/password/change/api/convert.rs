use crate::auth::user::password::change::infra::{
    ChangePasswordFields, ChangePasswordFieldsExtract, OverwritePasswordFields,
    OverwritePasswordFieldsExtract,
};

use crate::auth::user::{
    login_id::kernel::data::LoginId,
    password::{
        change::data::{ValidateChangePasswordFieldsError, ValidateOverwritePasswordFieldsError},
        kernel::infra::PlainPassword,
    },
};

impl ChangePasswordFields {
    pub fn convert(
        fields: ChangePasswordFieldsExtract,
    ) -> Result<Self, ValidateChangePasswordFieldsError> {
        Ok(Self {
            current_password: PlainPassword::convert(fields.current_password)
                .map_err(ValidateChangePasswordFieldsError::InvalidCurrentPassword)?,
            new_password: PlainPassword::convert(fields.new_password)
                .map_err(ValidateChangePasswordFieldsError::InvalidNewPassword)?,
        })
    }
}

impl OverwritePasswordFields {
    pub fn convert(
        fields: OverwritePasswordFieldsExtract,
    ) -> Result<Self, ValidateOverwritePasswordFieldsError> {
        Ok(Self {
            login_id: LoginId::convert(fields.login_id)
                .map_err(ValidateOverwritePasswordFieldsError::InvalidLoginId)?,
            new_password: PlainPassword::convert(fields.new_password)
                .map_err(ValidateOverwritePasswordFieldsError::InvalidNewPassword)?,
        })
    }
}

use crate::auth::user::password::change::infra::{
    ChangePasswordFields, ChangePasswordFieldsExtract, OverridePasswordFields,
    OverridePasswordFieldsExtract,
};

use crate::auth::user::{
    login_id::kernel::data::LoginId,
    password::{
        change::data::{ValidateChangePasswordFieldsError, ValidateOverridePasswordFieldsError},
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

impl OverridePasswordFields {
    pub fn convert(
        fields: OverridePasswordFieldsExtract,
    ) -> Result<Self, ValidateOverridePasswordFieldsError> {
        Ok(Self {
            login_id: LoginId::convert(fields.login_id)
                .map_err(ValidateOverridePasswordFieldsError::InvalidLoginId)?,
            new_password: PlainPassword::convert(fields.new_password)
                .map_err(ValidateOverridePasswordFieldsError::InvalidNewPassword)?,
        })
    }
}

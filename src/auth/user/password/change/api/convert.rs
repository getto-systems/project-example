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
    pub fn validate(
        fields: ChangePasswordFieldsExtract,
    ) -> Result<Self, ValidateChangePasswordFieldsError> {
        Ok(Self {
            current_password: PlainPassword::validate(fields.current_password)
                .map_err(ValidateChangePasswordFieldsError::InvalidCurrentPassword)?,
            new_password: PlainPassword::validate(fields.new_password)
                .map_err(ValidateChangePasswordFieldsError::InvalidNewPassword)?,
        })
    }
}

impl OverridePasswordFields {
    pub fn validate(
        fields: OverridePasswordFieldsExtract,
    ) -> Result<Self, ValidateOverridePasswordFieldsError> {
        Ok(Self {
            login_id: LoginId::validate(fields.login_id)
                .map_err(ValidateOverridePasswordFieldsError::InvalidLoginId)?,
            new_password: PlainPassword::validate(fields.new_password)
                .map_err(ValidateOverridePasswordFieldsError::InvalidPassword)?,
        })
    }
}

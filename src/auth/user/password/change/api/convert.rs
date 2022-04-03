use crate::auth::user::password::{
    change::infra::{ChangePasswordFields, ChangePasswordFieldsExtract},
    kernel::infra::PlainPassword,
};

use crate::auth::user::password::change::data::ValidateChangePasswordFieldsError;

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

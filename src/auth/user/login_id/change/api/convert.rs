use crate::auth::user::login_id::change::infra::{
    OverrideLoginIdFields, OverrideLoginIdFieldsExtract,
};

use crate::auth::user::login_id::{
    change::data::ValidateOverrideLoginIdFieldsError, kernel::data::LoginId,
};

impl OverrideLoginIdFields {
    pub fn validate(
        fields: OverrideLoginIdFieldsExtract,
    ) -> Result<OverrideLoginIdFields, ValidateOverrideLoginIdFieldsError> {
        Ok(Self {
            login_id: LoginId::convert(fields.login_id)
                .map_err(ValidateOverrideLoginIdFieldsError::InvalidCurrentLoginId)?,
            new_login_id: LoginId::convert(fields.new_login_id)
                .map_err(ValidateOverrideLoginIdFieldsError::InvalidNewLoginId)?,
        })
    }
}

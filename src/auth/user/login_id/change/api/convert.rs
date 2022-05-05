use crate::auth::user::login_id::change::infra::{
    OverwriteLoginIdFields, OverwriteLoginIdFieldsExtract,
};

use crate::auth::user::login_id::{
    change::data::ValidateOverwriteLoginIdFieldsError, kernel::data::LoginId,
};

impl OverwriteLoginIdFields {
    pub fn convert(
        fields: OverwriteLoginIdFieldsExtract,
    ) -> Result<OverwriteLoginIdFields, ValidateOverwriteLoginIdFieldsError> {
        Ok(Self {
            login_id: LoginId::convert(fields.login_id)
                .map_err(ValidateOverwriteLoginIdFieldsError::InvalidCurrentLoginId)?,
            new_login_id: LoginId::convert(fields.new_login_id)
                .map_err(ValidateOverwriteLoginIdFieldsError::InvalidNewLoginId)?,
        })
    }
}

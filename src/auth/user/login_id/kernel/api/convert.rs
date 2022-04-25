use crate::z_lib::validate::text::{check_text_empty, check_text_too_long};

use crate::{
    auth::user::login_id::kernel::data::{LoginIdExtract, ValidateLoginIdError},
    z_lib::validate::data::ValidateTextError,
};

impl LoginIdExtract for String {
    fn convert(self) -> Result<String, ValidateLoginIdError> {
        validate_login_id(&self).map_err(ValidateLoginIdError::Text)?;
        Ok(self)
    }
}

fn validate_login_id(value: &str) -> Result<(), ValidateTextError> {
    check_text_empty(value)?;
    check_text_too_long(value, 100)?; // ui の設定と同期させること
    Ok(())
}

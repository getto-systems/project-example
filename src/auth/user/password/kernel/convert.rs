use crate::z_lib::validate::data::ValidateTextError;
use crate::z_lib::validate::text::{check_text_empty, check_text_too_long};

use super::infra::PlainPasswordExtract;

use super::data::ValidatePasswordError;

impl PlainPasswordExtract for String {
    fn convert(self) -> Result<String, ValidatePasswordError> {
        validate_password(&self).map_err(ValidatePasswordError::Text)?;
        Ok(self)
    }
}

fn validate_password(value: &str) -> Result<(), ValidateTextError> {
    check_text_empty(value)?;
    check_text_too_long(value, 100)?; // ui の設定と同期させること
    Ok(())
}

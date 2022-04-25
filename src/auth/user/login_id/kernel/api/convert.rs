use crate::z_lib::validate::{text::{check_text_empty, check_text_too_long}, data::ValidateTextError};

use super::data::{LoginIdExtract, ValidateLoginIdError};

impl LoginIdExtract for String {
    fn convert(self) -> Result<String, ValidateLoginIdError> {
        validate_login_id(&self).map_err(ValidateLoginIdError::Text)?;
        Ok(self)
    }
}

fn validate_login_id(value: &str) -> Result<(), ValidateTextError> {
    check_text_empty(value)?;
    // login id には意味的な制限はないが、使用可能な最大文字数は定義しておく
    // ui の設定と同期させること
    check_text_too_long(value, 100)?;

    Ok(())
}

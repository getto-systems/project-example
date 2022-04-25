use crate::{
    auth::user::password::reset::kernel::data::{
        ResetTokenDestinationEmailExtract, ResetTokenEncodedExtract,
        ValidateResetTokenDestinationEmailError, ValidateResetTokenError,
    },
    z_lib::validate::{
        data::ValidateTextError,
        text::{check_text_empty, check_text_invalid_email, check_text_too_long},
    },
};

impl ResetTokenEncodedExtract for String {
    fn convert(self) -> Result<String, ValidateResetTokenError> {
        match self.chars().count() {
            n if n == 0 => Err(ValidateResetTokenError::Empty),
            _ => Ok(self),
        }
    }
}

impl ResetTokenDestinationEmailExtract for String {
    fn convert(self) -> Result<String, ValidateResetTokenDestinationEmailError> {
        validate_reset_token_destination_email(&self)
            .map_err(ValidateResetTokenDestinationEmailError::Text)?;
        Ok(self)
    }
}

fn validate_reset_token_destination_email(value: &str) -> Result<(), ValidateTextError> {
    check_text_empty(value)?;
    // email には意味的な制限はないが、使用可能な最大文字数は定義しておく
    // ui の設定と同期させること
    check_text_too_long(value, 255)?;
    check_text_invalid_email(value)?;
    Ok(())
}

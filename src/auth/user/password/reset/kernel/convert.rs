use crate::auth::user::password::reset::kernel::data::{
    ResetTokenDestinationEmailExtract, ResetTokenEncodedExtract,
    ValidateResetTokenDestinationEmailError, ValidateResetTokenError,
};

impl ResetTokenEncodedExtract for String {
    fn validate(self) -> Result<String, ValidateResetTokenError> {
        match self.chars().count() {
            n if n == 0 => Err(ValidateResetTokenError::Empty),
            _ => Ok(self),
        }
    }
}

// email には技術的な制限はないが、使用可能な最大文字数は定義しておく
// ui の設定と同期させること
const EMAIL_MAX_LENGTH: usize = 255;

impl ResetTokenDestinationEmailExtract for String {
    fn validate(self) -> Result<String, ValidateResetTokenDestinationEmailError> {
        if !self.contains("@") {
            return Err(ValidateResetTokenDestinationEmailError::Invalid);
        }
        match self.chars().count() {
            n if n == 0 => Err(ValidateResetTokenDestinationEmailError::Empty),
            n if n > EMAIL_MAX_LENGTH => Err(ValidateResetTokenDestinationEmailError::TooLong),
            _ => Ok(self),
        }
    }
}

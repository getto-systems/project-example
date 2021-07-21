use super::data::{LoginIdExtract, ValidateLoginIdError};

// login id には技術的な制限はないが、使用可能な最大文字数は定義しておく
// ui の設定と同期させること
const LOGIN_ID_MAX_LENGTH: usize = 100;

impl LoginIdExtract for String {
    fn validate(self) -> Result<String, ValidateLoginIdError> {
        match self.chars().count() {
            n if n == 0 => Err(ValidateLoginIdError::Empty),
            n if n > LOGIN_ID_MAX_LENGTH => Err(ValidateLoginIdError::TooLong),
            _ => Ok(self),
        }
    }
}

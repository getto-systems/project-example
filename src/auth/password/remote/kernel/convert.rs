use super::infra::PlainPasswordExtract;

use super::data::ValidatePasswordError;

// password には技術的な制限はないが、使用可能な最大文字数は定義しておく
// ui の設定と同期させること
const PASSWORD_MAX_LENGTH: usize = 100;

impl PlainPasswordExtract for String {
    fn validate(self) -> Result<String, ValidatePasswordError> {
        match self.chars().count() {
            n if n == 0 => Err(ValidatePasswordError::Empty),
            n if n > PASSWORD_MAX_LENGTH => Err(ValidatePasswordError::TooLong),
            _ => Ok(self),
        }
    }
}

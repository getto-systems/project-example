use super::data::ValidatePasswordError;

// password には技術的な制限はないが、使用可能な最大文字数は定義しておく
// ui の設定と同期させること
const PASSWORD_MAX_LENGTH: usize = 100;

pub fn validate_password(password: &str) -> Result<(), ValidatePasswordError> {
    match password.chars().count() {
        n if n == 0 => Err(ValidatePasswordError::Empty),
        n if n > PASSWORD_MAX_LENGTH => Err(ValidatePasswordError::TooLong),
        _ => Ok(()),
    }
}

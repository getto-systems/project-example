use super::data::ValidateLoginIdError;

// login id には技術的な制限はないが、使用可能な最大文字数は定義しておく
// ui の設定と同期させること
const LOGIN_ID_MAX_LENGTH: usize = 100;

pub fn validate_login_id(login_id: &str) -> Result<(), ValidateLoginIdError> {
    match login_id.chars().count() {
        n if n == 0 => Err(ValidateLoginIdError::Empty),
        n if n > LOGIN_ID_MAX_LENGTH => Err(ValidateLoginIdError::TooLong),
        _ => Ok(()),
    }
}

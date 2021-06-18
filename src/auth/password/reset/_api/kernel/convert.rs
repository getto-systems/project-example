use super::data::ValidateResetTokenError;

pub fn validate_reset_token(token: &str) -> Result<(), ValidateResetTokenError> {
    match token.chars().count() {
        n if n == 0 => Err(ValidateResetTokenError::Empty),
        _ => Ok(()),
    }
}

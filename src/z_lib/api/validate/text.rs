use crate::z_lib::validate::data::ValidateTextError;

pub fn check_text_empty(value: &str) -> Result<(), ValidateTextError> {
    if value.chars().count() == 0 {
        Err(ValidateTextError::Empty)
    } else {
        Ok(())
    }
}
pub fn check_text_too_long(value: &str, max_length: usize) -> Result<(), ValidateTextError> {
    if value.chars().count() > max_length {
        Err(ValidateTextError::TooLong)
    } else {
        Ok(())
    }
}
pub fn check_text_invalid_email(value: &str) -> Result<(), ValidateTextError> {
    if !value.contains("@") {
        Err(ValidateTextError::InvalidEmail)
    } else {
        Ok(())
    }
}

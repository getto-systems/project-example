use crate::auth::user::password::reset::remote::kernel::data::{
    ResetTokenEncodedExtract, ValidateResetTokenError,
};

impl ResetTokenEncodedExtract for String {
    fn validate(self) -> Result<String, ValidateResetTokenError> {
        match self.chars().count() {
            n if n == 0 => Err(ValidateResetTokenError::Empty),
            _ => Ok(self),
        }
    }
}

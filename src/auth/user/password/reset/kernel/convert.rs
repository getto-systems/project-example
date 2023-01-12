use crate::common::api::validate::text::{
    check_text_empty, check_text_invalid_email, check_text_too_long,
};

use crate::{
    auth::user::password::reset::kernel::data::{
        ResetPasswordTokenDestinationEmailExtract, ResetPasswordTokenExtract,
    },
    common::api::validate::data::ValidateTextError,
};

impl ResetPasswordTokenExtract for String {
    fn convert(self) -> Result<String, ValidateTextError> {
        check_text_empty(&self)?;
        Ok(self)
    }
}

impl ResetPasswordTokenDestinationEmailExtract for String {
    fn convert(self) -> Result<String, ValidateTextError> {
        check_text_empty(&self)?;
        check_text_too_long(&self, 255)?; // ui の設定と同期させること
        check_text_invalid_email(&self)?;
        Ok(self)
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use crate::auth::user::password::reset::kernel::data::{
        ResetPasswordToken, ResetPasswordTokenDestinationEmail,
        ValidateResetPasswordTokenDestinationError, ValidateResetPasswordTokenError,
    };

    #[test]
    fn success_convert_reset_password_token() -> Result<(), ValidateResetPasswordTokenError> {
        assert_eq!(
            format!(
                "{}",
                ResetPasswordToken::convert("reset-password-token".to_owned())?.extract()
            ),
            "reset-password-token",
        );
        Ok(())
    }

    #[test]
    fn success_convert_reset_password_token_destination_email(
    ) -> Result<(), ValidateResetPasswordTokenDestinationError> {
        assert_eq!(
            format!(
                "{}",
                ResetPasswordTokenDestinationEmail::convert("user@example.com".to_owned())?
            ),
            "email: user@example.com",
        );
        Ok(())
    }
}

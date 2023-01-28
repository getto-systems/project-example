use crate::auth::user::password::reset::kernel::y_protobuf::service::ResetTokenDestinationPb;

use crate::auth::user::password::reset::kernel::data::{
    ResetPasswordTokenDestination, ResetPasswordTokenDestinationEmail,
    ValidateResetPasswordTokenDestinationError,
};

impl TryFrom<Option<ResetTokenDestinationPb>> for ResetPasswordTokenDestination {
    type Error = ValidateResetPasswordTokenDestinationError;

    fn try_from(data: Option<ResetTokenDestinationPb>) -> Result<Self, Self::Error> {
        let data = data.ok_or(ValidateResetPasswordTokenDestinationError::NotFound)?;
        Ok(match data.r#type.as_str() {
            "email" => ResetPasswordTokenDestination::Email(
                ResetPasswordTokenDestinationEmail::convert(data.email)?,
            ),
            _ => Self::None,
        })
    }
}

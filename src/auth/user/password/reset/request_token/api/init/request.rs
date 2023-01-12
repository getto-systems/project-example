#[cfg(test)]
pub mod test {
    use crate::auth::user::password::reset::request_token::infra::{
        RequestResetPasswordTokenFields, RequestResetPasswordTokenFieldsExtract,
    };

    use crate::auth::user::login_id::kernel::data::ValidateLoginIdError;

    pub enum StaticRequestResetTokenFields {
        Valid(RequestResetPasswordTokenFields),
        Invalid(ValidateLoginIdError),
    }

    impl RequestResetPasswordTokenFieldsExtract for StaticRequestResetTokenFields {
        fn convert(self) -> Result<RequestResetPasswordTokenFields, ValidateLoginIdError> {
            match self {
                Self::Valid(fields) => Ok(fields),
                Self::Invalid(err) => Err(err),
            }
        }
    }
}

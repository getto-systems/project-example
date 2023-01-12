#[cfg(test)]
pub mod test {
    use crate::auth::user::password::reset::reset::infra::{
        ResetPasswordFields, ResetPasswordFieldsExtract,
    };

    use crate::auth::user::password::reset::reset::data::ValidateResetPasswordFieldsError;

    pub enum StaticResetPasswordFields {
        Valid(ResetPasswordFields),
        Invalid(ValidateResetPasswordFieldsError),
    }

    impl ResetPasswordFieldsExtract for StaticResetPasswordFields {
        fn convert(self) -> Result<ResetPasswordFields, ValidateResetPasswordFieldsError> {
            match self {
                Self::Valid(fields) => Ok(fields),
                Self::Invalid(err) => Err(err),
            }
        }
    }
}

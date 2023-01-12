#[cfg(test)]
pub mod test {
    use crate::auth::user::password::authenticate::infra::{
        AuthenticateWithPasswordFields, AuthenticateWithPasswordFieldsExtract,
    };

    use crate::auth::user::password::authenticate::data::ValidateAuthenticateWithPasswordFieldsError;

    pub enum StaticAuthenticateWithPasswordFields {
        Valid(AuthenticateWithPasswordFields),
        Invalid(ValidateAuthenticateWithPasswordFieldsError),
    }

    impl AuthenticateWithPasswordFieldsExtract for StaticAuthenticateWithPasswordFields {
        fn convert(
            self,
        ) -> Result<AuthenticateWithPasswordFields, ValidateAuthenticateWithPasswordFieldsError>
        {
            match self {
                Self::Valid(fields) => Ok(fields),
                Self::Invalid(err) => Err(err),
            }
        }
    }
}

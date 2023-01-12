#[cfg(test)]
pub mod test {
    use crate::auth::ticket::authorize::infra::{AuthorizeFields, AuthorizeFieldsExtract};

    use crate::auth::ticket::authorize::data::ValidateAuthorizeFieldsError;

    pub enum StaticAuthorizeFields {
        Valid(AuthorizeFields),
        Invalid(ValidateAuthorizeFieldsError),
    }

    impl AuthorizeFieldsExtract for StaticAuthorizeFields {
        fn convert(self) -> Result<AuthorizeFields, ValidateAuthorizeFieldsError> {
            match self {
                Self::Valid(fields) => Ok(fields),
                Self::Invalid(err) => Err(err),
            }
        }
    }
}

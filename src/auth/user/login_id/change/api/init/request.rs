#[cfg(test)]
pub mod test {
    use crate::auth::user::login_id::change::infra::{
        OverwriteLoginIdFields, OverwriteLoginIdFieldsExtract,
    };

    use crate::auth::user::login_id::change::data::ValidateOverwriteLoginIdFieldsError;

    pub enum StaticOverwriteLoginIdFields {
        Valid(OverwriteLoginIdFields),
        Invalid(ValidateOverwriteLoginIdFieldsError),
    }

    impl OverwriteLoginIdFieldsExtract for StaticOverwriteLoginIdFields {
        fn convert(self) -> Result<OverwriteLoginIdFields, ValidateOverwriteLoginIdFieldsError> {
            match self {
                Self::Valid(fields) => Ok(fields),
                Self::Invalid(err) => Err(err),
            }
        }
    }
}

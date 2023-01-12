use crate::common::api::validate::text::{check_text_empty, check_text_too_long};

use super::infra::PlainPasswordExtract;

use crate::common::api::validate::data::ValidateTextError;

impl PlainPasswordExtract for String {
    fn convert(self) -> Result<String, ValidateTextError> {
        check_text_empty(&self)?;
        check_text_too_long(&self, 100)?; // ui の設定と同期させること
        Ok(self)
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use crate::auth::user::password::kernel::{data::ValidatePasswordError, infra::PlainPassword};

    #[test]
    fn success_convert_password() -> Result<(), ValidatePasswordError> {
        assert_eq!(
            format!(
                "{}",
                PlainPassword::convert("my-password".to_owned())?.extract()
            ),
            "my-password",
        );
        Ok(())
    }
}

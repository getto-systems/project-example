use crate::common::api::validate::text::{check_text_empty, check_text_too_long};

use crate::{
    auth::user::login_id::kernel::data::LoginIdExtract,
    common::api::validate::data::ValidateTextError,
};

impl LoginIdExtract for String {
    fn convert(self) -> Result<String, ValidateTextError> {
        check_text_empty(&self)?;
        check_text_too_long(&self, 100)?; // ui の設定と同期させること
        Ok(self)
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use crate::auth::user::login_id::kernel::data::{LoginId, ValidateLoginIdError};

    #[test]
    fn success_convert_login_id() -> Result<(), ValidateLoginIdError> {
        assert_eq!(
            format!("{}", LoginId::convert("my-login-id".to_owned())?),
            "login-id: my-login-id",
        );
        Ok(())
    }
}

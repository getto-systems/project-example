use crate::common::api::validate::text::check_text_too_long;

use crate::{
    auth::user::account::kernel::data::AuthUserMemoExtract,
    common::api::validate::data::ValidateTextError,
};

impl AuthUserMemoExtract for String {
    fn convert(self) -> Result<Self, ValidateTextError> {
        check_text_too_long(&self, 255)?; // ui の設定と同期させること
        Ok(self)
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use crate::auth::user::account::kernel::data::{AuthUserMemo, ValidateAuthUserAccountError};

    #[test]
    fn success_convert_memo() -> Result<(), ValidateAuthUserAccountError> {
        assert_eq!(
            format!("{}", AuthUserMemo::convert("text".to_owned())?),
            "memo: text",
        );
        Ok(())
    }
}

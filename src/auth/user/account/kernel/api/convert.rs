use crate::z_lib::validate::text::check_text_too_long;

use crate::{
    auth::user::account::kernel::data::{
        AuthUserAttributesExtract, ValidateAuthUserAttributesError,
    },
    z_lib::validate::data::ValidateTextError,
};

impl AuthUserAttributesExtract {
    pub fn convert(self) -> Result<Self, ValidateAuthUserAttributesError> {
        validate_memo(&self.memo).map_err(ValidateAuthUserAttributesError::Memo)?;
        Ok(self)
    }
}

fn validate_memo(value: &str) -> Result<(), ValidateTextError> {
    check_text_too_long(value, 255)?; // ui の設定と同期させること
    Ok(())
}

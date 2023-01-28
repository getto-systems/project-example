use crate::auth::user::account::y_protobuf::service::AuthUserAccountPb;

use crate::auth::{
    ticket::kernel::data::AuthPermissionGranted,
    user::{
        account::kernel::data::{
            AuthUserAccount, AuthUserAccountAttrs, AuthUserMemo, ValidateAuthUserAccountError,
        },
        login_id::kernel::data::LoginId,
    },
};

impl TryFrom<Option<AuthUserAccountPb>> for AuthUserAccountAttrs {
    type Error = ValidateAuthUserAccountError;

    fn try_from(data: Option<AuthUserAccountPb>) -> Result<Self, Self::Error> {
        let data = data.ok_or(ValidateAuthUserAccountError::NotFound)?;
        Ok(Self {
            granted: AuthPermissionGranted::convert(data.granted)
                .map_err(ValidateAuthUserAccountError::Granted)?,
            memo: AuthUserMemo::convert(data.memo)?,
        })
    }
}

impl TryFrom<Option<AuthUserAccountPb>> for AuthUserAccount {
    type Error = ValidateAuthUserAccountError;

    fn try_from(data: Option<AuthUserAccountPb>) -> Result<Self, Self::Error> {
        let data = data.ok_or(ValidateAuthUserAccountError::NotFound)?;
        Ok(Self {
            login_id: LoginId::convert(data.login_id)
                .map_err(ValidateAuthUserAccountError::LoginId)?,
            attrs: AuthUserAccountAttrs {
                granted: AuthPermissionGranted::convert(data.granted)
                    .map_err(ValidateAuthUserAccountError::Granted)?,
                memo: AuthUserMemo::convert(data.memo)?,
            },
            reset_token_destination: data
                .reset_token_destination
                .try_into()
                .map_err(ValidateAuthUserAccountError::ResetTokenDestination)?,
        })
    }
}

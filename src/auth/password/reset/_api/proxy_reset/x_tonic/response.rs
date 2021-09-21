use crate::auth::password::reset::{
    _api::y_protobuf::api::ResetPasswordErrorKindPb,
    _common::y_protobuf::service::ResetPasswordResponsePb,
};

use crate::auth::password::reset::_api::proxy_reset::infra::ResetPasswordProxyResponse;

use crate::auth::{
    auth_ticket::_common::{encode::data::AuthTicketEncoded, kernel::data::AuthTokenEncoded},
    auth_user::_common::kernel::data::AuthUserExtract,
};

impl Into<Option<ResetPasswordProxyResponse>> for ResetPasswordResponsePb {
    fn into(self) -> Option<ResetPasswordProxyResponse> {
        if self.success {
            match (self.user, self.token) {
                (Some(user), Some(token)) => {
                    let user: AuthUserExtract = user.into();
                    let token: Option<AuthTokenEncoded> = token.into();
                    token.map(|token| {
                        ResetPasswordProxyResponse::Success(AuthTicketEncoded { user, token })
                    })
                }
                _ => None,
            }
        } else {
            match ResetPasswordErrorKindPb::from_i32(self.error) {
                Some(ResetPasswordErrorKindPb::AlreadyReset) => {
                    Some(ResetPasswordProxyResponse::AlreadyReset)
                }
                Some(ResetPasswordErrorKindPb::InvalidReset) => {
                    Some(ResetPasswordProxyResponse::InvalidReset)
                }
                _ => Some(ResetPasswordProxyResponse::InvalidReset),
            }
        }
    }
}

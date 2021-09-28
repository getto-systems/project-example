use crate::auth::user::password::reset::remote::y_protobuf::{
    api::ResetPasswordErrorKindPb, service::ResetPasswordResponsePb,
};

use crate::auth::user::password::reset::remote::proxy_reset::infra::ResetPasswordProxyResponse;

use crate::auth::{
    ticket::remote::{encode::data::AuthTicketEncoded, kernel::data::AuthTokenEncoded},
    user::remote::kernel::data::AuthUserExtract,
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

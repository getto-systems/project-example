use crate::auth::{
    auth_ticket::_common::{
        encode::data::EncodeAuthTicketResponse, kernel::data::AuthTokenEncoded,
    },
    auth_user::_common::kernel::data::AuthUserExtract,
    password::reset::{
        _api::reset::infra::ResetPasswordResponse,
        _common::y_protobuf::service::{ResetPasswordErrorKindPb, ResetPasswordResponsePb},
    },
};

impl Into<Option<ResetPasswordResponse>> for ResetPasswordResponsePb {
    fn into(self) -> Option<ResetPasswordResponse> {
        if self.success {
            match (self.user, self.token) {
                (Some(user), Some(token)) => {
                    let user: AuthUserExtract = user.into();
                    let token: Option<AuthTokenEncoded> = token.into();
                    token.map(|token| {
                        ResetPasswordResponse::Success(EncodeAuthTicketResponse::new(
                            user.into(),
                            token,
                        ))
                    })
                }
                _ => None,
            }
        } else {
            match ResetPasswordErrorKindPb::from_i32(self.error) {
                Some(ResetPasswordErrorKindPb::AlreadyReset) => {
                    Some(ResetPasswordResponse::AlreadyReset)
                }
                Some(ResetPasswordErrorKindPb::InvalidReset) => {
                    Some(ResetPasswordResponse::InvalidReset)
                }
                _ => Some(ResetPasswordResponse::InvalidReset),
            }
        }
    }
}

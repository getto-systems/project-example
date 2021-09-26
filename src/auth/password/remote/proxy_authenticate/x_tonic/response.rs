use crate::auth::password::_common::y_protobuf::service::AuthenticatePasswordResponsePb;

use crate::auth::password::remote::proxy_authenticate::infra::AuthenticatePasswordProxyResponse;

use crate::auth::{
    auth_ticket::_common::{encode::data::AuthTicketEncoded, kernel::data::AuthTokenEncoded},
    auth_user::_common::kernel::data::AuthUserExtract,
};

impl Into<Option<AuthenticatePasswordProxyResponse>> for AuthenticatePasswordResponsePb {
    fn into(self) -> Option<AuthenticatePasswordProxyResponse> {
        if self.success {
            match (self.user, self.token) {
                (Some(user), Some(token)) => {
                    let user: AuthUserExtract = user.into();
                    let token: Option<AuthTokenEncoded> = token.into();
                    token.map(|token| {
                        AuthenticatePasswordProxyResponse::Success(AuthTicketEncoded {
                            user,
                            token,
                        })
                    })
                }
                _ => None,
            }
        } else {
            Some(AuthenticatePasswordProxyResponse::InvalidPassword)
        }
    }
}

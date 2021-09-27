use crate::auth::password::_common::y_protobuf::service::AuthenticatePasswordResponsePb;

use crate::auth::password::remote::proxy_authenticate::infra::AuthenticatePasswordProxyResponse;

use crate::auth::{
    auth_ticket::{
        _common::kernel::data::AuthTokenEncoded, remote::encode::data::AuthTicketEncoded,
    },
    auth_user::remote::kernel::data::AuthUserExtract,
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

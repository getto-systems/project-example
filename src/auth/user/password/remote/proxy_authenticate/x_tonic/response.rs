use crate::auth::user::password::remote::y_protobuf::service::AuthenticatePasswordResponsePb;

use crate::auth::user::password::remote::proxy_authenticate::infra::AuthenticatePasswordProxyResponse;

use crate::auth::{
    ticket::remote::{encode::data::AuthTicketEncoded, kernel::data::AuthTokenEncoded},
    user::remote::kernel::data::AuthUserExtract,
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

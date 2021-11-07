use crate::auth::user::password::remote::y_protobuf::service::ChangePasswordResponsePb;

use crate::auth::user::password::remote::proxy_change::infra::ChangePasswordProxyResponse;

impl Into<Option<ChangePasswordProxyResponse>> for ChangePasswordResponsePb {
    fn into(self) -> Option<ChangePasswordProxyResponse> {
        if self.success {
            Some(ChangePasswordProxyResponse::Success)
        } else {
            Some(ChangePasswordProxyResponse::InvalidPassword)
        }
    }
}

use crate::auth::password::_common::y_protobuf::service::ChangePasswordResponsePb;

use crate::auth::password::_api::proxy_change::infra::ChangePasswordProxyResponse;

impl Into<Option<ChangePasswordProxyResponse>> for ChangePasswordResponsePb {
    fn into(self) -> Option<ChangePasswordProxyResponse> {
        if self.success {
            Some(ChangePasswordProxyResponse::Success)
        } else {
            Some(ChangePasswordProxyResponse::InvalidPassword)
        }
    }
}

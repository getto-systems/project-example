use crate::auth::password::reset::_common::y_protobuf::service::RequestResetTokenResponsePb;

use crate::auth::password::reset::remote::proxy_request_token::infra::RequestResetTokenProxyResponse;

impl Into<Option<RequestResetTokenProxyResponse>> for RequestResetTokenResponsePb {
    fn into(self) -> Option<RequestResetTokenProxyResponse> {
        if self.success {
            Some(RequestResetTokenProxyResponse::Success)
        } else {
            Some(RequestResetTokenProxyResponse::InvalidRequest)
        }
    }
}

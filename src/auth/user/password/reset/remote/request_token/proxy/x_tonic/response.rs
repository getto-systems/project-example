use crate::auth::user::password::reset::remote::y_protobuf::service::RequestResetTokenResponsePb;

use crate::auth::user::password::reset::remote::request_token::proxy::infra::RequestResetTokenProxyResponse;

impl Into<Option<RequestResetTokenProxyResponse>> for RequestResetTokenResponsePb {
    fn into(self) -> Option<RequestResetTokenProxyResponse> {
        if self.success {
            Some(RequestResetTokenProxyResponse::Success)
        } else {
            Some(RequestResetTokenProxyResponse::InvalidRequest)
        }
    }
}

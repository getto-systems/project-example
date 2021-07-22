use crate::auth::password::reset::_common::y_protobuf::service::RequestResetTokenResponsePb;

use crate::auth::password::reset::_api::request_token::infra::RequestResetTokenResponse;

impl Into<Option<RequestResetTokenResponse>> for RequestResetTokenResponsePb {
    fn into(self) -> Option<RequestResetTokenResponse> {
        if self.success {
            Some(RequestResetTokenResponse::Success)
        } else {
            Some(RequestResetTokenResponse::InvalidRequest)
        }
    }
}

use crate::auth::password::_common::y_protobuf::service::ChangePasswordResponsePb;

use crate::auth::password::_api::change::infra::ChangePasswordResponse;

impl Into<Option<ChangePasswordResponse>> for ChangePasswordResponsePb {
    fn into(self) -> Option<ChangePasswordResponse> {
        if self.success {
            Some(ChangePasswordResponse::Success)
        } else {
            Some(ChangePasswordResponse::InvalidPassword)
        }
    }
}

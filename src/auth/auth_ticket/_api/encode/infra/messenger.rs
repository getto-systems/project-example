use crate::auth::_api::y_protobuf::api::{AuthenticatePasswordResult_pb, AuthenticateResponse_pb};

use super::EncodeMessenger;

use crate::auth::auth_user::_api::kernel::data::GrantedAuthRoles;
use crate::z_details::_api::message::{data::MessageError, helper::encode_protobuf_base64};

pub struct EncodeRenewMessenger;

impl EncodeRenewMessenger {
    pub const fn new() -> Self {
        Self
    }
}

impl EncodeMessenger for EncodeRenewMessenger {
    fn encode(&self, granted_roles: GrantedAuthRoles) -> Result<String, MessageError> {
        let mut message = AuthenticateResponse_pb::new();

        let roles = message.mut_roles();
        granted_roles
            .extract()
            .into_iter()
            .for_each(|role| roles.push(role));

        encode_protobuf_base64(message)
    }
}

pub struct EncodeAuthenticatePasswordMessenger;

impl EncodeAuthenticatePasswordMessenger {
    pub const fn new() -> Self {
        Self
    }
}

impl EncodeMessenger for EncodeAuthenticatePasswordMessenger {
    fn encode(&self, granted_roles: GrantedAuthRoles) -> Result<String, MessageError> {
        let mut message = AuthenticatePasswordResult_pb::new();

        let mut user = AuthenticateResponse_pb::new();

        let roles = user.mut_roles();
        granted_roles
            .extract()
            .into_iter()
            .for_each(|role| roles.push(role));

        message.set_success(true);
        message.set_value(user);

        encode_protobuf_base64(message)
    }
}

use crate::auth::_api::y_protobuf::api::{
    AuthenticatePasswordResult_pb, AuthenticateResponse_pb, ResetPasswordResult_pb,
};

use crate::z_details::_api::message::helper::encode_protobuf_base64;

use super::EncodeMessenger;

use crate::auth::auth_user::_api::kernel::data::GrantedAuthRoles;
use crate::z_details::_api::message::data::MessageError;

pub struct RenewEncodeMessenger;

impl RenewEncodeMessenger {
    pub const fn new() -> Self {
        Self
    }
}

impl EncodeMessenger for RenewEncodeMessenger {
    fn encode(&self, granted_roles: GrantedAuthRoles) -> Result<String, MessageError> {
        let message = authenticate_response(granted_roles);
        encode_protobuf_base64(message)
    }
}

pub struct AuthenticatePasswordEncodeMessenger;

impl AuthenticatePasswordEncodeMessenger {
    pub const fn new() -> Self {
        Self
    }
}

impl EncodeMessenger for AuthenticatePasswordEncodeMessenger {
    fn encode(&self, granted_roles: GrantedAuthRoles) -> Result<String, MessageError> {
        let mut message = AuthenticatePasswordResult_pb::new();
        message.set_success(true);
        message.set_value(authenticate_response(granted_roles));
        encode_protobuf_base64(message)
    }
}

pub struct ResetPasswordEncodeMessenger;

impl ResetPasswordEncodeMessenger {
    pub const fn new() -> Self {
        Self
    }
}

impl EncodeMessenger for ResetPasswordEncodeMessenger {
    fn encode(&self, granted_roles: GrantedAuthRoles) -> Result<String, MessageError> {
        let mut message = ResetPasswordResult_pb::new();
        message.set_success(true);
        message.set_value(authenticate_response(granted_roles));
        encode_protobuf_base64(message)
    }
}

fn authenticate_response(granted_roles: GrantedAuthRoles) -> AuthenticateResponse_pb {
    let mut response = AuthenticateResponse_pb::new();

    let roles = response.mut_roles();
    granted_roles
        .extract()
        .into_iter()
        .for_each(|role| roles.push(role));

    response
}

#[cfg(test)]
pub mod test {
    use super::super::EncodeMessenger;

    use crate::auth::auth_user::_api::kernel::data::GrantedAuthRoles;
    use crate::z_details::_api::message::data::MessageError;

    pub struct StaticEncodeMessenger;

    impl StaticEncodeMessenger {
        pub const fn new() -> Self {
            Self
        }
    }

    impl EncodeMessenger for StaticEncodeMessenger {
        fn encode(&self, _granted_roles: GrantedAuthRoles) -> Result<String, MessageError> {
            Ok("encoded".into())
        }
    }
}

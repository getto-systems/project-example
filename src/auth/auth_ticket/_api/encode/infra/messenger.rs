use crate::auth::_api::y_protobuf::api::{AuthenticatePasswordResult_pb, AuthenticateResponse_pb};

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
        let mut message = AuthenticateResponse_pb::new();

        let roles = message.mut_roles();
        granted_roles
            .extract()
            .into_iter()
            .for_each(|role| roles.push(role));

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

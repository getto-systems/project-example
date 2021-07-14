use std::iter::FromIterator;

use crate::auth::{
    auth_ticket::_api::y_protobuf::api::AuthenticateResponsePb,
    password::{
        _api::y_protobuf::api::AuthenticatePasswordResultPb,
        reset::_api::y_protobuf::api::ResetPasswordResultPb,
    },
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
        encode_protobuf_base64(authenticate_response(granted_roles))
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
        encode_protobuf_base64(AuthenticatePasswordResultPb {
            success: true,
            value: Some(authenticate_response(granted_roles)),
            ..Default::default()
        })
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
        encode_protobuf_base64(ResetPasswordResultPb {
            success: true,
            value: Some(authenticate_response(granted_roles)),
            ..Default::default()
        })
    }
}

fn authenticate_response(granted_roles: GrantedAuthRoles) -> AuthenticateResponsePb {
    AuthenticateResponsePb {
        roles: Vec::from_iter(granted_roles.extract().into_iter()),
        ..Default::default()
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

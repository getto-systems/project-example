use std::iter::FromIterator;

use crate::auth::{
    auth_ticket::_api::y_protobuf::api::AuthenticateResponsePb,
    auth_user::_common::kernel::data::AuthUserExtract,
};

impl Into<AuthenticateResponsePb> for AuthUserExtract {
    fn into(self) -> AuthenticateResponsePb {
        AuthenticateResponsePb {
            roles: Vec::from_iter(self.granted_roles.into_iter()),
        }
    }
}

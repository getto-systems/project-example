use std::iter::FromIterator;

use crate::auth::ticket::_api::y_protobuf::api::AuthenticateResponsePb;

use crate::auth::user::remote::kernel::data::AuthUserExtract;

impl Into<AuthenticateResponsePb> for AuthUserExtract {
    fn into(self) -> AuthenticateResponsePb {
        AuthenticateResponsePb {
            roles: Vec::from_iter(self.granted_roles.into_iter()),
        }
    }
}

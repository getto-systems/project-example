use std::iter::FromIterator;

use crate::auth::ticket::remote::y_protobuf::api::AuthenticateApiResponsePb;

use crate::auth::user::remote::kernel::data::AuthUserExtract;

impl Into<AuthenticateApiResponsePb> for AuthUserExtract {
    fn into(self) -> AuthenticateApiResponsePb {
        AuthenticateApiResponsePb {
            roles: Vec::from_iter(self.granted_roles.into_iter()),
        }
    }
}

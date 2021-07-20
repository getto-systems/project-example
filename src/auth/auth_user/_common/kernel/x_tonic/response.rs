use std::{collections::HashSet, iter::FromIterator};

use crate::auth::auth_user::_common::y_protobuf::service::AuthUserPb;

use crate::auth::auth_user::_common::kernel::data::AuthUserExtract;

impl Into<AuthUserPb> for AuthUserExtract {
    fn into(self) -> AuthUserPb {
        AuthUserPb {
            user_id: self.user_id,
            granted_roles: Vec::from_iter(self.granted_roles.into_iter()),
        }
    }
}

impl Into<AuthUserExtract> for AuthUserPb {
    fn into(self) -> AuthUserExtract {
        AuthUserExtract {
            user_id: self.user_id,
            granted_roles: HashSet::from_iter(self.granted_roles.into_iter()),
        }
    }
}

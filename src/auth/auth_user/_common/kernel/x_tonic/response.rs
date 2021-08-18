use std::{collections::HashSet, iter::FromIterator};

use tonic::{Response, Status};

use crate::auth::auth_ticket::_common::y_protobuf::service::ValidateApiTokenRequestPb;
use crate::auth::{
    auth_ticket::_common::y_protobuf::service::ValidateApiTokenResponsePb,
    auth_user::_common::y_protobuf::service::AuthUserPb,
};

use crate::auth::auth_user::_common::kernel::data::{AuthUser, AuthUserExtract, RequireAuthRoles};
use crate::z_details::_common::response::tonic::RespondTo;

impl RespondTo<ValidateApiTokenResponsePb> for AuthUser {
    fn respond_to(self) -> Result<Response<ValidateApiTokenResponsePb>, Status> {
        Ok(Response::new(ValidateApiTokenResponsePb {
            user: Some(self.extract().into()),
        }))
    }
}

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

impl Into<ValidateApiTokenRequestPb> for RequireAuthRoles {
    fn into(self) -> ValidateApiTokenRequestPb {
        match self {
            Self::Nothing => ValidateApiTokenRequestPb {
                allow_any_role: true,
                ..Default::default()
            },
            Self::HasAny(require_roles) => ValidateApiTokenRequestPb {
                allow_any_role: false,
                require_roles: Vec::from_iter(require_roles.extract().into_iter()),
            },
        }
    }
}

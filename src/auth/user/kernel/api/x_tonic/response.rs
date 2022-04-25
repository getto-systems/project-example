use std::{collections::HashSet, iter::FromIterator};

use tonic::{Response, Status};

use crate::z_lib::response::tonic::ServiceResponder;

use crate::auth::{
    ticket::validate::y_protobuf::service::{AuthorizeRequestPb, AuthorizeResponsePb},
    user::y_protobuf::service::GrantedAuthRolesPb,
};

use crate::auth::user::kernel::data::{AuthUser, GrantedAuthRoles, RequireAuthRoles};

impl Into<GrantedAuthRolesPb> for GrantedAuthRoles {
    fn into(self) -> GrantedAuthRolesPb {
        GrantedAuthRolesPb {
            granted_roles: Vec::from_iter(self.extract().into_iter()),
        }
    }
}

impl Into<GrantedAuthRoles> for GrantedAuthRolesPb {
    fn into(self) -> GrantedAuthRoles {
        GrantedAuthRoles::restore(HashSet::from_iter(self.granted_roles.into_iter()))
    }
}

impl ServiceResponder<AuthorizeResponsePb> for AuthUser {
    fn respond_to(self) -> Result<Response<AuthorizeResponsePb>, Status> {
        Ok(Response::new(AuthorizeResponsePb {}))
    }
}

impl Into<AuthorizeRequestPb> for RequireAuthRoles {
    fn into(self) -> AuthorizeRequestPb {
        match self {
            Self::Nothing => AuthorizeRequestPb {
                allow_any_role: true,
                ..Default::default()
            },
            Self::HasAny(require_roles) => AuthorizeRequestPb {
                allow_any_role: false,
                require_roles: Vec::from_iter(
                    require_roles
                        .into_iter()
                        .map(|role| role.as_str().to_owned()),
                ),
            },
        }
    }
}

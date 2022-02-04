use std::{collections::HashSet, iter::FromIterator};

use tonic::{Response, Status};

use crate::z_lib::remote::response::tonic::RespondTo;

use crate::auth::{
    ticket::y_protobuf::service::{ValidateApiTokenRequestPb, ValidateApiTokenResponsePb},
    user::y_protobuf::service::GrantedAuthRolesPb,
};

use crate::auth::user::kernel::data::{AuthUser, GrantedAuthRoles, RequireAuthRoles};

impl RespondTo<ValidateApiTokenResponsePb> for AuthUser {
    fn respond_to(self) -> Result<Response<ValidateApiTokenResponsePb>, Status> {
        Ok(Response::new(ValidateApiTokenResponsePb {}))
    }
}

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

impl Into<ValidateApiTokenRequestPb> for RequireAuthRoles {
    fn into(self) -> ValidateApiTokenRequestPb {
        match self {
            Self::Nothing => ValidateApiTokenRequestPb {
                allow_any_role: true,
                ..Default::default()
            },
            Self::HasAny(require_roles) => ValidateApiTokenRequestPb {
                allow_any_role: false,
                require_roles: Vec::from_iter(require_roles.into_iter()),
            },
        }
    }
}

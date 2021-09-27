use std::collections::HashSet;

use crate::auth::{
    auth_ticket::{
        remote::validate::infra::ValidateApiTokenRequestDecoder,
        _common::y_protobuf::service::ValidateApiTokenRequestPb,
    },
    auth_user::remote::kernel::data::{AuthRoles, RequireAuthRoles},
};

pub struct PbValidateApiTokenRequestDecoder {
    request: ValidateApiTokenRequestPb,
}

impl PbValidateApiTokenRequestDecoder {
    pub const fn new(request: ValidateApiTokenRequestPb) -> Self {
        Self { request }
    }
}

impl ValidateApiTokenRequestDecoder for PbValidateApiTokenRequestDecoder {
    fn decode(self) -> RequireAuthRoles {
        if self.request.allow_any_role {
            RequireAuthRoles::Nothing
        } else {
            let mut require_roles = HashSet::new();
            self.request.require_roles.into_iter().for_each(|role| {
                require_roles.insert(role);
            });
            RequireAuthRoles::HasAny(AuthRoles::restore(require_roles))
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::{
        auth_ticket::remote::validate::infra::ValidateApiTokenRequestDecoder,
        auth_user::remote::kernel::data::RequireAuthRoles,
    };

    pub struct StaticValidateApiTokenRequestDecoder {
        pub require_roles: RequireAuthRoles,
    }

    impl ValidateApiTokenRequestDecoder for StaticValidateApiTokenRequestDecoder {
        fn decode(self) -> RequireAuthRoles {
            self.require_roles
        }
    }
}

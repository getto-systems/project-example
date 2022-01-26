use crate::auth::user::remote::kernel::data::RequireAuthRoles;

pub trait ValidateApiTokenRequestDecoder {
    fn decode(self) -> RequireAuthRoles;
}

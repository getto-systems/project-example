use crate::{
    auth::user::{kernel::data::GrantedAuthRolesBasket, login_id::kernel::data::LoginIdBasket},
    z_lib::remote::search::data::SearchPage,
};

pub struct SearchAuthUserAccountBasket {
    pub page: SearchPage,
    pub users: Vec<AuthUserAccountBasket>,
}
pub struct AuthUserAccountBasket {
    pub login_id: LoginIdBasket,
    pub granted_roles: GrantedAuthRolesBasket,
}

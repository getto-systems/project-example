use crate::{
    auth::user::{
        login_id::remote::data::LoginIdBasket, remote::kernel::data::GrantedAuthRolesBasket,
    },
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

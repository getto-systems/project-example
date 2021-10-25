use crate::{
    auth::user::{
        login_id::remote::data::LoginIdBasket, remote::kernel::data::GrantedAuthRolesBasket,
    },
    z_lib::remote::search::data::SearchPage,
};

pub struct SearchUserAccountBasket {
    pub page: SearchPage,
    pub users: Vec<UserAccountBasket>,
}
pub struct UserAccountBasket {
    pub login_id: LoginIdBasket,
    pub granted_roles: GrantedAuthRolesBasket,
}

use crate::auth::user::account::remote::y_protobuf::service::SearchUserAccountResponsePb;

use crate::auth::user::account::remote::proxy_search::infra::SearchUserAccountProxyResponse;

use crate::{
    auth::user::{
        account::remote::search::data::{SearchUserAccountBasket, UserAccountBasket},
        login_id::remote::data::LoginIdBasket,
        remote::kernel::data::GrantedAuthRolesBasket,
    },
    z_lib::remote::search::data::SearchPage,
};

impl Into<SearchUserAccountProxyResponse> for SearchUserAccountResponsePb {
    fn into(self) -> SearchUserAccountProxyResponse {
        SearchUserAccountProxyResponse::Success(SearchUserAccountBasket {
            page: SearchPage {
                offset: self.offset,
                limit: self.limit,
                all: self.all,
            },
            users: self
                .users
                .into_iter()
                .map(|user| UserAccountBasket {
                    login_id: LoginIdBasket::new(user.login_id),
                    granted_roles: GrantedAuthRolesBasket::new(
                        user.granted_roles.into_iter().collect(),
                    ),
                })
                .collect(),
        })
    }
}

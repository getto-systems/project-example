use crate::auth::user::account::remote::y_protobuf::service::SearchAuthUserAccountResponsePb;

use crate::auth::user::account::remote::proxy_search::infra::SearchAuthUserAccountProxyResponse;

use crate::{
    auth::user::{
        account::remote::search::data::{SearchAuthUserAccountBasket, AuthUserAccountBasket},
        login_id::remote::data::LoginIdBasket,
        remote::kernel::data::GrantedAuthRolesBasket,
    },
    z_lib::remote::search::data::SearchPage,
};

impl Into<SearchAuthUserAccountProxyResponse> for SearchAuthUserAccountResponsePb {
    fn into(self) -> SearchAuthUserAccountProxyResponse {
        SearchAuthUserAccountProxyResponse::Success(SearchAuthUserAccountBasket {
            page: SearchPage {
                offset: self.offset,
                limit: self.limit,
                all: self.all,
            },
            users: self
                .users
                .into_iter()
                .map(|user| AuthUserAccountBasket {
                    login_id: LoginIdBasket::new(user.login_id),
                    granted_roles: GrantedAuthRolesBasket::new(
                        user.granted_roles.into_iter().collect(),
                    ),
                })
                .collect(),
        })
    }
}

use crate::auth::user::account::y_protobuf::service::search_auth_user_account_pb_server::SearchAuthUserAccountPbServer;

use crate::auth::user::account::search::remote::x_tonic::route::ServiceSearch;

pub struct AccountServer;

impl AccountServer {
    pub fn search(&self) -> SearchAuthUserAccountPbServer<ServiceSearch> {
        SearchAuthUserAccountPbServer::new(ServiceSearch)
    }
}

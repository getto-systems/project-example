use crate::auth::user::account::search::y_protobuf::service::search_auth_user_account_pb_server::SearchAuthUserAccountPbServer;

use crate::auth::user::account::{
    modify::x_tonic::route::ServiceModifyUser, register::x_tonic::route::ServiceRegisterUser,
    search::x_tonic::route::ServiceSearch, unregister::x_tonic::route::ServiceUnregisterUser,
};

#[derive(Default)]
pub struct AccountServer {
    pub register: ServiceRegisterUser,
    pub modify: ServiceModifyUser,
    pub unregister: ServiceUnregisterUser,
}

impl AccountServer {
    pub fn search(&self) -> SearchAuthUserAccountPbServer<ServiceSearch> {
        SearchAuthUserAccountPbServer::new(ServiceSearch)
    }
}

use crate::auth::user::account::{
    modify::y_protobuf::service::modify_auth_user_account_pb_server::ModifyAuthUserAccountPbServer,
    register::y_protobuf::service::register_auth_user_account_pb_server::RegisterAuthUserAccountPbServer,
    search::y_protobuf::service::search_auth_user_account_pb_server::SearchAuthUserAccountPbServer,
};

use crate::auth::user::account::{
    modify::x_tonic::route::ServiceModifyUser, register::x_tonic::route::ServiceRegisterUser,
    search::x_tonic::route::ServiceSearch,
};

pub struct AccountServer;

impl AccountServer {
    pub fn search(&self) -> SearchAuthUserAccountPbServer<ServiceSearch> {
        SearchAuthUserAccountPbServer::new(ServiceSearch)
    }
    pub fn modify_user(&self) -> ModifyAuthUserAccountPbServer<ServiceModifyUser> {
        ModifyAuthUserAccountPbServer::new(ServiceModifyUser)
    }
    pub fn register_user(&self) -> RegisterAuthUserAccountPbServer<ServiceRegisterUser> {
        RegisterAuthUserAccountPbServer::new(ServiceRegisterUser)
    }
}

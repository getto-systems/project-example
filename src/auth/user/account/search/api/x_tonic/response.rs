use tonic::{Response, Status};

use crate::auth::user::account::{
    search::y_protobuf::service::SearchAuthUserAccountResponsePb,
    y_protobuf::service::AuthUserAccountPb,
};

use crate::z_lib::response::tonic::ServiceResponder;

use super::super::action::{SearchAuthUserAccountEvent, SearchAuthUserAccountState};

use crate::auth::user::account::search::data::SearchAuthUserAccountBasket;

impl ServiceResponder<SearchAuthUserAccountResponsePb> for SearchAuthUserAccountState {
    fn respond_to(self) -> Result<Response<SearchAuthUserAccountResponsePb>, Status> {
        match self {
            Self::Validate(_) => Err(Status::permission_denied("permission denied")),
            Self::Search(event) => event.respond_to(),
        }
    }
}

impl ServiceResponder<SearchAuthUserAccountResponsePb> for SearchAuthUserAccountEvent {
    fn respond_to(self) -> Result<Response<SearchAuthUserAccountResponsePb>, Status> {
        match self {
            Self::Success(response) => response.respond_to(),
            Self::RepositoryError(err) => err.respond_to(),
        }
    }
}

impl ServiceResponder<SearchAuthUserAccountResponsePb> for SearchAuthUserAccountBasket {
    fn respond_to(self) -> Result<Response<SearchAuthUserAccountResponsePb>, Status> {
        Ok(Response::new(self.into()))
    }
}

impl Into<SearchAuthUserAccountResponsePb> for SearchAuthUserAccountBasket {
    fn into(self) -> SearchAuthUserAccountResponsePb {
        SearchAuthUserAccountResponsePb {
            offset: self.page.offset,
            limit: self.page.limit,
            all: self.page.all,
            users: self
                .users
                .into_iter()
                .map(|user| AuthUserAccountPb {
                    login_id: user.login_id.extract(),
                    granted_roles: user.granted_roles.extract().into_iter().collect(),
                })
                .collect(),
            ..Default::default()
        }
    }
}
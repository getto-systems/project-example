use tonic::{Response, Status};

use crate::auth::user::account::{
    search::y_protobuf::service::SearchAuthUserAccountResponsePb,
    y_protobuf::service::AuthUserAccountPb,
};

use crate::common::api::response::tonic::ServiceResponder;

use super::super::action::{SearchAuthUserAccountEvent, SearchAuthUserAccountState};

use crate::auth::user::account::search::data::AuthUserAccountSearch;

impl ServiceResponder<SearchAuthUserAccountResponsePb> for SearchAuthUserAccountState {
    fn respond_to(self) -> Result<Response<SearchAuthUserAccountResponsePb>, Status> {
        match self {
            Self::Authorize(event) => event.respond_to(),
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

impl ServiceResponder<SearchAuthUserAccountResponsePb> for AuthUserAccountSearch {
    fn respond_to(self) -> Result<Response<SearchAuthUserAccountResponsePb>, Status> {
        Ok(Response::new(self.into()))
    }
}

impl Into<SearchAuthUserAccountResponsePb> for AuthUserAccountSearch {
    fn into(self) -> SearchAuthUserAccountResponsePb {
        SearchAuthUserAccountResponsePb {
            page: Some(self.page.into()),
            sort: Some(self.sort.into()),
            users: self
                .users
                .into_iter()
                .map(|user| AuthUserAccountPb {
                    login_id: user.login_id.extract(),
                    granted: user.attrs.granted.extract().into_iter().collect(),
                    memo: user.attrs.memo.extract(),
                    reset_token_destination: Some(user.reset_token_destination.into()),
                })
                .collect(),
            ..Default::default()
        }
    }
}

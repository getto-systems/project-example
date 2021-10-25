use tonic::{Response, Status};

use crate::auth::user::account::remote::y_protobuf::service::{
    SearchUserAccountResponsePb, UserAccountPb,
};

use crate::z_lib::remote::response::tonic::RespondTo;

use super::super::event::SearchUserAccountEvent;

use crate::auth::user::account::remote::search::data::SearchUserAccountBasket;

impl RespondTo<SearchUserAccountResponsePb> for SearchUserAccountEvent {
    fn respond_to(self) -> Result<Response<SearchUserAccountResponsePb>, Status> {
        match self {
            Self::Success(response) => response.respond_to(),
            Self::Validate(_) => Err(Status::cancelled("change password cancelled")),
            Self::RepositoryError(err) => err.respond_to(),
        }
    }
}

impl RespondTo<SearchUserAccountResponsePb> for SearchUserAccountBasket {
    fn respond_to(self) -> Result<Response<SearchUserAccountResponsePb>, Status> {
        Ok(Response::new(self.into()))
    }
}

impl Into<SearchUserAccountResponsePb> for SearchUserAccountBasket {
    fn into(self) -> SearchUserAccountResponsePb {
        SearchUserAccountResponsePb {
            offset: self.page.offset,
            limit: self.page.limit,
            all: self.page.all,
            users: self
                .users
                .into_iter()
                .map(|user| UserAccountPb {
                    login_id: user.login_id.extract(),
                    granted_roles: user.granted_roles.extract().into_iter().collect(),
                })
                .collect(),
            ..Default::default()
        }
    }
}

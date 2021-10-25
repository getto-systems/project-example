use tonic::{Response, Status};

use crate::auth::user::account::remote::y_protobuf::service::SearchUserAccountResponsePb;

use crate::z_lib::remote::response::tonic::RespondTo;

use super::super::action::SearchUserAccountState;

impl RespondTo<SearchUserAccountResponsePb> for SearchUserAccountState {
    fn respond_to(self) -> Result<Response<SearchUserAccountResponsePb>, Status> {
        match self {
            Self::Search(event) => event.respond_to(),
        }
    }
}

use tonic::{Response, Status};

use crate::auth::user::account::remote::y_protobuf::service::SearchAuthUserAccountResponsePb;

use crate::z_lib::remote::response::tonic::RespondTo;

use super::super::action::SearchAuthUserAccountState;

impl RespondTo<SearchAuthUserAccountResponsePb> for SearchAuthUserAccountState {
    fn respond_to(self) -> Result<Response<SearchAuthUserAccountResponsePb>, Status> {
        match self {
            Self::Search(event) => event.respond_to(),
        }
    }
}

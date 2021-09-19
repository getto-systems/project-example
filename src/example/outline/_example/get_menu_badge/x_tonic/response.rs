use tonic::{Response, Status};

use crate::z_details::_common::response::tonic::RespondTo;

use crate::example::outline::_common::y_protobuf::service::GetMenuBadgeResponsePb;

use super::super::event::GetOutlineMenuBadgeEvent;

impl RespondTo<GetMenuBadgeResponsePb> for GetOutlineMenuBadgeEvent {
    fn respond_to(self) -> Result<Response<GetMenuBadgeResponsePb>, Status> {
        match self {
            Self::Success(menu_badge) => {
                let response: GetMenuBadgeResponsePb = menu_badge.into();
                Ok(Response::new(response))
            }
            Self::ValidateError(_) => Err(Status::cancelled("get outline menu cancelled")),
            Self::RepositoryError(err) => err.respond_to(),
        }
    }
}

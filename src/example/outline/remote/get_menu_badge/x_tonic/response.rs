use tonic::{Response, Status};

use crate::z_lib::remote::response::tonic::RespondTo;

use crate::example::outline::remote::y_protobuf::service::GetMenuBadgeResponsePb;

use super::super::event::GetOutlineMenuBadgeEvent;

impl RespondTo<GetMenuBadgeResponsePb> for GetOutlineMenuBadgeEvent {
    fn respond_to(self) -> Result<Response<GetMenuBadgeResponsePb>, Status> {
        match self {
            Self::Success(menu_badge) => {
                let response: GetMenuBadgeResponsePb = menu_badge.into();
                Ok(Response::new(response))
            }
            Self::ValidateError(_) => Err(Status::unauthenticated("unauthenticated")),
            Self::RepositoryError(err) => err.respond_to(),
        }
    }
}
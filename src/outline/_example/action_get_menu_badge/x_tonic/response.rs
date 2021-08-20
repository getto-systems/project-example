use tonic::{Response, Status};

use crate::z_details::_common::response::tonic::RespondTo;

use crate::outline::_common::y_protobuf::service::GetMenuBadgeResponsePb;

use super::super::action::GetOutlineMenuBadgeState;

impl RespondTo<GetMenuBadgeResponsePb> for GetOutlineMenuBadgeState {
    fn respond_to(self) -> Result<Response<GetMenuBadgeResponsePb>, Status> {
        match self {
            Self::GetMenuBadge(event) => event.respond_to(),
        }
    }
}

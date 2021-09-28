use tonic::{Response, Status};

use crate::z_lib::remote::response::tonic::RespondTo;

use crate::example::outline::remote::y_protobuf::service::GetMenuBadgeResponsePb;

use super::super::action::GetOutlineMenuBadgeState;

impl RespondTo<GetMenuBadgeResponsePb> for GetOutlineMenuBadgeState {
    fn respond_to(self) -> Result<Response<GetMenuBadgeResponsePb>, Status> {
        match self {
            Self::GetMenuBadge(event) => event.respond_to(),
        }
    }
}

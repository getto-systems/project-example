use tonic::{Response, Status};

use crate::z_lib::response::tonic::ServiceResponder;

use crate::core::outline::load::y_protobuf::service::LoadMenuBadgeResponsePb;

use super::super::action::{LoadOutlineMenuBadgeEvent, LoadOutlineMenuBadgeState};

use crate::core::outline::load::data::{OutlineMenuBadge, OutlineMenuBadgeCount};

impl ServiceResponder<LoadMenuBadgeResponsePb> for LoadOutlineMenuBadgeState {
    fn respond_to(self) -> Result<Response<LoadMenuBadgeResponsePb>, Status> {
        match self {
            Self::Validate(_) => Err(Status::unauthenticated("unauthenticated")),
            Self::LoadMenuBadge(event) => event.respond_to(),
        }
    }
}

impl ServiceResponder<LoadMenuBadgeResponsePb> for LoadOutlineMenuBadgeEvent {
    fn respond_to(self) -> Result<Response<LoadMenuBadgeResponsePb>, Status> {
        match self {
            Self::Success(menu_badge) => {
                let response: LoadMenuBadgeResponsePb = menu_badge.into();
                Ok(Response::new(response))
            }
            Self::RepositoryError(err) => err.respond_to(),
        }
    }
}

impl Into<OutlineMenuBadge> for LoadMenuBadgeResponsePb {
    fn into(self) -> OutlineMenuBadge {
        OutlineMenuBadge {
            index: OutlineMenuBadgeCount::restore(self.index),
        }
    }
}

impl Into<LoadMenuBadgeResponsePb> for OutlineMenuBadge {
    fn into(self) -> LoadMenuBadgeResponsePb {
        LoadMenuBadgeResponsePb {
            index: self.index.extract(),
        }
    }
}
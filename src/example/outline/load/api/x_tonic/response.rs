use tonic::{Response, Status};

use crate::z_lib::api::response::tonic::RespondTo;

use crate::example::outline::y_protobuf::service::LoadMenuBadgeResponsePb;

use super::super::action::{LoadOutlineMenuBadgeEvent, LoadOutlineMenuBadgeState};

use crate::example::outline::load::api::data::{OutlineMenuBadge, OutlineMenuBadgeCount};

impl RespondTo<LoadMenuBadgeResponsePb> for LoadOutlineMenuBadgeState {
    fn respond_to(self) -> Result<Response<LoadMenuBadgeResponsePb>, Status> {
        match self {
            Self::Validate(_) => Err(Status::unauthenticated("unauthenticated")),
            Self::LoadMenuBadge(event) => event.respond_to(),
        }
    }
}

impl RespondTo<LoadMenuBadgeResponsePb> for LoadOutlineMenuBadgeEvent {
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

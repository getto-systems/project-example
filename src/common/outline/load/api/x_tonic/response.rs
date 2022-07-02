use tonic::{Response, Status};

use crate::z_lib::response::tonic::ServiceResponder;

use crate::common::outline::load::y_protobuf::service::{
    LoadMenuBadgeEntryPb, LoadMenuBadgeResponsePb,
};

use super::super::action::{LoadOutlineMenuBadgeEvent, LoadOutlineMenuBadgeState};

use crate::common::outline::load::data::OutlineMenuBadge;

impl ServiceResponder<LoadMenuBadgeResponsePb> for LoadOutlineMenuBadgeState {
    fn respond_to(self) -> Result<Response<LoadMenuBadgeResponsePb>, Status> {
        match self {
            Self::Authorize(event) => event.respond_to(),
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

impl Into<LoadMenuBadgeResponsePb> for OutlineMenuBadge {
    fn into(self) -> LoadMenuBadgeResponsePb {
        let response = LoadMenuBadgeResponsePb {
            items: self
                .extract()
                .into_iter()
                .map(|(path, count)| LoadMenuBadgeEntryPb {
                    path: path.to_string(),
                    count: count.extract(),
                })
                .collect(),
        };

        response
    }
}

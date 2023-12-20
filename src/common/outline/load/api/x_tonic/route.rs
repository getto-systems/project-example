use std::sync::Arc;

use tonic::{Request, Response, Status};

use crate::common::outline::load::y_protobuf::service::{
    load_outline_menu_badge_pb_server::{LoadOutlineMenuBadgePb, LoadOutlineMenuBadgePbServer},
    LoadOutlineMenuBadgeRequestPb, LoadOutlineMenuBadgeResponsePb, MenuBadgePb,
};

use crate::x_outside_feature::core::feature::CoreAppFeature;

use crate::{
    common::api::{logger::detail::StdoutJsonLogger, response::x_tonic::ServiceResponder},
    x_content::menu::badge::GatherOutlineMenuBadgeAction,
};

use crate::common::outline::load::action::LoadOutlineMenuBadgeAction;

use crate::common::{
    api::request::data::RequestInfo,
    outline::load::data::{LoadOutlineMenuBadgeError, OutlineMenuBadge},
};

#[derive(Default)]
pub struct ServiceLoadMenuBadge;

impl ServiceLoadMenuBadge {
    pub fn server(&self) -> LoadOutlineMenuBadgePbServer<Self> {
        LoadOutlineMenuBadgePbServer::new(Self)
    }
}

#[async_trait::async_trait]
impl LoadOutlineMenuBadgePb for ServiceLoadMenuBadge {
    async fn load(
        &self,
        request: Request<LoadOutlineMenuBadgeRequestPb>,
    ) -> Result<Response<LoadOutlineMenuBadgeResponsePb>, Status> {
        async {
            let feature = CoreAppFeature::from_request(&request);
            let info = RequestInfo::from_metadata(request.metadata());
            let logger = Arc::new(StdoutJsonLogger::with_request(info.clone()));

            LoadOutlineMenuBadgeAction::new(GatherOutlineMenuBadgeAction::live(
                &feature, info, &logger,
            ))
            .with_logger(logger)
            .load()
            .await
        }
        .await
        .respond_to()
    }
}

impl ServiceResponder<LoadOutlineMenuBadgeResponsePb> for OutlineMenuBadge {
    fn respond_to(self) -> Result<Response<LoadOutlineMenuBadgeResponsePb>, Status> {
        Ok(Response::new(LoadOutlineMenuBadgeResponsePb {
            items: self
                .extract()
                .into_iter()
                .map(|(path, count)| MenuBadgePb {
                    path: path.extract(),
                    count: count.extract(),
                })
                .collect(),
        }))
    }
}

impl ServiceResponder<LoadOutlineMenuBadgeResponsePb> for LoadOutlineMenuBadgeError {
    fn respond_to(self) -> Result<Response<LoadOutlineMenuBadgeResponsePb>, Status> {
        match self {
            Self::RepositoryError(err) => err.respond_to(),
            Self::AuthorizeProxyError(err) => err.respond_to(),
        }
    }
}

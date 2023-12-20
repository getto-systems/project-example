use std::sync::Arc;

use actix_web::{get, web::Data, HttpRequest, HttpResponse};

use crate::x_outside_feature::proxy::feature::ProxyAppFeature;

use crate::common::api::{
    feature::AsInfra, logger::detail::StdoutJsonLogger, response::x_actix_web::ProxyResponder,
};

use crate::{
    auth::action::CheckAuthorizeTokenAction,
    common::outline::load::proxy::action::LoadOutlineMenuBadgeProxyAction,
};

use crate::common::api::request::data::RequestInfo;

#[get("menu-badge")]
pub async fn service_load_menu_badge(
    feature: Data<ProxyAppFeature>,
    request: HttpRequest,
    body: String,
) -> HttpResponse {
    async {
        let info = RequestInfo::from_request(&request);
        let logger = Arc::new(StdoutJsonLogger::with_request(info.clone()));

        let infra = CheckAuthorizeTokenAction::live(feature.as_infra())
            .with_logger(logger.clone())
            .pick_authorized_infra(&feature, &request)
            .await?;

        LoadOutlineMenuBadgeProxyAction::live(infra)
            .with_logger(logger)
            .call(info, &request, body)
            .await
    }
    .await
    .respond_to()
}

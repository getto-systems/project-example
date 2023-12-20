use std::sync::Arc;

use actix_web::{
    get,
    web::{Data, Path},
    HttpRequest, HttpResponse,
};

use crate::x_outside_feature::proxy::feature::ProxyAppFeature;

use crate::common::api::{
    feature::AsInfra, logger::detail::StdoutJsonLogger, response::x_actix_web::ProxyResponder,
};

use crate::auth::{
    ticket::authorize::action::CheckAuthorizeTokenAction,
    user::account::search::proxy::action::SearchAuthUserAccountProxyAction,
};

use crate::common::api::request::data::RequestInfo;

#[get("/search/{body}")]
async fn service_search_user(
    feature: Data<ProxyAppFeature>,
    request: HttpRequest,
    path: Path<String>,
) -> HttpResponse {
    async {
        let info = RequestInfo::from_request(&request);
        let logger = Arc::new(StdoutJsonLogger::with_request(info.clone()));

        let infra = CheckAuthorizeTokenAction::live(feature.as_infra())
            .with_logger(logger.clone())
            .pick_authorized_infra(&feature, &request)
            .await?;

        SearchAuthUserAccountProxyAction::live(infra)
            .with_logger(logger)
            .call(info, &request, path.into_inner())
            .await
    }
    .await
    .respond_to()
}

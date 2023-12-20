use std::sync::Arc;

use actix_web::{patch, web::Data, HttpRequest, HttpResponse};

use crate::{
    common::api::response::x_actix_web::ProxyResponder,
    x_outside_feature::proxy::feature::ProxyAppFeature,
};

use crate::common::api::{feature::AsInfra, logger::detail::StdoutJsonLogger};

use crate::auth::{
    ticket::authorize::action::CheckAuthorizeTokenAction,
    user::login_id::change::proxy::action::OverwriteLoginIdProxyAction,
};

use crate::common::api::request::data::RequestInfo;

#[patch("/overwrite")]
pub async fn service_overwrite_login_id(
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

        OverwriteLoginIdProxyAction::live(infra)
            .with_logger(logger)
            .call(info, &request, body)
            .await
    }
    .await
    .respond_to()
}

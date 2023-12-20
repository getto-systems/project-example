use std::sync::Arc;

use actix_web::{post, web::Data, HttpRequest, HttpResponse};

use crate::{
    common::api::{
        feature::AsInfra, logger::detail::StdoutJsonLogger, response::x_actix_web::ProxyResponder,
    },
    x_outside_feature::proxy::feature::ProxyAppFeature,
};

use crate::auth::user::password::reset::request_token::proxy::action::RequestResetPasswordTokenProxyAction;

use crate::common::api::request::data::RequestInfo;

#[post("/token")]
pub async fn service_request_token(
    feature: Data<ProxyAppFeature>,
    request: HttpRequest,
    body: String,
) -> HttpResponse {
    async {
        let info = RequestInfo::from_request(&request);
        let logger = Arc::new(StdoutJsonLogger::with_request(info.clone()));

        RequestResetPasswordTokenProxyAction::live(feature.as_infra())
            .with_logger(logger.clone())
            .request(info, (), body)
            .await
    }
    .await
    .respond_to()
}

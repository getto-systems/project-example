use std::sync::Arc;

use actix_web::{post, web::Data, HttpRequest, HttpResponse};

use crate::{
    common::api::response::x_actix_web::ProxyResponder,
    x_outside_feature::proxy::feature::ProxyAppFeature,
};

use crate::common::api::{feature::AsInfra, logger::detail::StdoutJsonLogger};

use crate::auth::user::password::reset::reset::proxy::action::ResetPasswordProxyAction;

use crate::common::api::request::data::RequestInfo;

#[post("")]
pub async fn service_reset(
    feature: Data<ProxyAppFeature>,
    request: HttpRequest,
    body: String,
) -> HttpResponse {
    async {
        let info = RequestInfo::from_request(&request);
        let logger = Arc::new(StdoutJsonLogger::with_request(info.clone()));

        ResetPasswordProxyAction::live(feature.as_infra())
            .with_logger(logger.clone())
            .request(info, (), body)
            .await
    }
    .await
    .respond_to()
}

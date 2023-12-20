use std::sync::Arc;

use actix_web::{post, web::Data, HttpRequest, HttpResponse};

use crate::{
    common::api::{
        feature::AsInfra, logger::detail::StdoutJsonLogger, response::x_actix_web::ProxyResponder,
    },
    x_outside_feature::proxy::feature::ProxyAppFeature,
};

use crate::auth::user::password::authenticate::proxy::action::AuthenticateWithPasswordProxyAction;

use crate::common::api::request::data::RequestInfo;

#[post("/authenticate")]
async fn service_authenticate(
    feature: Data<ProxyAppFeature>,
    request: HttpRequest,
    body: String,
) -> HttpResponse {
    async {
        let info = RequestInfo::from_request(&request);
        let logger = Arc::new(StdoutJsonLogger::with_request(info.clone()));

        AuthenticateWithPasswordProxyAction::live(feature.as_infra())
            .with_logger(logger.clone())
            .authenticate(info, (), body)
            .await
    }
    .await
    .respond_to()
}

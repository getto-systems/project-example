use getto_application_test::ApplicationActionStateHolder;
use pretty_assertions::assert_eq;

use crate::{
    auth::init::test::{StaticAuthorizeToken, StaticAuthorizeWithTokenInfra},
    common::proxy::init::test::StaticCoreProxyMaterial,
};

use crate::common::proxy::action::CoreProxyAction;

use crate::common::proxy::infra::ProxyCall;

use crate::{
    auth::data::AuthPermissionRequired,
    common::proxy::data::{CoreProxyError, ProxyMetadataExtract, ProxyResponseBody},
};

#[tokio::test]
async fn success_call_proxy() {
    let holder = ApplicationActionStateHolder::new();

    let material = StaticCoreProxyMaterial {
        authorize_with_token: StaticAuthorizeWithTokenInfra::for_common_test(),
        proxy_call: StaticProxyCall,
    };
    let request = ();

    let mut action =
        CoreProxyAction::with_material(("test-proxy", AuthPermissionRequired::Nothing), material);
    action.subscribe(holder.handler());

    let result = action.ignite(StaticAuthorizeToken, request).await;
    assert_eq!(
        holder.extract(),
        vec![
            "authorize with token success",
            "try to proxy call: test-proxy",
            "proxy call success",
        ],
    );
    assert!(result.is_ok());
}

pub struct StaticProxyCall;

#[async_trait::async_trait]
impl ProxyCall for StaticProxyCall {
    type Request = ();
    type Response = ProxyResponseBody;
    type Error = CoreProxyError;

    async fn call(
        &self,
        _metadata: impl ProxyMetadataExtract,
        _request: Self::Request,
    ) -> Result<Self::Response, Self::Error> {
        Ok(ProxyResponseBody::restore("RESPONSE".to_owned()))
    }
}

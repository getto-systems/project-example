use crate::{
    auth::data::{CheckAuthorizeTokenError, ValidateAuthorizeTokenError},
    common::{api::request::data::RequestInfo, proxy::data::ProxyMetadataExtract},
};

pub trait ProxyCallInfra {
    type ProxyCall: ProxyCall;

    fn proxy_call(&self) -> &Self::ProxyCall;
}

pub trait ProxyCallLogger<R, E> {
    fn try_to_proxy_call(&self);
    fn invalid_authorize_token(
        &self,
        err: ValidateAuthorizeTokenError,
    ) -> ValidateAuthorizeTokenError;
    fn failed_to_proxy_call(&self, err: E) -> E;
    fn succeed_to_proxy_call(&self, response: R) -> R;
}

#[async_trait::async_trait]
pub trait ProxyCall {
    type Request;
    type Response;
    type Error: From<ValidateAuthorizeTokenError> + From<CheckAuthorizeTokenError>;

    async fn call(
        &self,
        info: RequestInfo,
        metadata: impl ProxyMetadataExtract,
        request: Self::Request,
    ) -> Result<Self::Response, Self::Error>;
}

mod detail {
    use crate::common::api::logger::detail::StdoutJsonLogger;

    use crate::auth::data::ValidateAuthorizeTokenError;

    impl<R, E: std::fmt::Display> super::ProxyCallLogger<R, E> for StdoutJsonLogger {
        fn try_to_proxy_call(&self) {
            self.debug(format!("try to proxy-call"));
        }
        fn invalid_authorize_token(
            &self,
            err: ValidateAuthorizeTokenError,
        ) -> ValidateAuthorizeTokenError {
            self.fatal(format!("invalid authorize-token; {}", &err));
            err
        }
        fn failed_to_proxy_call(&self, err: E) -> E {
            self.fatal(format!("failed to proxy-call; {}", &err));
            err
        }
        fn succeed_to_proxy_call(&self, response: R) -> R {
            self.debug(format!("succeed to proxy-call"));
            response
        }
    }
}

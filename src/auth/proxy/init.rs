#[cfg(test)]
pub mod test {
    use crate::auth::{
        proxy::infra::AuthProxyService, ticket::validate::infra::AuthMetadataContent,
    };

    use crate::auth::proxy::data::AuthProxyError;

    pub struct StaticProxyService {
        pub name: &'static str,
        pub response: StaticProxyResponse,
    }
    pub enum StaticProxyResponse {
        Succeed(String),
    }

    #[async_trait::async_trait]
    impl AuthProxyService for StaticProxyService {
        type Response = String;

        fn name(&self) -> &str {
            self.name
        }
        async fn call(
            self,
            _metadata: AuthMetadataContent,
        ) -> Result<Self::Response, AuthProxyError> {
            match self.response {
                StaticProxyResponse::Succeed(response) => Ok(response),
            }
        }
    }
}

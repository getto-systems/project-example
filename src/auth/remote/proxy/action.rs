use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::auth::remote::method::{
    validate_auth_metadata, ValidateAuthMetadataEvent, ValidateAuthMetadataInfra,
};

use crate::auth::remote::proxy::infra::AuthProxyService;

pub enum AuthProxyState<R, E> {
    Metadata(ValidateAuthMetadataEvent),
    TryToCall(String),
    Response(R),
    ServiceError(E),
}

mod auth_proxy_state {
    use super::AuthProxyState;

    const SUCCESS: &'static str = "proxy call success";
    const ERROR: &'static str = "proxy call error";

    impl<R, E: std::fmt::Display> std::fmt::Display for AuthProxyState<R, E> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Metadata(event) => event.fmt(f),
                Self::TryToCall(target) => write!(f, "try to proxy call: {}", target),
                Self::Response(_) => write!(f, "{}", SUCCESS),
                Self::ServiceError(err) => write!(f, "{}; {}", ERROR, err),
            }
        }
    }
}

pub trait AuthProxyMaterial {
    type Validate: ValidateAuthMetadataInfra;
    type ProxyService: AuthProxyService;

    fn extract(self) -> (Self::Validate, Self::ProxyService);
}

pub struct AuthProxyAction<M: AuthProxyMaterial> {
    pubsub: ActionStatePubSub<
        AuthProxyState<
            <<M as AuthProxyMaterial>::ProxyService as AuthProxyService>::Response,
            <<M as AuthProxyMaterial>::ProxyService as AuthProxyService>::Error,
        >,
    >,
    material: M,
}

impl<M: AuthProxyMaterial> AuthProxyAction<M> {
    pub fn with_material(material: M) -> Self {
        Self {
            pubsub: ActionStatePubSub::new(),
            material,
        }
    }

    pub fn subscribe(
        &mut self,
        handler: impl 'static
            + Fn(
                &AuthProxyState<
                    <<M as AuthProxyMaterial>::ProxyService as AuthProxyService>::Response,
                    <<M as AuthProxyMaterial>::ProxyService as AuthProxyService>::Error,
                >,
            )
            + Send
            + Sync,
    ) {
        self.pubsub.subscribe(handler);
    }

    pub async fn ignite(
        self,
    ) -> MethodResult<
        AuthProxyState<
            <<M as AuthProxyMaterial>::ProxyService as AuthProxyService>::Response,
            <<M as AuthProxyMaterial>::ProxyService as AuthProxyService>::Error,
        >,
    > {
        let pubsub = self.pubsub;
        let (validate, proxy_service) = self.material.extract();

        let metadata = validate_auth_metadata(&validate, |event| {
            pubsub.post(AuthProxyState::Metadata(event))
        })?;

        pubsub.post(AuthProxyState::TryToCall(proxy_service.name().into()));

        Ok(pubsub.post(AuthProxyState::Response(
            proxy_service
                .call(metadata)
                .await
                .map_err(|err| pubsub.post(AuthProxyState::ServiceError(err)))?,
        )))
    }
}

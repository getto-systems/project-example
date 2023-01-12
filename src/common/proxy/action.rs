use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::{
    auth::{
        data::{AuthPermissionRequired, AuthorizeTokenExtract},
        method::{authorize_with_token, AuthorizeWithTokenEvent, AuthorizeWithTokenInfra},
    },
    common::proxy::{
        data::{CoreProxyError, ProxyResponseBody},
        event::CoreProxyCallEvent,
        infra::ProxyCall,
    },
};

pub enum CoreProxyState {
    AuthorizeWithToken(AuthorizeWithTokenEvent),
    ProxyCall(CoreProxyCallEvent<ProxyResponseBody>),
}

impl std::fmt::Display for CoreProxyState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AuthorizeWithToken(event) => event.fmt(f),
            Self::ProxyCall(event) => event.fmt(f),
        }
    }
}

pub trait CoreProxyMaterial {
    type AuthorizeWithToken: AuthorizeWithTokenInfra;
    type ProxyCall: ProxyCall<Response = ProxyResponseBody, Error = CoreProxyError>;

    fn authorize_with_token(&self) -> &Self::AuthorizeWithToken;
    fn proxy_call(&self) -> &Self::ProxyCall;
}

pub struct CoreProxyAction<M: CoreProxyMaterial> {
    pubsub: ActionStatePubSub<CoreProxyState>,
    name: &'static str,
    required: AuthPermissionRequired,
    material: M,
}

pub type CoreProxyParams = (&'static str, AuthPermissionRequired);

impl<M: CoreProxyMaterial> CoreProxyAction<M> {
    pub fn with_material((name, required): CoreProxyParams, material: M) -> Self {
        Self {
            pubsub: ActionStatePubSub::new(),
            name,
            required,
            material,
        }
    }

    pub fn subscribe(&mut self, handler: impl 'static + Fn(&CoreProxyState) + Send + Sync) {
        self.pubsub.subscribe(handler);
    }

    pub async fn ignite(
        self,
        token: impl AuthorizeTokenExtract,
        request: <<M as CoreProxyMaterial>::ProxyCall as ProxyCall>::Request,
    ) -> MethodResult<CoreProxyState> {
        let (_ticket, token, _required) = authorize_with_token(
            self.material.authorize_with_token(),
            (token, self.required),
            |event| self.pubsub.post(CoreProxyState::AuthorizeWithToken(event)),
        )
        .await?;

        self.pubsub
            .post(CoreProxyState::ProxyCall(CoreProxyCallEvent::TryToCall(
                self.name.to_owned(),
            )));

        Ok(self
            .pubsub
            .post(CoreProxyState::ProxyCall(CoreProxyCallEvent::Response(
                self.material
                    .proxy_call()
                    .call(token, request)
                    .await
                    .map_err(|err| {
                        self.pubsub.post(CoreProxyState::ProxyCall(
                            CoreProxyCallEvent::ServiceError(err),
                        ))
                    })?,
            ))))
    }
}

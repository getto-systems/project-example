use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::auth::ticket::authenticate::action::AuthenticateWithTokenActionInfo;

use crate::auth::ticket::authenticate::{
    method::{authenticate_with_token, AuthenticateWithTokenEvent, AuthenticateWithTokenInfra},
    proxy::method::AuthProxyCallEvent,
};

use crate::common::proxy::infra::ProxyCall;

use crate::auth::{
    proxy::data::AuthProxyError,
    ticket::{
        authenticate::proxy::data::ProxyResponseAuthenticated,
        kernel::data::AuthenticateTokenExtract,
    },
};

pub enum AuthenticateWithTokenProxyState {
    AuthenticateWithToken(AuthenticateWithTokenEvent),
    ProxyCall(AuthProxyCallEvent<ProxyResponseAuthenticated>),
}

impl std::fmt::Display for AuthenticateWithTokenProxyState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AuthenticateWithToken(event) => event.fmt(f),
            Self::ProxyCall(event) => event.fmt(f),
        }
    }
}

pub trait AuthenticateWithTokenProxyMaterial {
    type AuthenticateWithToken: AuthenticateWithTokenInfra;
    type ProxyCall: ProxyCall<
        Request = (),
        Response = ProxyResponseAuthenticated,
        Error = AuthProxyError,
    >;

    fn authenticate_with_token(&self) -> &Self::AuthenticateWithToken;
    fn proxy_call(&self) -> &Self::ProxyCall;
}

pub struct AuthenticateWithTokenProxyAction<M: AuthenticateWithTokenProxyMaterial> {
    pubsub: ActionStatePubSub<AuthenticateWithTokenProxyState>,
    material: M,
}

impl<M: AuthenticateWithTokenProxyMaterial> AuthenticateWithTokenProxyAction<M> {
    pub fn with_material(material: M) -> Self {
        Self {
            pubsub: ActionStatePubSub::new(),
            material,
        }
    }

    pub fn subscribe(
        &mut self,
        handler: impl 'static + Fn(&AuthenticateWithTokenProxyState) + Send + Sync,
    ) {
        self.pubsub.subscribe(handler);
    }

    pub async fn ignite(
        self,
        token: impl AuthenticateTokenExtract,
    ) -> MethodResult<AuthenticateWithTokenProxyState> {
        let (_ticket, token) =
            authenticate_with_token(self.material.authenticate_with_token(), token, |event| {
                self.pubsub
                    .post(AuthenticateWithTokenProxyState::AuthenticateWithToken(
                        event,
                    ))
            })
            .await?;

        self.pubsub.post(AuthenticateWithTokenProxyState::ProxyCall(
            AuthProxyCallEvent::TryToCall(AuthenticateWithTokenActionInfo.name().to_owned()),
        ));

        Ok(self.pubsub.post(AuthenticateWithTokenProxyState::ProxyCall(
            AuthProxyCallEvent::Response(
                self.material
                    .proxy_call()
                    .call(token, ())
                    .await
                    .map_err(|err| {
                        self.pubsub.post(AuthenticateWithTokenProxyState::ProxyCall(
                            AuthProxyCallEvent::ServiceError(err),
                        ))
                    })?,
            ),
        )))
    }
}

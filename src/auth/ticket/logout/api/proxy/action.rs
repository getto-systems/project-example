use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::auth::ticket::logout::action::LogoutActionInfo;

use crate::auth::ticket::authenticate::{
    method::{authenticate_with_token, AuthenticateWithTokenEvent, AuthenticateWithTokenInfra},
    proxy::method::AuthProxyCallEvent,
};

use crate::common::proxy::infra::ProxyCall;

use crate::{
    auth::{proxy::data::AuthProxyError, ticket::kernel::data::AuthenticateTokenExtract},
    common::proxy::data::ProxyResponseBody,
};

pub enum LogoutProxyState {
    AuthenticateWithToken(AuthenticateWithTokenEvent),
    ProxyCall(AuthProxyCallEvent<ProxyResponseBody>),
}

impl std::fmt::Display for LogoutProxyState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AuthenticateWithToken(event) => event.fmt(f),
            Self::ProxyCall(event) => event.fmt(f),
        }
    }
}

pub trait LogoutProxyMaterial {
    type AuthenticateWithToken: AuthenticateWithTokenInfra;
    type ProxyCall: ProxyCall<Request = (), Response = ProxyResponseBody, Error = AuthProxyError>;

    fn authenticate_with_token(&self) -> &Self::AuthenticateWithToken;
    fn proxy_call(&self) -> &Self::ProxyCall;
}

pub struct LogoutProxyAction<M: LogoutProxyMaterial> {
    pubsub: ActionStatePubSub<LogoutProxyState>,
    material: M,
}

impl<M: LogoutProxyMaterial> LogoutProxyAction<M> {
    pub fn with_material(material: M) -> Self {
        Self {
            pubsub: ActionStatePubSub::new(),
            material,
        }
    }

    pub fn subscribe(&mut self, handler: impl 'static + Fn(&LogoutProxyState) + Send + Sync) {
        self.pubsub.subscribe(handler);
    }

    pub async fn ignite(
        self,
        token: impl AuthenticateTokenExtract,
    ) -> MethodResult<LogoutProxyState> {
        let (_ticket, token) =
            authenticate_with_token(self.material.authenticate_with_token(), token, |event| {
                self.pubsub
                    .post(LogoutProxyState::AuthenticateWithToken(event))
            })
            .await?;

        self.pubsub
            .post(LogoutProxyState::ProxyCall(AuthProxyCallEvent::TryToCall(
                LogoutActionInfo.name().to_owned(),
            )));

        Ok(self
            .pubsub
            .post(LogoutProxyState::ProxyCall(AuthProxyCallEvent::Response(
                self.material
                    .proxy_call()
                    .call(token, ())
                    .await
                    .map_err(|err| {
                        self.pubsub.post(LogoutProxyState::ProxyCall(
                            AuthProxyCallEvent::ServiceError(err),
                        ))
                    })?,
            ))))
    }
}

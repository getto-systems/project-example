use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::auth::user::password::authenticate::action::AuthenticateWithPasswordActionInfo;

use crate::auth::ticket::authenticate::proxy::method::AuthProxyCallEvent;

use crate::common::proxy::infra::ProxyCall;

use crate::auth::{
    proxy::data::AuthProxyError, ticket::authenticate::proxy::data::ProxyResponseAuthenticated,
};

pub enum AuthenticateWithPasswordProxyState {
    ProxyCall(AuthProxyCallEvent<ProxyResponseAuthenticated>),
}

impl std::fmt::Display for AuthenticateWithPasswordProxyState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ProxyCall(event) => event.fmt(f),
        }
    }
}

pub trait AuthenticateWithPasswordProxyMaterial {
    type ProxyCall: ProxyCall<
        Request = String,
        Response = ProxyResponseAuthenticated,
        Error = AuthProxyError,
    >;

    fn proxy_call(&self) -> &Self::ProxyCall;
}

pub struct AuthenticateWithPasswordProxyAction<M: AuthenticateWithPasswordProxyMaterial> {
    pubsub: ActionStatePubSub<AuthenticateWithPasswordProxyState>,
    material: M,
}

impl<M: AuthenticateWithPasswordProxyMaterial> AuthenticateWithPasswordProxyAction<M> {
    pub fn with_material(material: M) -> Self {
        Self {
            pubsub: ActionStatePubSub::new(),
            material,
        }
    }

    pub fn subscribe(
        &mut self,
        handler: impl 'static + Fn(&AuthenticateWithPasswordProxyState) + Send + Sync,
    ) {
        self.pubsub.subscribe(handler);
    }

    pub async fn ignite(self, request: String) -> MethodResult<AuthenticateWithPasswordProxyState> {
        self.pubsub
            .post(AuthenticateWithPasswordProxyState::ProxyCall(
                AuthProxyCallEvent::TryToCall(AuthenticateWithPasswordActionInfo.name().to_owned()),
            ));

        Ok(self
            .pubsub
            .post(AuthenticateWithPasswordProxyState::ProxyCall(
                AuthProxyCallEvent::Response(
                    self.material
                        .proxy_call()
                        .call((), request)
                        .await
                        .map_err(|err| {
                            self.pubsub
                                .post(AuthenticateWithPasswordProxyState::ProxyCall(
                                    AuthProxyCallEvent::ServiceError(err),
                                ))
                        })?,
                ),
            )))
    }
}

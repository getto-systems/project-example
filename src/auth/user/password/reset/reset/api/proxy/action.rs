use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::auth::user::password::reset::reset::action::ResetPasswordActionInfo;

use crate::auth::ticket::authenticate::proxy::method::AuthProxyCallEvent;

use crate::common::proxy::infra::ProxyCall;

use crate::auth::{
    proxy::data::AuthProxyError, ticket::authenticate::proxy::data::ProxyResponseAuthenticated,
};

pub enum ResetPasswordProxyState {
    ProxyCall(AuthProxyCallEvent<ProxyResponseAuthenticated>),
}

impl std::fmt::Display for ResetPasswordProxyState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ProxyCall(event) => event.fmt(f),
        }
    }
}

pub trait ResetPasswordProxyMaterial {
    type ProxyCall: ProxyCall<
        Request = String,
        Response = ProxyResponseAuthenticated,
        Error = AuthProxyError,
    >;

    fn proxy_call(&self) -> &Self::ProxyCall;
}

pub struct ResetPasswordProxyAction<M: ResetPasswordProxyMaterial> {
    pubsub: ActionStatePubSub<ResetPasswordProxyState>,
    material: M,
}

impl<M: ResetPasswordProxyMaterial> ResetPasswordProxyAction<M> {
    pub fn with_material(material: M) -> Self {
        Self {
            pubsub: ActionStatePubSub::new(),
            material,
        }
    }

    pub fn subscribe(
        &mut self,
        handler: impl 'static + Fn(&ResetPasswordProxyState) + Send + Sync,
    ) {
        self.pubsub.subscribe(handler);
    }

    // TODO reset token を metadata として受け取り、metadata として送信するほうが対象性がいい気がする
    pub async fn ignite(self, request: String) -> MethodResult<ResetPasswordProxyState> {
        self.pubsub.post(ResetPasswordProxyState::ProxyCall(
            AuthProxyCallEvent::TryToCall(ResetPasswordActionInfo.name().to_owned()),
        ));

        Ok(self.pubsub.post(ResetPasswordProxyState::ProxyCall(
            AuthProxyCallEvent::Response(
                self.material
                    .proxy_call()
                    .call((), request)
                    .await
                    .map_err(|err| {
                        self.pubsub.post(ResetPasswordProxyState::ProxyCall(
                            AuthProxyCallEvent::ServiceError(err),
                        ))
                    })?,
            ),
        )))
    }
}

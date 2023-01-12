use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::auth::user::password::reset::request_token::action::RequestResetTokenActionInfo;

use crate::auth::ticket::authenticate::proxy::method::AuthProxyCallEvent;

use crate::common::proxy::infra::ProxyCall;

use crate::{auth::proxy::data::AuthProxyError, common::proxy::data::ProxyResponseBody};

pub enum RequestResetTokenProxyState {
    ProxyCall(AuthProxyCallEvent<ProxyResponseBody>),
}

impl std::fmt::Display for RequestResetTokenProxyState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ProxyCall(event) => event.fmt(f),
        }
    }
}

pub trait RequestResetTokenProxyMaterial {
    type ProxyCall: ProxyCall<
        Request = String,
        Response = ProxyResponseBody,
        Error = AuthProxyError,
    >;

    fn proxy_call(&self) -> &Self::ProxyCall;
}

pub struct RequestResetTokenProxyAction<M: RequestResetTokenProxyMaterial> {
    pubsub: ActionStatePubSub<RequestResetTokenProxyState>,
    material: M,
}

impl<M: RequestResetTokenProxyMaterial> RequestResetTokenProxyAction<M> {
    pub fn with_material(material: M) -> Self {
        Self {
            pubsub: ActionStatePubSub::new(),
            material,
        }
    }

    pub fn subscribe(
        &mut self,
        handler: impl 'static + Fn(&RequestResetTokenProxyState) + Send + Sync,
    ) {
        self.pubsub.subscribe(handler);
    }

    pub async fn ignite(self, request: String) -> MethodResult<RequestResetTokenProxyState> {
        self.pubsub.post(RequestResetTokenProxyState::ProxyCall(
            AuthProxyCallEvent::TryToCall(RequestResetTokenActionInfo.name().to_owned()),
        ));

        Ok(self.pubsub.post(RequestResetTokenProxyState::ProxyCall(
            AuthProxyCallEvent::Response(
                self.material
                    .proxy_call()
                    .call((), request)
                    .await
                    .map_err(|err| {
                        self.pubsub.post(RequestResetTokenProxyState::ProxyCall(
                            AuthProxyCallEvent::ServiceError(err),
                        ))
                    })?,
            ),
        )))
    }
}

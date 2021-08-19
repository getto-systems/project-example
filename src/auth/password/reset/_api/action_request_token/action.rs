use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::auth::password::reset::_api::request_token::infra::RequestResetTokenRequestDecoder;
use crate::auth::password::reset::_api::request_token::{
    event::RequestResetTokenEvent, infra::RequestResetTokenInfra, method::request_reset_token,
};

use crate::z_details::_api::message::data::MessageError;

pub enum RequestResetTokenState {
    RequestToken(RequestResetTokenEvent),
    MessageError(MessageError),
}

impl std::fmt::Display for RequestResetTokenState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MessageError(err) => {
                write!(f, "request reset token error; message error: {}", err)
            }
            Self::RequestToken(event) => write!(f, "{}", event),
        }
    }
}

pub trait RequestResetTokenMaterial {
    type RequestToken: RequestResetTokenInfra;

    fn request_token(&self) -> &Self::RequestToken;
}

pub struct RequestResetTokenAction<M: RequestResetTokenMaterial> {
    pubsub: ActionStatePubSub<RequestResetTokenState>,
    material: M,
}

impl<M: RequestResetTokenMaterial> RequestResetTokenAction<M> {
    pub fn with_material(material: M) -> Self {
        Self {
            pubsub: ActionStatePubSub::new(),
            material,
        }
    }

    pub fn subscribe(&mut self, handler: impl 'static + Fn(&RequestResetTokenState) + Send + Sync) {
        self.pubsub.subscribe(handler);
    }

    pub async fn ignite(
        self,
        request_decoder: impl RequestResetTokenRequestDecoder,
    ) -> MethodResult<RequestResetTokenState> {
        let pubsub = self.pubsub;
        let m = self.material;

        let fields = request_decoder
            .decode()
            .map_err(RequestResetTokenState::MessageError)?;

        request_reset_token(m.request_token(), fields, |event| {
            pubsub.post(RequestResetTokenState::RequestToken(event))
        })
        .await
    }
}

use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::auth::ticket::{
    authenticate::method::{
        authenticate_with_token, AuthenticateWithTokenEvent, AuthenticateWithTokenInfra,
    },
    encode::method::{encode_auth_token, EncodeAuthTokenEvent, EncodeAuthTokenInfra},
};

use crate::auth::ticket::kernel::data::AuthenticateTokenExtract;

pub enum AuthenticateWithTokenState {
    AuthenticateWithToken(AuthenticateWithTokenEvent),
    Encode(EncodeAuthTokenEvent),
}

impl std::fmt::Display for AuthenticateWithTokenState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AuthenticateWithToken(event) => event.fmt(f),
            Self::Encode(event) => event.fmt(f),
        }
    }
}

pub trait AuthenticateWithTokenMaterial {
    type AuthenticateWithToken: AuthenticateWithTokenInfra;
    type Encode: EncodeAuthTokenInfra;

    fn authenticate_with_token(&self) -> &Self::AuthenticateWithToken;
    fn encode(&self) -> &Self::Encode;
}

pub struct AuthenticateWithTokenAction<M: AuthenticateWithTokenMaterial> {
    pub info: AuthenticateWithTokenActionInfo,
    pubsub: ActionStatePubSub<AuthenticateWithTokenState>,
    material: M,
}

pub struct AuthenticateWithTokenActionInfo;

impl AuthenticateWithTokenActionInfo {
    pub const fn name(&self) -> &'static str {
        "auth.ticket.authenticate"
    }
}

impl<M: AuthenticateWithTokenMaterial> AuthenticateWithTokenAction<M> {
    pub fn with_material(material: M) -> Self {
        Self {
            info: AuthenticateWithTokenActionInfo,
            pubsub: ActionStatePubSub::new(),
            material,
        }
    }

    pub fn subscribe(
        &mut self,
        handler: impl 'static + Fn(&AuthenticateWithTokenState) + Send + Sync,
    ) {
        self.pubsub.subscribe(handler);
    }

    pub async fn ignite(
        self,
        token: impl AuthenticateTokenExtract,
    ) -> MethodResult<AuthenticateWithTokenState> {
        let pubsub = self.pubsub;

        let (ticket, _token) =
            authenticate_with_token(self.material.authenticate_with_token(), token, |event| {
                pubsub.post(AuthenticateWithTokenState::AuthenticateWithToken(event))
            })
            .await?;

        encode_auth_token(self.material.encode(), ticket, |event| {
            pubsub.post(AuthenticateWithTokenState::Encode(event))
        })
        .await
    }
}

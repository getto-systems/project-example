use crate::auth::ticket::authorize::action::ClarifyAuthorizeTokenActionInfo;

use crate::auth::ticket::authenticate::proxy::method::AuthProxyCallEvent;

use crate::{
    auth::ticket::authorize::infra::AuthorizeFieldsExtract, common::proxy::infra::ProxyCall,
};

use crate::auth::{
    proxy::data::AuthProxyError,
    ticket::{authorize::data::ValidateAuthorizeFieldsError, kernel::data::AuthPermissionRequired},
    user::kernel::data::AuthUserId,
};

pub enum AuthorizeEvent {
    Invalid(ValidateAuthorizeFieldsError),
    ProxyCall(AuthProxyCallEvent<AuthUserId>),
}

const ERROR: &'static str = "authorize error";

impl std::fmt::Display for AuthorizeEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid(err) => write!(f, "{}; {}", ERROR, err),
            Self::ProxyCall(event) => event.fmt(f),
        }
    }
}

pub trait AuthorizeInfra {
    type ProxyCall: ProxyCall<
        Request = AuthPermissionRequired,
        Response = AuthUserId,
        Error = AuthProxyError,
    >;

    fn proxy_call(&self) -> &Self::ProxyCall;
}

pub async fn authorize<S>(
    infra: &impl AuthorizeInfra,
    fields: impl AuthorizeFieldsExtract,
    post: impl Fn(AuthorizeEvent) -> S,
) -> Result<AuthUserId, S> {
    let fields = fields
        .convert()
        .map_err(|err| post(AuthorizeEvent::Invalid(err)))?;

    post(AuthorizeEvent::ProxyCall(AuthProxyCallEvent::TryToCall(
        format!(
            "{}({})",
            ClarifyAuthorizeTokenActionInfo.name(),
            &fields.required
        ),
    )));

    let user_id = infra
        .proxy_call()
        .call(fields.token, fields.required)
        .await
        .map_err(|err| {
            post(AuthorizeEvent::ProxyCall(AuthProxyCallEvent::ServiceError(
                err,
            )))
        })?;

    post(AuthorizeEvent::ProxyCall(AuthProxyCallEvent::Response(
        user_id.clone(),
    )));

    Ok(user_id)
}

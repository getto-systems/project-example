use crate::{auth::proxy::data::AuthProxyError, common::proxy::event::ProxyCallEvent};

pub type AuthProxyCallEvent<R> = ProxyCallEvent<R, AuthProxyError>;

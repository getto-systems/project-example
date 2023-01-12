use crate::{
    auth::{proxy::data::ProxyDomain, ticket::kernel::data::AuthToken},
    common::proxy::data::ProxyResponseBody,
};

pub type ProxyResponseAuthenticated = (ProxyResponseBody, Option<(AuthToken, ProxyDomain)>);

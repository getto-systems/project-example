use crate::common::proxy::data::CoreProxyError;

pub enum ProxyCallEvent<R, E> {
    TryToCall(String),
    Response(R),
    ServiceError(E),
}

pub type CoreProxyCallEvent<R> = ProxyCallEvent<R, CoreProxyError>;

const SUCCESS: &'static str = "proxy call success";
const ERROR: &'static str = "proxy call error";

impl<R, E: std::fmt::Display> std::fmt::Display for ProxyCallEvent<R, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TryToCall(target) => write!(f, "try to proxy call: {}", target),
            Self::Response(_) => write!(f, "{}", SUCCESS),
            Self::ServiceError(err) => write!(f, "{}; {}", ERROR, err),
        }
    }
}

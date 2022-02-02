use crate::avail::unexpected_error::remote::y_protobuf::service::notify_pb_server::NotifyPbServer;

use crate::avail::unexpected_error::notify::remote::x_tonic::route::ServiceNotify;

pub struct UnexpectedErrorServer;

impl UnexpectedErrorServer {
    pub fn notify(&self) -> NotifyPbServer<ServiceNotify> {
        NotifyPbServer::new(ServiceNotify)
    }
}

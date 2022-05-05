use crate::auth::user::login_id::change::y_protobuf::service::overwrite_login_id_pb_server::OverwriteLoginIdPbServer;

use crate::auth::user::login_id::change::x_tonic::route::ServiceOverwriteLoginId;

pub struct LoginIdServer;

impl LoginIdServer {
    pub fn overwrite(&self) -> OverwriteLoginIdPbServer<ServiceOverwriteLoginId> {
        OverwriteLoginIdPbServer::new(ServiceOverwriteLoginId)
    }
}

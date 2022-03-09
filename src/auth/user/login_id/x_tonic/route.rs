use crate::auth::user::login_id::change::y_protobuf::service::override_login_id_pb_server::OverrideLoginIdPbServer;

use crate::auth::user::login_id::change::x_tonic::route::ServiceOverrideLoginId;

pub struct LoginIdServer;

impl LoginIdServer {
    pub fn override_login_id(&self) -> OverrideLoginIdPbServer<ServiceOverrideLoginId> {
        OverrideLoginIdPbServer::new(ServiceOverrideLoginId)
    }
}

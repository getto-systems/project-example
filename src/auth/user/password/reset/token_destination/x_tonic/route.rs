use crate::auth::user::password::reset::token_destination::change::y_protobuf::service::change_reset_token_destination_pb_server::ChangeResetTokenDestinationPbServer;

use crate::auth::user::password::reset::token_destination::change::x_tonic::route::ServiceChangeDestination;

pub struct TokenDestinationServer;

impl TokenDestinationServer {
    pub fn change(&self) -> ChangeResetTokenDestinationPbServer<ServiceChangeDestination> {
        ChangeResetTokenDestinationPbServer::new(ServiceChangeDestination)
    }
}

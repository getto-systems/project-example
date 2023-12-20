use crate::auth::user::password::reset::token_destination::change::x_tonic::route::ServiceChangeDestination;

#[derive(Default)]
pub struct TokenDestinationServer {
    pub change: ServiceChangeDestination,
}

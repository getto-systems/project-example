use crate::auth::user::login_id::change::x_tonic::route::ServiceOverwriteLoginId;

#[derive(Default)]
pub struct LoginIdServer {
    pub overwrite: ServiceOverwriteLoginId,
}

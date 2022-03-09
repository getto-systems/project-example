use crate::auth::user::login_id::{
    change::data::OverrideLoginIdRepositoryError, kernel::data::LoginId,
};

pub trait OverrideLoginIdRequestDecoder {
    fn decode(self) -> OverrideLoginIdFieldsExtract;
}

pub struct OverrideLoginIdFieldsExtract {
    pub login_id: String,
    pub new_login_id: String,
}

#[async_trait::async_trait]
pub trait OverrideLoginIdRepository {
    async fn override_login_id<'a>(
        &self,
        login_id: &'a LoginId,
        new_login_id: LoginId,
    ) -> Result<(), OverrideLoginIdRepositoryError>;
}

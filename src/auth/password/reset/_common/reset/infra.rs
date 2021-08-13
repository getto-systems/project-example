#[derive(Clone)]
pub struct ResetPasswordFieldsExtract {
    pub reset_token: String,
    pub login_id: String,
    pub password: String,
}

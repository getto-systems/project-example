pub trait AuthenticatePasswordRequestDecoder {
    fn decode(self) -> AuthenticatePasswordFieldsExtract;
}

pub struct AuthenticatePasswordFieldsExtract {
    pub login_id: String,
    pub password: String,
}

pub trait ChangePasswordRequestDecoder {
    fn decode(self) -> ChangePasswordFieldsExtract;
}

pub struct ChangePasswordFieldsExtract {
    pub current_password: String,
    pub new_password: String,
}

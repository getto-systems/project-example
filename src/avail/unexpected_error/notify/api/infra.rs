pub struct NotifyUnexpectedErrorFields {
    // 例外的に validation しないで受け取る
    pub err: String,
}

pub trait NotifyUnexpectedErrorFieldsExtract {
    fn convert(self) -> NotifyUnexpectedErrorFields;
}

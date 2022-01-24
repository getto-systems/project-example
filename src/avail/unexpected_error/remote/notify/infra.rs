pub trait NotifyUnexpectedErrorRequestDecoder {
    fn decode(self) -> NotifyUnexpectedErrorFieldsExtract;
}

pub struct NotifyUnexpectedErrorFieldsExtract {
    pub err: String,
}

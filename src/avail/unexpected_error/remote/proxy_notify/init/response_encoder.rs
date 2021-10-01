use crate::example::remote::proxy::ExampleProxyResponseEncoder;

use crate::{
    avail::unexpected_error::remote::proxy_notify::data::NotifyUnexpectedErrorProxyMessage,
    z_lib::remote::message::data::MessageError,
};

pub struct ResponseEncoder;

impl<'a> ExampleProxyResponseEncoder<(), NotifyUnexpectedErrorProxyMessage> for ResponseEncoder {
    fn encode(&self, _response: ()) -> Result<NotifyUnexpectedErrorProxyMessage, MessageError> {
        Ok(NotifyUnexpectedErrorProxyMessage)
    }
}
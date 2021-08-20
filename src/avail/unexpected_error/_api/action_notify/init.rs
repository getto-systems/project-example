use actix_web::HttpRequest;

use crate::x_outside_feature::_api::feature::AppFeature;

use crate::avail::unexpected_error::_api::notify::init::{
    request_decoder::ProstNotifyUnexpectedErrorRequestDecoder, NotifyUnexpectedErrorStruct,
};

use super::action::{NotifyUnexpectedErrorAction, NotifyUnexpectedErrorMaterial};

use crate::avail::unexpected_error::_api::notify::infra::NotifyUnexpectedErrorRequestDecoder;

pub struct NotifyUnexpectedErrorFeature<'a> {
    notify: NotifyUnexpectedErrorStruct<'a>,
}

impl<'a> NotifyUnexpectedErrorFeature<'a> {
    pub fn action(
        feature: &'a AppFeature,
        request_id: &'a str,
        request: &'a HttpRequest,
    ) -> NotifyUnexpectedErrorAction<Self> {
        NotifyUnexpectedErrorAction::with_material(Self {
            notify: NotifyUnexpectedErrorStruct::new(&feature.auth, request_id, request),
        })
    }
    pub fn request_decoder(body: String) -> impl NotifyUnexpectedErrorRequestDecoder {
        ProstNotifyUnexpectedErrorRequestDecoder::new(body)
    }
}

impl<'a> NotifyUnexpectedErrorMaterial for NotifyUnexpectedErrorFeature<'a> {
    type Notify = NotifyUnexpectedErrorStruct<'a>;

    fn notify(&self) -> &Self::Notify {
        &self.notify
    }
}

use tonic::metadata::MetadataMap;

use crate::{
    common::api::request::x_tonic::metadata::metadata, x_outside_feature::data::RequestId,
};

pub const COOKIE_AUTHENTICATE_TOKEN: &'static str = "__Secure-GETTO-EXAMPLE-AUTHENTICATE-TOKEN";
pub const COOKIE_AUTHORIZE_TOKEN: &'static str = "__Secure-GETTO-EXAMPLE-AUTHORIZE-TOKEN";

pub const METADATA_AUTHENTICATE_TOKEN: &'static str = "getto-example-authenticate-token";
pub const METADATA_AUTHORIZE_TOKEN: &'static str = "getto-example-authorize-token";
pub const METADATA_REQUEST_ID: &'static str = "getto-example-request-id";

pub const COOKIE_CLOUDFRONT_SIGNATURE: &'static str = "CloudFront-Signature";
pub const COOKIE_CLOUDFRONT_KEY_PAIR_ID: &'static str = "CloudFront-Key-Pair-Id";
pub const COOKIE_CLOUDFRONT_POLICY: &'static str = "CloudFront-Policy";

impl RequestId {
    pub fn from_metadata(map: &MetadataMap) -> RequestId {
        match metadata(map, METADATA_REQUEST_ID) {
            Ok(Some(request_id)) => RequestId::restore(request_id.to_owned()),
            _ => RequestId::not_specified(),
        }
    }
}

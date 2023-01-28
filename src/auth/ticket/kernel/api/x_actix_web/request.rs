use actix_web::HttpRequest;

use crate::common::api::request::x_actix_web::metadata::cookie;

use crate::x_content::metadata::{COOKIE_AUTHENTICATE_TOKEN, COOKIE_AUTHORIZE_TOKEN};

use crate::auth::ticket::kernel::data::{
    AuthenticateToken, AuthenticateTokenExtract, AuthorizeToken, AuthorizeTokenExtract,
    ValidateAuthenticateTokenError, ValidateAuthorizeTokenError,
};

impl AuthenticateTokenExtract for &HttpRequest {
    fn convert(self) -> Result<AuthenticateToken, ValidateAuthenticateTokenError> {
        Ok(AuthenticateToken::restore(
            cookie(self, COOKIE_AUTHENTICATE_TOKEN)
                .ok_or(ValidateAuthenticateTokenError::NotFound)?,
        ))
    }
}

impl AuthorizeTokenExtract for &HttpRequest {
    fn convert(self) -> Result<AuthorizeToken, ValidateAuthorizeTokenError> {
        Ok(AuthorizeToken::restore(
            cookie(self, COOKIE_AUTHORIZE_TOKEN).ok_or(ValidateAuthorizeTokenError::NotFound)?,
        ))
    }
}

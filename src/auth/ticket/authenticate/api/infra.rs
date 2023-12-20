use crate::auth::ticket::{
    authenticate::data::CheckAuthenticateTokenSuccess,
    kernel::data::{
        AuthTicket, AuthenticateToken, DecodeAuthenticateTokenError, ValidateAuthenticateTokenError,
    },
};

pub trait CheckAuthenticateTokenInfra {
    type TokenDecoder: AuthenticateTokenDecoder;

    fn token_decoder(&self) -> &Self::TokenDecoder;
}

pub trait CheckAuthenticateTokenLogger: Send + Sync {
    fn try_to_check_authenticate_token(&self);
    fn invalid_request(
        &self,
        err: ValidateAuthenticateTokenError,
    ) -> ValidateAuthenticateTokenError;
    fn invalid_token(&self, err: DecodeAuthenticateTokenError) -> DecodeAuthenticateTokenError;
    fn succeed_to_check_authenticate_token(
        &self,
        auth: CheckAuthenticateTokenSuccess,
    ) -> CheckAuthenticateTokenSuccess;
}

pub trait AuthenticateTokenDecoder {
    fn decode(&self, token: AuthenticateToken) -> Result<AuthTicket, DecodeAuthenticateTokenError>;
}

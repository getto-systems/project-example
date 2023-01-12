use crate::auth::ticket::kernel::data::{
    AuthTicket, AuthenticateToken, DecodeAuthenticateTokenError,
};

pub trait AuthenticateTokenDecoder {
    fn decode(&self, token: AuthenticateToken)
        -> Result<AuthTicket, DecodeAuthenticateTokenError>;
}

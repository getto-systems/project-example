use crate::auth::auth_ticket::_auth::kernel::infra::AuthTicketInfra;

use crate::auth::auth_ticket::_auth::{
    encode::data::{AuthTokenEncodedData, EncodeAuthTokenError},
    kernel::data::{AuthTicket, ExpireDateTime, ExpireDuration},
};

pub trait EncodeAuthTicketInfra {
    type TicketInfra: AuthTicketInfra;
    type TicketEncoder: AuthTokenEncoder;
    type ApiEncoder: AuthTokenEncoder;
    type CloudfrontEncoder: AuthTokenEncoder;

    fn ticket_infra(&self) -> &Self::TicketInfra;
    fn ticket_encoder(&self) -> &Self::TicketEncoder;
    fn api_encoder(&self) -> &Self::ApiEncoder;
    fn cloudfront_encoder(&self) -> &Self::CloudfrontEncoder;
    fn config(&self) -> &EncodeAuthTicketConfig;
}

pub trait AuthTokenEncoder {
    fn encode(
        &self,
        ticket: AuthTicket,
        expires: ExpireDateTime,
    ) -> Result<Vec<AuthTokenEncodedData>, EncodeAuthTokenError>;
}

pub struct EncodeAuthTicketConfig {
    pub ticket_expires: ExpireDuration,
    pub api_expires: ExpireDuration,
    pub cloudfront_expires: ExpireDuration,
}

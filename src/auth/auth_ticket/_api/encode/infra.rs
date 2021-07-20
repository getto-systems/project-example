use crate::auth::auth_ticket::_api::kernel::infra::AuthTicketInfra;

use crate::{
    auth::{
        auth_ticket::_api::{
            encode::data::{AuthTokenEncodedData, EncodeAuthTokenError},
            kernel::data::{AuthTicket, ExpireDateTime, ExpireDuration},
        },
        auth_user::_common::kernel::data::GrantedAuthRoles,
    },
    z_details::_api::message::data::MessageError,
};

pub trait EncodeAuthTicketInfra {
    type TicketInfra: AuthTicketInfra;
    type TicketEncoder: AuthTokenEncoder;
    type ApiEncoder: AuthTokenEncoder;
    type CloudfrontEncoder: AuthTokenEncoder;
    type Messenger: EncodeMessenger;

    fn ticket_infra(&self) -> &Self::TicketInfra;
    fn ticket_encoder(&self) -> &Self::TicketEncoder;
    fn api_encoder(&self) -> &Self::ApiEncoder;
    fn cloudfront_encoder(&self) -> &Self::CloudfrontEncoder;
    fn messenger(&self) -> &Self::Messenger;
    fn config(&self) -> &EncodeAuthTicketConfig;
}

pub trait AuthTokenEncoder {
    fn encode(
        &self,
        ticket: AuthTicket,
        expires: ExpireDateTime,
    ) -> Result<Vec<AuthTokenEncodedData>, EncodeAuthTokenError>;
}

pub trait EncodeMessenger {
    fn encode(&self, granted_roles: GrantedAuthRoles) -> Result<String, MessageError>;
}

pub struct EncodeAuthTicketConfig {
    pub ticket_expires: ExpireDuration,
    pub api_expires: ExpireDuration,
    pub cloudfront_expires: ExpireDuration,
}

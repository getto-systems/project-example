mod token_encoder;

use crate::auth::_auth::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::auth_ticket::_auth::kernel::init::AuthTicketStruct;
use token_encoder::{
    ApiJwtAuthTokenEncoder, CookieCloudfrontTokenEncoder, TicketJwtAuthTokenEncoder,
};

use super::infra::{EncodeAuthTicketConfig, EncodeAuthTicketInfra};

pub struct EncodeAuthTicketStruct<'a> {
    ticket_infra: AuthTicketStruct<'a>,
    ticket_encoder: TicketJwtAuthTokenEncoder<'a>,
    api_encoder: ApiJwtAuthTokenEncoder<'a>,
    cloudfront_encoder: CookieCloudfrontTokenEncoder<'a>,
    config: EncodeAuthTicketConfig,
}

impl<'a> EncodeAuthTicketStruct<'a> {
    pub fn new(feature: &'a AuthOutsideFeature) -> Self {
        Self {
            ticket_infra: AuthTicketStruct::new(feature),
            ticket_encoder: TicketJwtAuthTokenEncoder::new(&feature.secret.ticket.encoding_key),
            api_encoder: ApiJwtAuthTokenEncoder::new(&feature.secret.api.encoding_key),
            cloudfront_encoder: CookieCloudfrontTokenEncoder::new(&feature.secret.cloudfront),
            config: EncodeAuthTicketConfig {
                ticket_expires: feature.config.ticket_expires,
                api_expires: feature.config.api_expires,
                cloudfront_expires: feature.config.cloudfront_expires,
            },
        }
    }
}

impl<'a> EncodeAuthTicketInfra for EncodeAuthTicketStruct<'a> {
    type TicketInfra = AuthTicketStruct<'a>;
    type TicketEncoder = TicketJwtAuthTokenEncoder<'a>;
    type ApiEncoder = ApiJwtAuthTokenEncoder<'a>;
    type CloudfrontEncoder = CookieCloudfrontTokenEncoder<'a>;

    fn extract(
        self,
    ) -> (
        Self::TicketInfra,
        Self::TicketEncoder,
        Self::ApiEncoder,
        Self::CloudfrontEncoder,
        EncodeAuthTicketConfig,
    ) {
        (
            self.ticket_infra,
            self.ticket_encoder,
            self.api_encoder,
            self.cloudfront_encoder,
            self.config,
        )
    }
}

#[cfg(test)]
pub mod test {
    pub use super::token_encoder::test::{StaticAuthTokenEncoder, StaticCloudfrontTokenEncoder};
    use crate::auth::auth_ticket::_auth::kernel::init::test::StaticAuthTicketStruct;

    use crate::auth::auth_ticket::_auth::encode::infra::{
        EncodeAuthTicketConfig, EncodeAuthTicketInfra,
    };

    pub struct StaticEncodeAuthTicketStruct<'a> {
        pub ticket_infra: StaticAuthTicketStruct<'a>,
        pub ticket_encoder: StaticAuthTokenEncoder,
        pub api_encoder: StaticAuthTokenEncoder,
        pub cloudfront_encoder: StaticCloudfrontTokenEncoder,
        pub config: EncodeAuthTicketConfig,
    }

    impl<'a> EncodeAuthTicketInfra for StaticEncodeAuthTicketStruct<'a> {
        type TicketInfra = StaticAuthTicketStruct<'a>;
        type TicketEncoder = StaticAuthTokenEncoder;
        type ApiEncoder = StaticAuthTokenEncoder;
        type CloudfrontEncoder = StaticCloudfrontTokenEncoder;

        fn extract(
            self,
        ) -> (
            Self::TicketInfra,
            Self::TicketEncoder,
            Self::ApiEncoder,
            Self::CloudfrontEncoder,
            EncodeAuthTicketConfig,
        ) {
            (
                self.ticket_infra,
                self.ticket_encoder,
                self.api_encoder,
                self.cloudfront_encoder,
                self.config,
            )
        }
    }
}

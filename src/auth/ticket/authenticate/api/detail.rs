use std::sync::Arc;

use actix_web::web::Data;

use crate::{
    auth::ticket::kernel::detail::token::authenticate::decoder::JwtAuthenticateTokenDecoder,
    common::api::{feature::AsInfra, logger::detail::StdoutJsonLogger},
    x_outside_feature::{auth::feature::AuthAppFeature, proxy::feature::ProxyAppFeature},
};

use crate::auth::ticket::authenticate::infra::{
    CheckAuthenticateTokenInfra, CheckAuthenticateTokenLogger,
};

use crate::auth::ticket::{
    authenticate::data::CheckAuthenticateTokenSuccess,
    kernel::data::{DecodeAuthenticateTokenError, ValidateAuthenticateTokenError},
};

pub struct LiveCheckAuthenticateTokenInfra {
    token_decoder: JwtAuthenticateTokenDecoder,
}

impl AsInfra<LiveCheckAuthenticateTokenInfra> for Data<ProxyAppFeature> {
    fn as_infra(&self) -> LiveCheckAuthenticateTokenInfra {
        LiveCheckAuthenticateTokenInfra {
            token_decoder: self.as_infra(),
        }
    }
}

impl AsInfra<LiveCheckAuthenticateTokenInfra> for Arc<AuthAppFeature> {
    fn as_infra(&self) -> LiveCheckAuthenticateTokenInfra {
        LiveCheckAuthenticateTokenInfra {
            token_decoder: self.as_infra(),
        }
    }
}

impl CheckAuthenticateTokenInfra for LiveCheckAuthenticateTokenInfra {
    type TokenDecoder = JwtAuthenticateTokenDecoder;

    fn token_decoder(&self) -> &Self::TokenDecoder {
        &self.token_decoder
    }
}

impl CheckAuthenticateTokenLogger for StdoutJsonLogger {
    fn try_to_check_authenticate_token(&self) {
        self.info(format!("try to check authenticate-token"));
    }
    fn invalid_request(
        &self,
        err: ValidateAuthenticateTokenError,
    ) -> ValidateAuthenticateTokenError {
        self.incident(format!("failed to validate authenticate-token; {}", err));
        err
    }
    fn invalid_token(&self, err: DecodeAuthenticateTokenError) -> DecodeAuthenticateTokenError {
        self.incident(format!("failed to decode authenticate-token; {}", err));
        err
    }
    fn succeed_to_check_authenticate_token(
        &self,
        auth: CheckAuthenticateTokenSuccess,
    ) -> CheckAuthenticateTokenSuccess {
        self.info(format!("succeed to check authenticate-token; {}", auth));
        auth
    }
}

#[cfg(test)]
pub mod test {
    use crate::{
        auth::ticket::kernel::detail::token::authenticate::decoder::test::MockAuthenticateTokenDecoder,
        common::api::feature::AsInfra,
    };

    use crate::auth::ticket::authenticate::infra::CheckAuthenticateTokenInfra;

    pub struct MockCheckAuthenticateTokenInfra {
        pub token_decoder: MockAuthenticateTokenDecoder,
    }

    impl AsInfra<MockCheckAuthenticateTokenInfra> for MockAuthenticateTokenDecoder {
        fn as_infra(&self) -> MockCheckAuthenticateTokenInfra {
            MockCheckAuthenticateTokenInfra {
                token_decoder: self.clone(),
            }
        }
    }

    impl<'a> CheckAuthenticateTokenInfra for MockCheckAuthenticateTokenInfra {
        type TokenDecoder = MockAuthenticateTokenDecoder;

        fn token_decoder(&self) -> &Self::TokenDecoder {
            &self.token_decoder
        }
    }
}

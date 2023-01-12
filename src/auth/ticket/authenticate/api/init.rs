use crate::auth::x_outside_feature::feature::AuthOutsideDecodingKey;
use crate::x_outside_feature::auth::feature::AuthAppFeature;

use crate::auth::ticket::{
    encode::init::ActiveEncodeAuthTokenInfra,
    kernel::init::token::authenticate::decoder::JwtAuthenticateTokenDecoder,
};

use crate::auth::ticket::authenticate::action::{
    AuthenticateWithTokenAction, AuthenticateWithTokenMaterial,
};

use crate::auth::ticket::authenticate::method::AuthenticateWithTokenInfra;

pub struct ActiveAuthenticateWithTokenMaterial<'a> {
    authenticate_with_token: ActiveAuthenticateWithTokenInfra<'a>,
    encode: ActiveEncodeAuthTokenInfra<'a>,
}

impl<'a> ActiveAuthenticateWithTokenMaterial<'a> {
    pub fn action(feature: &'a AuthAppFeature) -> AuthenticateWithTokenAction<Self> {
        AuthenticateWithTokenAction::with_material(Self {
            authenticate_with_token: ActiveAuthenticateWithTokenInfra::new(&feature.decoding_key),
            encode: ActiveEncodeAuthTokenInfra::new(feature),
        })
    }
}

impl<'a> AuthenticateWithTokenMaterial for ActiveAuthenticateWithTokenMaterial<'a> {
    type AuthenticateWithToken = ActiveAuthenticateWithTokenInfra<'a>;
    type Encode = ActiveEncodeAuthTokenInfra<'a>;

    fn authenticate_with_token(&self) -> &Self::AuthenticateWithToken {
        &self.authenticate_with_token
    }
    fn encode(&self) -> &Self::Encode {
        &self.encode
    }
}

pub struct ActiveAuthenticateWithTokenInfra<'a> {
    token_decoder: JwtAuthenticateTokenDecoder<'a>,
}

impl<'a> ActiveAuthenticateWithTokenInfra<'a> {
    pub fn new(decoding_key: &'a AuthOutsideDecodingKey) -> Self {
        Self {
            token_decoder: JwtAuthenticateTokenDecoder::new(decoding_key),
        }
    }
}

impl<'a> AuthenticateWithTokenInfra for ActiveAuthenticateWithTokenInfra<'a> {
    type TokenDecoder = JwtAuthenticateTokenDecoder<'a>;

    fn token_decoder(&self) -> &Self::TokenDecoder {
        &self.token_decoder
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::ticket::{
        encode::init::test::StaticEncodeAuthTokenInfra,
        kernel::init::token::authenticate::decoder::test::StaticAuthenticateTokenDecoder,
    };

    use crate::auth::ticket::authenticate::action::AuthenticateWithTokenMaterial;

    use crate::auth::ticket::authenticate::method::AuthenticateWithTokenInfra;

    pub struct StaticAuthenticateWithTokenMaterial<'a> {
        pub authenticate_with_token: StaticAuthenticateWithTokenInfra,
        pub encode: StaticEncodeAuthTokenInfra<'a>,
    }

    impl<'a> AuthenticateWithTokenMaterial for StaticAuthenticateWithTokenMaterial<'a> {
        type AuthenticateWithToken = StaticAuthenticateWithTokenInfra;
        type Encode = StaticEncodeAuthTokenInfra<'a>;

        fn authenticate_with_token(&self) -> &Self::AuthenticateWithToken {
            &self.authenticate_with_token
        }
        fn encode(&self) -> &Self::Encode {
            &self.encode
        }
    }

    pub struct StaticAuthenticateWithTokenInfra {
        pub token_decoder: StaticAuthenticateTokenDecoder,
    }

    impl<'a> AuthenticateWithTokenInfra for StaticAuthenticateWithTokenInfra {
        type TokenDecoder = StaticAuthenticateTokenDecoder;

        fn token_decoder(&self) -> &Self::TokenDecoder {
            &self.token_decoder
        }
    }
}

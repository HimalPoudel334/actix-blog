use std::sync::Arc;

use chrono::{Duration, Utc};
use jsonwebtoken::{
    decode, encode, errors::Error, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};

use crate::config::ApplicationConfiguration;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

pub fn create_jwt_token(user_id: i32, app_config: Arc<ApplicationConfiguration>) -> String {
    //set the claims
    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::minutes(60)).timestamp() as usize;

    let claims: TokenClaims = TokenClaims {
        sub: user_id.to_string(),
        exp,
        iat,
    };

    encode_token(claims, app_config.jwt_secret.to_owned())
}

fn encode_token(claims: TokenClaims, jwt_secret: String) -> String {
    let header: Header = Header {
        alg: Algorithm::HS512,
        ..Default::default()
    };

    //encode the claims to create a token
    encode(
        // &Header::default(),
        &header,
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )
    .unwrap()
}

pub fn decode_token(jwt_token: String, jwt_secret: String) -> Result<TokenClaims, Error> {
    let validation: Validation = Validation::new(Algorithm::HS512);

    let token_claims = decode::<TokenClaims>(
        &jwt_token,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        // &Validation::default(),
        &validation,
    )?;

    Ok(token_claims.claims)
}

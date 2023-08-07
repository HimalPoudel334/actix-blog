use std::sync::Arc;

use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
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

    //encode the claims to create a token
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(app_config.jwt_secret.as_ref()),
    )
    .unwrap()
}

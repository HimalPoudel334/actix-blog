use crate::utils::token_helper::TokenClaims;
use crate::{config::ApplicationConfiguration, responses::error::ErrorResponse};
use actix_web::{
    error::ErrorUnauthorized, http, web, Error as ActixWebError, FromRequest, HttpMessage,
};
use jsonwebtoken::{decode, DecodingKey, Validation};

use std::future::{ready, Ready};

pub struct JwtMiddleware {
    pub user_id: String,
}

impl FromRequest for JwtMiddleware {
    type Error = ActixWebError;

    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let app_config = req
            .app_data::<web::Data<ApplicationConfiguration>>()
            .unwrap();

        //get the cookie and extract the token
        let token = req
            .cookie("token")
            .map(|c| c.value().to_string())
            .or_else(|| {
                req.headers()
                    .get(http::header::AUTHORIZATION)
                    .map(|h| h.to_str().unwrap().split_at(7).1.to_string())
            });

        //if token is not there, then the request is unauthenticated
        if token.is_none() {
            let json_error = ErrorResponse {
                status: "fail".to_string(),
                message: "You are not logged in, please provide token".to_string(),
            };
            return ready(Err(ErrorUnauthorized(json_error)));
        }

        //if token is there, decode the token to TokenClaims struct using the secret key of .env file
        let claims = match decode::<TokenClaims>(
            &token.unwrap(),
            &DecodingKey::from_secret(app_config.jwt_secret.as_ref()),
            &Validation::default(),
        ) {
            Ok(token_claims) => token_claims.claims,
            Err(_) => {
                let json_error = ErrorResponse {
                    status: "fail".to_string(),
                    message: "Invalid token".to_string(),
                };
                return ready(Err(ErrorUnauthorized(json_error)));
            }
        };

        //if decoding is successful, parse the sub claim (this claim stores the user id)
        let user_id: &str = claims.sub.as_str();

        //insert the user id to request header
        req.extensions_mut().insert::<String>(user_id.to_owned());

        ready(Ok(JwtMiddleware {
            user_id: user_id.to_owned(),
        }))
    }
}

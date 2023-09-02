use crate::config::ApplicationConfiguration;
use crate::utils::token_helper::decode_token;
use actix_web::http::header::LOCATION;
use actix_web::{http, web, Error as ActixWebError, FromRequest, HttpMessage};
use actix_web::{HttpResponse, ResponseError};

use derive_more::Display;
use std::future::{ready, Ready};

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmt = "BadRequest: {_0}")]
    BadRequest(String),

    #[display(fmt = "Unauthorized: {_0}")]
    Unauthorized(String),
}

// impl ResponseError trait allows to convert our errors into http responses with appropriate data
impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            ServiceError::Unauthorized(ref path) => HttpResponse::SeeOther()
                .append_header((
                    LOCATION,
                    format!("/auth/login?return_url={}", path).as_str(),
                ))
                .finish(),
        }
    }
}

pub struct JwtMiddleware {
    pub user_id: i32,
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

        //get the path of the request
        let path: &str = req.path();
        println!("requested path is {}", path);
        req.extensions_mut().insert::<String>(path.to_owned());

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
            return ready(Err(ServiceError::Unauthorized(path.into()).into()));
        }

        //if token is there, decode the token to TokenClaims struct using the secret key of .env file
        let claims = match decode_token(token.unwrap(), app_config.jwt_secret.to_owned()) {
            Ok(decoded) => decoded,
            Err(e) => {
                return ready(Err(ServiceError::BadRequest(
                    format!("Invalid token: {}", e).into(),
                )
                .into()));
            }
        };

        //if decoding is successful, parse the sub claim (this claim stores the user id)
        let user_id: i32 = claims.sub.as_str().parse().expect("Couldn't parse user_id");
        //insert the user id to request header
        req.extensions_mut().insert::<i32>(user_id);

        ready(Ok(JwtMiddleware { user_id }))
    }
}

/*use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    http::{
        self,
        header::{HeaderValue, LOCATION},
    },
    web, Error, HttpResponse,
};
use futures::future::{ok, ready};
use futures::{future::Ready, Future};
use jsonwebtoken::{decode, DecodingKey, Validation};
use std::pin::Pin;

use crate::{config::ApplicationConfiguration, utils::token_helper::TokenClaims};

pub struct Auth;

impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware { service }))
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(
        &self,
        ctx: &mut core::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        //extract the app config from request
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
            return Box::pin(async move {
                Ok(req.into_response(
                    HttpResponse::SeeOther()
                        .append_header((LOCATION, HeaderValue::from_static("/auth/login")))
                        .finish(),
                ))
            });
        }

        //if token is there, decode the token to TokenClaims struct using the secret key of .env file
        let claims = match decode::<TokenClaims>(
            &token.unwrap(),
            &DecodingKey::from_secret(app_config.jwt_secret.as_ref()),
            &Validation::default(),
        ) {
            Ok(token_claims) => token_claims.claims,
            Err(_) => {
                return Box::pin(async move {
                    Ok(req.error_response(HttpResponse::Unauthorized().finish().into_body()))
                });
            }
        };

        //if decoding is successful, parse the sub claim (this claim stores the user id)
        let user_id: &str = claims.sub.as_str();

        //insert the user id to request header
        // req.extensions_mut().insert::<String>(path);

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}*/

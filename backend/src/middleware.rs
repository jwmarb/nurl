use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    http::header,
    Error, HttpMessage,
};
use futures::future::{ready, LocalBoxFuture, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::{
    rc::Rc,
    task::{Context, Poll},
};

use crate::constants::NURL_SECRET;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    pub username: String,
    pub exp: usize,
}

pub struct ExtractUsernameJWT;

impl<S, B> Transform<S, ServiceRequest> for ExtractUsernameJWT
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = ExtractUsernameMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ExtractUsernameMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct ExtractUsernameMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for ExtractUsernameMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();

        Box::pin(async move {
            // Extract token from Authorization header
            if let Some(header_val) = req
                .headers()
                .get(header::AUTHORIZATION)
                .and_then(|h| h.to_str().ok())
                .filter(|s| s.starts_with("Bearer "))
            {
                let token = &header_val[7..]; // Skip "Bearer " prefix
                let secret = NURL_SECRET.as_bytes();

                // Decode the token
                if let Ok(data) = decode::<Claims>(
                    token,
                    &DecodingKey::from_secret(secret),
                    &Validation::default(),
                ) {
                    // Extract username and insert it into request extensions
                    let username = data.claims.username;
                    req.extensions_mut().insert(username);
                }
            }

            // Continue with the request processing
            service.call(req).await
        })
    }
}

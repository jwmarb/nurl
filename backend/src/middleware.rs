use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    http::header,
    Error, HttpMessage,
};
use futures::future::{ready, LocalBoxFuture, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation};
use std::{
    rc::Rc,
    task::{Context, Poll},
};

use crate::{constants::NURL_SECRET, structs::Claims};

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
            process_auth_header(&req);
            service.call(req).await
        })
    }
}

/// Extract token from Authorization header and process it
pub fn process_auth_header(req: &ServiceRequest) {
    if let Some(token) = extract_token_from_header(req) {
        if let Some(username) = validate_and_extract_username(&token) {
            req.extensions_mut().insert(username);
        }
    }
}

/// Extract the JWT token from the Authorization header
pub fn extract_token_from_header(req: &ServiceRequest) -> Option<String> {
    req.headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .filter(|s| s.starts_with("Bearer "))
        .map(|s| s[7..].to_string()) // Skip "Bearer " prefix
}

/// Validate the JWT token and extract username
pub fn validate_and_extract_username(token: &str) -> Option<String> {
    let secret = NURL_SECRET.as_bytes();

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret),
        &Validation::default(),
    )
    .map(|data| data.claims.username)
    .ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::Claims;
    use actix_web::http::header;
    use actix_web::test::TestRequest;
    use jsonwebtoken::{encode, EncodingKey, Header};
    use std::time::{SystemTime, UNIX_EPOCH};

    fn create_test_token(username: &str) -> String {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let claims = Claims {
            username: username.to_string(),
            exp: (now + 3600) as usize, // Valid for 1 hour
        };
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(NURL_SECRET.as_bytes()),
        )
        .unwrap()
    }

    #[test]
    fn test_extract_token_from_header() {
        // Test with valid header
        let token = "valid_token_here";
        let req = TestRequest::default()
            .insert_header((header::AUTHORIZATION, format!("Bearer {}", token)))
            .to_srv_request();

        let extracted = extract_token_from_header(&req);
        assert_eq!(extracted, Some(token.to_string()));

        // Test with no authorization header
        let req = TestRequest::default().to_srv_request();
        let extracted = extract_token_from_header(&req);
        assert_eq!(extracted, None);

        // Test with incorrect prefix
        let req = TestRequest::default()
            .insert_header((header::AUTHORIZATION, format!("Basic {}", token)))
            .to_srv_request();
        let extracted = extract_token_from_header(&req);
        assert_eq!(extracted, None);
    }

    #[test]
    fn test_validate_and_extract_username() {
        // Test with valid token
        let username = "test_user";
        let token = create_test_token(username);
        let extracted = validate_and_extract_username(&token);
        assert_eq!(extracted, Some(username.to_string()));

        // Test with invalid token
        let extracted = validate_and_extract_username("invalid_token");
        assert_eq!(extracted, None);
    }

    #[test]
    fn test_process_auth_header() {
        // Test with valid token
        let username = "test_user";
        let token = create_test_token(username);
        let req = TestRequest::default()
            .insert_header((header::AUTHORIZATION, format!("Bearer {}", token)))
            .to_srv_request();

        process_auth_header(&req);

        // Check if username was inserted into request extensions
        let binding = req.extensions();
        let ext_username = binding.get::<String>();
        assert!(ext_username.is_some());
        assert_eq!(ext_username.unwrap(), username);

        // Test with invalid token
        let req = TestRequest::default()
            .insert_header((header::AUTHORIZATION, "Bearer invalid_token"))
            .to_srv_request();

        process_auth_header(&req);

        // No username should be inserted for invalid token
        let binding = req.extensions();
        let ext_username = binding.get::<String>();
        assert!(ext_username.is_none());
    }
}

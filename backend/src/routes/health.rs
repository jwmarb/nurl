use actix_web::{get, HttpResponse, Responder};

use crate::structs::APIResponse;

/// Health check endpoint
/// 
/// This endpoint is used to verify that the service is running and responding to requests.
/// It returns a simple "alive" message in the response data.
/// 
/// # Returns
/// HTTP response with status 200 and a JSON body containing "alive" in the data field
#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json(APIResponse::data("alive"))
}

/// Test module for the health endpoint
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http::StatusCode, test, App};
    use serde_json::Value;

    /// Tests that the health endpoint returns the expected response
    #[actix_rt::test]
    async fn test_health_endpoint() {
        // Create test app with the handler
        let app = test::init_service(App::new().service(health)).await;

        // Send test request
        let req = test::TestRequest::get().uri("/health").to_request();

        // Call service and get response
        let resp = test::call_service(&app, req).await;

        // Assert status code is OK
        assert_eq!(resp.status(), StatusCode::OK);

        // Check response body
        let body = test::read_body(resp).await;
        let json: Value = serde_json::from_slice(&body).expect("Failed to parse JSON response");

        // Validate the response structure and content
        assert_eq!(json["data"], "alive");
    }

    /// Tests that the health endpoint rejects non-GET requests
    #[actix_rt::test]
    async fn test_health_endpoint_wrong_method() {
        // Create test app with the handler
        let app = test::init_service(App::new().service(health)).await;

        // Try with POST instead of GET
        let req = test::TestRequest::post().uri("/health").to_request();
        let resp = test::call_service(&app, req).await;

        // Assert method not allowed
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    /// Tests that the health endpoint rejects requests to incorrect paths
    #[actix_rt::test]
    async fn test_health_endpoint_wrong_path() {
        // Create test app with the handler
        let app = test::init_service(App::new().service(health)).await;

        // Try with an incorrect path
        let req = test::TestRequest::get().uri("/health/wrong").to_request();
        let resp = test::call_service(&app, req).await;

        // Assert not found
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }
}

use axum::{routing::get, Router};

use crate::handler::health_checker_handler;

pub fn create_router() -> Router {
    Router::new().route("/api/ping", get(health_checker_handler))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Method, Request, StatusCode};

    use serde_json::Value;

    use tower::ServiceExt;

    #[tokio::test]
    async fn test_ping_endpoint() {
        let app = create_router();

        // Create the request
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/ping")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        // Convert response body to bytes
        let body_bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
        let body_json: Value = serde_json::from_str(&body_str).unwrap();

        // Create the expected JSON response
        let expected_json: Value = serde_json::json!({
            "status": "success",
            "message": "pong"
        });

        assert_eq!(body_json, expected_json);
    }
}

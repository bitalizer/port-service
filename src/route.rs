use crate::database;
use crate::database::AppState;
use axum::error_handling::HandleErrorLayer;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{routing::get, BoxError, Router};
use std::borrow::Cow;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use crate::handler::{create_port_handler, health_checker_handler, port_list_handler};

type SharedState = Arc<RwLock<AppState>>;

pub fn create_router() -> Router {
    let shared_state = SharedState::default();
    create_router_with_state(shared_state)
}

pub fn create_router_with_state(shared_state: SharedState) -> Router {
    Router::new()
        .route("/api/ping", get(health_checker_handler))
        .route(
            "/api/ports",
            get(port_list_handler).post(create_port_handler),
        )
        .layer(
            ServiceBuilder::new()
                // Handle errors from middleware
                .layer(HandleErrorLayer::new(handle_error))
                .load_shed()
                .concurrency_limit(1024)
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http()),
        )
        .with_state(shared_state)
}

async fn handle_error(error: BoxError) -> impl IntoResponse {
    if error.is::<tower::timeout::error::Elapsed>() {
        return (StatusCode::REQUEST_TIMEOUT, Cow::from("request timed out"));
    }

    if error.is::<tower::load_shed::error::Overloaded>() {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Cow::from("service is overloaded, try again later"),
        );
    }

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Cow::from(format!("Unhandled internal error: {}", error)),
    )
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs;
    use super::*;
    use axum::body::Body;
    use axum::http::{Method, Request, StatusCode};

    use serde_json::Value;

    use tower::ServiceExt;
    use crate::model::Port;
    use crate::response::{GenericResponse, PortListResponse};

    #[tokio::test]
    async fn test_health_checker_handler() {
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

    #[tokio::test]
    async fn port_list_handler() {
        let shared_state = SharedState::default();

        let data = fs::read_to_string("./ports.json")
            .expect("Unable to read file");

        let ports: HashMap<String, Port> = serde_json::from_str(&data).expect("JSON does not have correct format.");
        let ports_len = ports.len();

        for (key, value) in ports
        {
            shared_state.write().unwrap().db.insert(key, value);
        }

        let app = create_router_with_state(shared_state);

        // Create the request
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/ports")
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
        let port_list_response: PortListResponse = serde_json::from_str(&body_str).unwrap();

        assert_eq!(ports_len, port_list_response.results);
    }

    #[tokio::test]
    async fn create_port_handler() {
        let shared_state = SharedState::default();

        let data = fs::read_to_string("./ports.json")
            .expect("Unable to read file");

        let app = create_router_with_state(shared_state);

        // Create the request
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/ports")
                    .method(Method::POST)
                    .body(Body::from(data))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        // Convert response body to bytes
        let body_bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
        let port_list_response: GenericResponse = serde_json::from_str(&body_str).unwrap();

        assert_eq!(shared_state.read().unwrap().db.len(), 0);
    }
}

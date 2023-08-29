use crate::response::GenericResponse;
use axum::{response::IntoResponse, Json};

pub async fn health_checker_handler() -> impl IntoResponse {
    let json_response = GenericResponse {
        status: "success".to_string(),
        message: "pong".to_string(),
    };

    Json(json_response)
}

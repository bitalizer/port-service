use std::collections::HashMap;

use crate::database::SharedState;
use crate::model::Port;
use crate::response::{GenericResponse, PortListResponse};
use axum::extract::State;
use axum::http::StatusCode;
use axum::{response::IntoResponse, Json};
use serde_json::Value;

pub async fn health_checker_handler() -> impl IntoResponse {
    let json_response = GenericResponse {
        status: "success".to_string(),
        message: "pong".to_string(),
    };

    Json(json_response)
}

pub async fn port_list_handler(State(state): State<SharedState>) -> impl IntoResponse {
    let db = &state.read().unwrap().db;

    let ports = db.values().cloned().collect();

    let json_response = PortListResponse {
        status: "success".to_string(),
        results: db.keys().count(),
        ports,
    };

    Json(json_response)
}

pub async fn create_port_handler(
    State(state): State<SharedState>,
    Json(data): Json<HashMap<String, Port>>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    for (key, value) in data {
        state.write().unwrap().db.insert(key, value);
    }

    let json_response = GenericResponse {
        status: "success".to_string(),
        message: "Ports uploaded successfully".to_string(),
    };

    Ok((StatusCode::CREATED, Json(json_response)))
}

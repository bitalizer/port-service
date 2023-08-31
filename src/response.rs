use crate::model::Port;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PortListResponse {
    pub status: String,
    pub results: usize,
    pub ports: Vec<Port>,
}

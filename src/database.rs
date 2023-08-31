use crate::model::Port;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Default)]
pub struct AppState {
    pub db: HashMap<String, Port>,
}

pub type SharedState = Arc<RwLock<AppState>>;